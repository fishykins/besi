use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of force, with a base unit of newtons.
    Force
}

define_units! { Force =>
    newton: ("N", 1.0),
    kilonewton: ("kN", 1000.0),
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
