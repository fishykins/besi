use std::ops::Mul;

use crate::{define_measurement, define_units, mass::Mass, prelude::Acceleration};

define_measurement! {
    /// A measurement of force, with a base unit of newtons.
    Force
}

define_units! { Force =>
    newton: ("N", 1.0),
    kilonewton: ("kN", 1000.0),
}

impl Mul<Mass> for Acceleration {
    type Output = Force;
    fn mul(self, rhs: Mass) -> Force {
        Force(self.as_ref() * rhs.as_ref())
    }
}

impl Mul<Acceleration> for Mass {
    type Output = Force;
    fn mul(self, rhs: Acceleration) -> Force {
        Force(self.as_ref() * rhs.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let force = Force::new::<kilonewton>(1.0);
        assert_eq!(force.as_ref(), &1000.0);
    }
}
