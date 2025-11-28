use crate::{define_measurement, define_units};
use std::f64::consts::PI;

define_measurement! {
    /// A measurement of an angle, with a base unit of radians.
    Angle
}

impl Angle {
    pub const NORTH: Self = Self(0.0);
    pub const EAST: Self = Self(0.5);
    pub const SOUTH: Self = Self(1.0);
    pub const WEST: Self = Self(1.5);

}

define_units! { Angle =>
    radian: ("rad", 1.0),
    degree: ("°", PI / 180.0),
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn it_works() {
        let angle = Angle::new::<degree>(180.0);
        assert_eq!(angle.as_ref(), &PI);
    }

    #[test]
    fn const_zero_is_zero() {
        let zero = Angle::ZERO;
        assert_eq!(zero.0, 0.0);
    }

    #[test]
    fn get_works() {
        let angle = Angle(PI);
        assert_eq!(angle.get::<radian>(), PI);
        assert_eq!(angle.get::<degree>(), 180.0);
    }
}
