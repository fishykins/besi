use std::fmt;

/// A measurement of temperature, with a base unit of Kelvin.
#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct Temperature(pub f64);

impl Temperature {
    /// A constant for the zero value.
    pub const ZERO: Self = Self(0.0);

    /// Creates a new temperature value from a given unit.
    pub fn new<T: TemperatureUnit>(value: f64) -> Self {
        Self(T::to_kelvin(value))
    }

    /// Gets the temperature value in terms of a specific unit.
    pub fn get<T: TemperatureUnit>(&self) -> f64 {
        T::from_kelvin(self.0)
    }
}

/// A trait for units of temperature.
pub trait TemperatureUnit {
    /// Converts a value in this unit to Kelvin.
    fn to_kelvin(value: f64) -> f64;
    /// Converts a value from Kelvin to this unit.
    fn from_kelvin(kelvin: f64) -> f64;
    /// The symbol for this unit.
    const SYMBOL: &'static str;
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let temp_c = self.get::<celsius>();
        if temp_c.abs() > 100.0 {
            write!(f, "{:.2} {}", self.get::<kelvin>(), kelvin::SYMBOL)
        } else if temp_c.abs() < 1.0 && temp_c.abs() > 1.0e-9 {
            write!(f, "{:.2} {}", self.get::<kelvin>(), kelvin::SYMBOL)
        } else {
            write!(f, "{:.2} {}", temp_c, celsius::SYMBOL)
        }
    }
}

// Implement standard math operators
impl std::ops::Add for Temperature {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Temperature {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::Mul<f64> for Temperature {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl std::ops::Div<f64> for Temperature {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl AsRef<f64> for Temperature {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}
impl AsMut<f64> for Temperature {
    fn as_mut(&mut self) -> &mut f64 {
        &mut self.0
    }
}

/// Kelvin (K)
#[derive(Default, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct kelvin;
impl TemperatureUnit for kelvin {
    fn to_kelvin(value: f64) -> f64 {
        value
    }
    fn from_kelvin(k: f64) -> f64 {
        k
    }
    const SYMBOL: &'static str = "K";
}

/// Celsius (°C)
#[derive(Default, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct celsius;
impl TemperatureUnit for celsius {
    fn to_kelvin(value: f64) -> f64 {
        value + 273.15
    }
    fn from_kelvin(k: f64) -> f64 {
        k - 273.15
    }
    const SYMBOL: &'static str = "°C";
}

/// Fahrenheit (°F)
#[derive(Default, Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct fahrenheit;
impl TemperatureUnit for fahrenheit {
    fn to_kelvin(value: f64) -> f64 {
        (value - 32.0) * 5.0 / 9.0 + 273.15
    }
    fn from_kelvin(k: f64) -> f64 {
        (k - 273.15) * 9.0 / 5.0 + 32.0
    }
    const SYMBOL: &'static str = "°F";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions() {
        let temp = Temperature::new::<celsius>(0.0);
        assert_eq!(temp.get::<kelvin>(), 273.15);
        assert_eq!(temp.get::<celsius>(), 0.0);
        assert!((temp.get::<fahrenheit>() - 32.0).abs() < 1e-9);

        let temp = Temperature::new::<fahrenheit>(32.0);
        assert_eq!(temp.get::<celsius>(), 0.0);

        let temp = Temperature::new::<kelvin>(0.0);
        assert_eq!(temp.get::<celsius>(), -273.15);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Temperature::new::<celsius>(0.0)), "0.00 °C");
        assert_eq!(format!("{}", Temperature::new::<celsius>(30.0)), "30.00 °C");
        assert_eq!(
            format!("{}", Temperature::new::<celsius>(100.0)),
            "100.00 °C"
        );
        assert_eq!(
            format!("{}", Temperature::new::<celsius>(101.0)),
            "374.15 K"
        );
    }
}
