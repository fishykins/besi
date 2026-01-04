use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of velocity, with a base unit of meters per second.
    Velocity
}

define_measurement! {
    /// A measurement of angular velocity, with a base unit of radians per second.
    AngularVelocity
}

define_units! { Velocity =>
    meters_per_second: ("m/s", 1.0),
    kilometers_per_hour: ("km/h", 1.0 / 3.6),
    kilometers_per_second: ("km/s", 1000.0),
}

define_units! { AngularVelocity =>
    radian_per_second: ("rad/s", 1.0),
    degree_per_second: ("°/s", 1.745_329_251_994_329_5_E-2),
    revolution_per_second: ("rev/s", 6.283_185_307_179_586_E0),
    revolution_per_minute: ("rev/min", 1.047_197_551_196_597_7_E-1),
    revolution_per_hour: ("rev/h", 1.745_329_251_994_329_6_E-3),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let velocity = Velocity::new::<kilometers_per_hour>(36.0);
        assert_eq!(velocity.as_ref(), &10.0);
    }

    #[test]
    fn const_zero_is_zero() {
        let zero = Velocity::ZERO;
        assert_eq!(zero.0, 0.0);
    }

    #[test]
    fn get_works() {
        let velocity = Velocity(10.0);
        assert_eq!(velocity.get::<meters_per_second>(), 10.0);
        assert_eq!(velocity.get::<kilometers_per_hour>(), 36.0);
    }
}
