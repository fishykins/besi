# `besi` - Ergonomic Units of Measurement for Rust

[!Crates.io](https://crates.io/crates/besi)
[!Docs.rs](https://docs.rs/besi)

`besi` is a unit-of-measurement library for Rust. It is designed with a focus on game development and physics simulations, especially for integration with the Bevy Engine.
It provides strongly-typed wrappers around primitive types (like `f64`) to ensure that you don't accidentally mix incompatible units, like adding `Length` to `Time`.

"Wait a second", I hear you say- Why not just use `UOM`? `Besi` is uglier, dumber, and featurelesser than `UOM` (one of the greatest rust crates on the market) so why waste
my time with this pile of garbage? Well, if you happen to be a person who wants strongly typed measurements that aren't hidden behind generics and can be intergrated 
with `Bevy` with features like `Reflect` built in, this might be your best bet.

## Core Features

*   **Type Safety:** Prevents bugs by enforcing unit correctness at compile time. `Length + Time` won't compile!
*   **Ergonomic API:** This crate is a drop in replacement for UOM, which is the far superior crate (it's honestly amazing). 
*   **Automatic "Best Unit" Formatting:** The `Display` trait implementation intelligently selects the most readable unit for printing (e.g., `1500.0` meters prints as `"1.50 km"`).
*   **`DPos3` for 3D Vectors:** A built-in 3D position vector type that maintains unit correctness and provides standard vector math operations (`normalize`, `dot`, `length`, etc.).
*   **Extensible:** Easily define your own measurement types and units using simple macros.
*   **Bevy Integration:** Supports `bevy_reflect` behind a feature flag for seamless integration into Bevy projects.
*   **Serialization:** Full `serde` support for all measurement types.

## Quick Start

1.  Add `besi` to your `Cargo.toml`:

    ```toml
    [dependencies]
    besi = "0.1.0" # Replace with the latest version
    ```

2.  Use the `prelude` to easily import the most common types and start working with units:

    ```rust
    use besi::prelude::*;

    fn main() {
        // Create a Length of 1.5 kilometers.
        let distance = Length::new::<kilometer>(1.5);

        // Create a Time of 15 minutes.
        let time = Time::new::<minute>(15.0);

        // Calculations are unit-aware.
        // Dividing Length by Time gives Velocity.
        let velocity = distance / time.get::<hour>(); // km / h

        // The `Display` trait automatically picks a nice unit.
        println!("Distance: {}", distance); // "1.50 km"
        println!("Time: {}", time);         // "15.00 min"

        // You can get the value in any compatible unit.
        println!("Velocity: {:.2} m/s", velocity.get::<meters_per_second>());

        // Type safety prevents mistakes! This will not compile:
        // let error = distance + time;
    }
    ```

## Available Measurements

`besi` comes with a wide range of pre-defined measurement types:

*   `Angle` (radian, degree)
*   `AngularVelocity` (radian/s, degree/s, ...)
*   `Energy` (joule, kilojoule, ...)
*   `Force` (newton, kilonewton)
*   `Length` (meter, kilometer, astronomical_unit, ...)
*   `Mass` (kilogram, gram, solar_mass, ...)
*   `Power` (watt, kilowatt, solar_luminosity, ...)
*   `Pressure` (pascal, bar, atmosphere)
*   `Temperature` (kelvin, celsius, fahrenheit)
*   `Time` (second, minute, hour, year, ...)
*   `Velocity` (m/s, km/h, ...)
*   `Volume` (cubic_meter, liter, ...)
*   `VolumeRate` (m³/s, L/s)
*   And more!

## Position Vectors (`DPos3`)

For working with 3D space, `besi` provides `DPos3`, a vector type where each component is a `Length`. This is perfect for representing positions in a world while maintaining unit correctness.
It comes with standard vector math operations and can be converted into the bevy native Vec3 and DVec3 with relative ease. 

```rust
use besi::prelude::*;
use besi::position::DPos3;

// Create a position 5000 meters along the X axis.
let pos1 = DPos3::new::<meter>(5000.0, 0.0, 0.0);

// Create another position 2 kilometers along the Y axis.
let pos2 = DPos3::new::<kilometer>(0.0, 2.0, 0.0);

// Vector addition works as expected.
let combined_pos = pos1 + pos2;
println!("Combined: {}", combined_pos); // "(5.00 km, 2.00 km, 0.00 m)"

// Calculate the magnitude (distance from origin).
let distance_from_origin = combined_pos.length();
println!("Distance from origin: {}", distance_from_origin); // "5.39 km"

// Normalize to get a direction vector (returns a unitless bevy_math::DVec3).
let direction = combined_pos.normalize();
```

## Defining Custom Measurements

You can easily add your own measurement types and units using the `define_measurement!` and `define_units!` macros.

Here's how you could define a measurement for `Acceleration`:

```rust
// in a new file, e.g., `src/acceleration.rs`
use besi::{define_measurement, define_units};

// 1. Define the measurement type. The base unit is meters per second squared.
// Why this isn't already in the crate is slightly dumb, but hey. You got a free example.
define_measurement! {
    /// A measurement of acceleration.
    Acceleration
}

// 2. Define the associated units and their conversion factor to the base unit.
define_units! { Acceleration =>
    meters_per_second_squared: ("m/s²", 1.0),
    kilometers_per_hour_squared: ("km/h²", 1.0 / (3.6 * 3.6)),
    standard_gravity: ("g", 9.80665),
}
```


## Feature Flags

*   `position`: Enables the `DPos3` and `LatLon` types. This is enabled by default.
*   `reflect`: Implements `bevy_reflect::Reflect` for all measurement types, and is also enabled by default.


## License

This project is licensed under either of

*   Apache License, Version 2.0, (LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
*   MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
