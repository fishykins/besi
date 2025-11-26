#[macro_export]
macro_rules! define_units {
    ($measurement:ident => $($unit:ident: ($symbol:expr, $factor:expr)),* $(,)?) => {
        $crate::paste::paste! {
            pub trait [<$measurement Unit>]: $crate::BusiUnit {}
        }
        $(
            #[allow(non_camel_case_types)]
            #[derive(Default, Debug, Clone, Copy)]
            pub struct $unit;

            $crate::paste::paste! {
                impl $crate::BusiUnit for $unit { const SCALE_FACTOR: f64 = $factor; }
                impl [<$measurement Unit>] for $unit {}
            }

            inventory::submit! {
                $crate::UnitDisplayInfo {
                    measurement_type_name: stringify!($measurement),
                    symbol: $symbol,
                    scale_factor: $factor,
                }
            }
        )*
    };
}

/// Defines a new measurement type with all associated boilerplate.
///
/// This macro creates the struct and implements `new`, `get`, `ZERO`,
/// standard math operators, and a `Display` trait that automatically
/// selects the best unit.
#[macro_export]
macro_rules! define_measurement {
    (
        $(#[$outer:meta])* // The measurement name (e.g., `Length`).
        $name:ident
    ) => {
        $crate::paste::paste! {

        $(#[$outer])*
        #[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
        #[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
        pub struct $name(pub f64);

        impl $name {
            /// A constant for the zero value.
            pub const ZERO: Self = Self(0.0);

            /// Creates a new value from a given unit.
            pub fn new<T: [<$name Unit>]>(value: f64) -> Self {
                Self(value * T::SCALE_FACTOR)
            }

            /// Gets the value in terms of a specific unit.
            pub fn get<T: [<$name Unit>]>(&self) -> f64 {
                self.0 / T::SCALE_FACTOR
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let value = self.0;

                let best_unit = inventory::iter::<$crate::UnitDisplayInfo>
                    .into_iter()
                    .filter(|unit| unit.measurement_type_name == stringify!($name))
                    .filter(|unit| value.abs() >= unit.scale_factor)
                    .max_by(|a, b| a.scale_factor.partial_cmp(&b.scale_factor).unwrap())
                    .unwrap_or_else(|| {
                        inventory::iter::<$crate::UnitDisplayInfo>
                            .into_iter()
                            .filter(|unit| unit.measurement_type_name == stringify!($name))
                            .min_by(|a, b| a.scale_factor.partial_cmp(&b.scale_factor).unwrap())
                            .unwrap_or_else(|| panic!("No busi units defined for {}", stringify!($name)))
                    });

                write!(f, "{:.2} {}", value / best_unit.scale_factor, best_unit.symbol)
            }
        }

        impl std::ops::Add for $name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
        }

        impl std::ops::Sub for $name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0) }
        }

        impl std::ops::Mul<f64> for $name {
            type Output = Self;
            fn mul(self, rhs: f64) -> Self::Output { Self(self.0 * rhs) }
        }

        impl std::ops::Div<f64> for $name {
            type Output = Self;
            fn div(self, rhs: f64) -> Self::Output { Self(self.0 / rhs) }
        }

        impl std::ops::Div<$name> for $name {
            type Output = f64;
            fn div(self, rhs: Self) -> Self::Output { self.0 / rhs.0 }
        }

        impl AsRef<f64> for $name { fn as_ref(&self) -> &f64 { &self.0 } }
        impl AsMut<f64> for $name { fn as_mut(&mut self) -> &mut f64 { &mut self.0 } }

        }
    };
}