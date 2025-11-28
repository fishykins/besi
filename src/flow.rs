use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of flow, with a base unit of cubic meters per second.
    VolumeRate
}

define_units! { VolumeRate =>
    cubic_meters_per_second: ("m³/s", 1.0),
    liter_per_second: ("L/s", 0.001),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let flow = VolumeRate::new::<liter_per_second>(1000.0);
        assert_eq!(flow.as_ref(), &1.0);
    }
}
