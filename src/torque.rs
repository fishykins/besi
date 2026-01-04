use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of torque, with a base unit of newton meters.
    Torque
}

define_units! { Torque =>
    newton_meter: ("N·m", 1.0),
    kilonewton_meter: ("kN·m", 1000.0),
    meganewton_meter: ("MN·m", 1.0e6),
    teranewton_meter: ("TN·m", 1.0e9),
    giganewton_meter: ("GN·m", 1.0e12),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let torque = Torque::new::<kilonewton_meter>(1.0);
        assert_eq!(torque.as_ref(), &1000.0);
    }
}