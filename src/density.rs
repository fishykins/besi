use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of density, with a base unit of kilograms per cubic meter.
    MassDensity
}

define_units! { MassDensity =>
    kilogram_per_cubic_meter: ("kg/m³", 1.0),
    kilogram_per_cubic_centimeter: ("kg/cm³", 1.0_E3),
    gram_per_cubic_centimeter: ("g/cm³", 1.0_E6)
}
