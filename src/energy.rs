use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of energy, with a base unit of joules.
    Energy
}

define_units! { Energy =>
    joule: ("J", 1.0),
    kilojoule: ("kJ", 1000.0),
    megajoule: ("MJ", 1.0e6),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let energy = Energy::new::<kilojoule>(1.0);
        assert_eq!(energy.as_ref(), &1000.0);
    }
}
