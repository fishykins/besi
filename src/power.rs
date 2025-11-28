use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of power, with a base unit of watts.
    Power
}

define_units! { Power =>
    watt: ("W", 1.0),
    kilowatt: ("kW", 1000.0),
    megawatt: ("MW", 1.0e6),
    terawatt: ("TW", 1.0e9),
    gigawatt: ("GW", 1.0e12),
    exawatt: ("EW", 1.0e15),
    solar_luminosity: ("L☉", 3.846e26),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let power = Power::new::<kilowatt>(1.0);
        assert_eq!(power.as_ref(), &1000.0);
    }
}
