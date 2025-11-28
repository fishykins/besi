use crate::{define_measurement, define_units};

define_measurement! {
    /// A measurement of time, with a base unit of seconds.
    Time
}

define_units! { Time =>
    second: ("s", 1.0),
    millisecond: ("ms", 0.001),
    minute: ("min", 60.0),
    hour: ("h", 3600.0),
    day: ("d", 86400.0),
    day_sidereal: ("d'", 86164.09),
    week: ("w", 604800.0),
    month: ("mo", 2592000.0),
    year: ("y", 31536000.0),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let time = Time::new::<minute>(1.0);
        assert_eq!(time.as_ref(), &60.0);
    }

    #[test]
    fn const_zero_is_zero() {
        let zero = Time::ZERO;
        assert_eq!(zero.0, 0.0);
    }

    #[test]
    fn get_works() {
        let time = Time(120.0);
        assert_eq!(time.get::<second>(), 120.0);
        assert_eq!(time.get::<minute>(), 2.0);
        assert_eq!(time.get::<hour>(), 120.0 / 3600.0);
    }

    #[test]
    fn display_works() {
        assert_eq!(Time(1500.0).to_string(), "25.00 min");
        assert_eq!(Time(59.0).to_string(), "59.00 s");
        assert_eq!(Time(1.23).to_string(), "1.23 s");
        assert_eq!(Time(0.5).to_string(), "500.00 ms");
        assert_eq!(Time(0.0001).to_string(), "0.10 ms");
        assert_eq!(Time(-3700.0).to_string(), "-1.03 h");
    }
}
