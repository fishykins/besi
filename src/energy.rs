use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of energy, with a base unit of joules.
    Energy
}

define_measurement! {
    /// A measurement of energy, with a base unit of joules.
    MolarEnergy
}


define_units! { Energy =>
    joule: ("J", 1.0),
    kilojoule: ("kJ", 1000.0),
    megajoule: ("MJ", 1.0e6),
}

define_units! { MolarEnergy =>
    joule_per_mole: ("J/mol", 1.0),
    kilojoule_per_mole: ("kJ/mol", 1000.0),
    megajoulee_per_mole: ("MJ/mol", 1.0e6),
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
