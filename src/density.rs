use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of density, with a base unit of kilograms per cubic meter.
    Density
}

define_units! { Density =>
    kilogram_per_cubic_meter: ("kg/m³", 1.0),
    kilogram_per_cubic_centimeter: ("kg/cm³", 1.0_E3)
}
