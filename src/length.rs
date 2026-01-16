use std::ops::Mul;

use crate::{define_measurement, define_units, time::Time, velocity::Velocity};

define_measurement! {
    /// A measurement of distance, with a base unit of meters.
    Length
}

define_units! { Length =>
    astronomical_unit: ("AU", 1.496e11),
    solar_radius: ("R☉", 6.957e8),
    earth_radius: ("R⊕", 6.371e6),
    kilometer: ("km", 1000.0),
    meter: ("m", 1.0),
    centimeter: ("cm", 0.01),
}

impl Mul<Time> for Velocity {
    type Output = Length;
    fn mul(self, rhs: Time) -> Self::Output {
        Length(self.0 * rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::length::*;

    #[test]
    fn it_works() {
        let length = Length::new::<kilometer>(1.0);
        assert_eq!(length.as_ref(), &1000.0);
    }

    #[test]
    fn const_zero_is_zero() {
        let zero = Length::ZERO;
        assert_eq!(zero.0, 0.0);
    }

    #[test]
    fn get_works() {
        let length = Length(1234.5);
        assert_eq!(length.get::<kilometer>(), 1.2345);
        assert_eq!(length.get::<meter>(), 1234.5);
        assert_eq!(length.get::<centimeter>(), 123450.0);
    }

    #[test]
    fn display_works() {
        assert_eq!(Length(1500.0).to_string(), "1.50 km");
        assert_eq!(Length(999.0).to_string(), "999.00 m");
        assert_eq!(Length(1.23).to_string(), "1.23 m");
        assert_eq!(Length(0.5).to_string(), "50.00 cm");
        assert_eq!(Length(0.001).to_string(), "0.10 cm");
        assert_eq!(Length(-2500.0).to_string(), "-2.50 km");
        assert_eq!(Length::new::<earth_radius>(1.0).to_string(), "1.00 R⊕");
        assert_eq!(Length::new::<solar_radius>(1.0).to_string(), "1.00 R☉");
        assert_eq!(Length::new::<astronomical_unit>(1.0).to_string(), "1.00 AU");
    }
}
