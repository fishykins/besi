use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of mass, with a base unit of kilograms.
    Mass
}

define_units! { Mass =>
    kilogram: ("kg", 1.0),
    gram: ("g", 0.001),
    tonne: ("t", 1000.0),
    earth_mass: ("M⊕", 5.972e24),
    solar_mass: ("M☉", 1.989e30),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mass = Mass::new::<tonne>(1.0);
        assert_eq!(mass.as_ref(), &1000.0);
    }
}
