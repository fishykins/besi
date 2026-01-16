use std::ops::Div;

use crate::{define_measurement, define_units, prelude::Velocity, time::Time};

define_measurement! {
    /// A measurement of acceleration, with a base unit of meters per second squared.
    Acceleration
}

define_units! { Acceleration =>
    meters_per_second_squared: ("m/s²", 1.0),
    standard_gravity: ("g", 9.80665),
}

impl Div<Time> for Velocity {
    type Output = Acceleration;
    fn div(self, rhs: Time) -> Self::Output {
        Acceleration(self.0 / rhs.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let acc = Acceleration::new::<standard_gravity>(1.0);
        assert_eq!(acc.as_ref(), &9.80665);
    }
}