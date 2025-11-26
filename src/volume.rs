use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of volume, with a base unit of cubic meters.
    Volume
}

define_units! { Volume =>
    cubic_meter: ("m³", 1.0),
    liter: ("L", 0.001),
    milliliter: ("mL", 1.0e-6),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let volume = Volume::new::<liter>(1000.0);
        assert_eq!(volume.as_ref(), &1.0);
    }
}
