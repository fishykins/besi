use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of pressure, with a base unit of pascals.
    Pressure
}

define_units! { Pressure =>
    pascal: ("Pa", 1.0),
    kilopascal: ("kPa", 1000.0),
    bar: ("bar", 100_000.0),
    atmosphere: ("atm", 1.013_25_E5),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pressure = Pressure::new::<bar>(1.0);
        assert_eq!(pressure.as_ref(), &100_000.0);
    }
}
