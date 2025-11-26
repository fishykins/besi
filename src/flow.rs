use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of flow, with a base unit of cubic meters per second.
    Flow
}

define_units! { Flow =>
    cubic_meters_per_second: ("m³/s", 1.0),
    liters_per_second: ("L/s", 0.001),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let flow = Flow::new::<liters_per_second>(1000.0);
        assert_eq!(flow.as_ref(), &1.0);
    }
}
