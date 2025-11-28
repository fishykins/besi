use crate::{
    angle::*,
    length::{Length, LengthUnit},
};
use bevy_math::{DVec3, prelude::*};
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct DPos3 {
    pub x: Length,
    pub y: Length,
    pub z: Length,
}

#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LatLon {
    pub lat: Angle,
    pub lon: Angle,
}

impl DPos3 {
    /// A constant for the zero value (origin).
    pub const ZERO: Self = Self {
        x: Length::ZERO,
        y: Length::ZERO,
        z: Length::ZERO,
    };

    pub fn new<T: LengthUnit>(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: Length::new::<T>(x),
            y: Length::new::<T>(y),
            z: Length::new::<T>(z),
        }
    }

    pub fn from_lengths(x: Length, y: Length, z: Length) -> Self {
        Self { x, y, z }
    }

    pub fn from_vec3<T: LengthUnit>(vec3: Vec3) -> Self {
        Self {
            x: Length::new::<T>(vec3.x as f64),
            y: Length::new::<T>(vec3.y as f64),
            z: Length::new::<T>(vec3.z as f64),
        }
    }

    pub fn from_dvec3<T: LengthUnit>(vec3: DVec3) -> Self {
        Self {
            x: Length::new::<T>(vec3.x),
            y: Length::new::<T>(vec3.y),
            z: Length::new::<T>(vec3.z),
        }
    }

    pub fn to_vec3<T: LengthUnit>(&self) -> Vec3 {
        Vec3::new(
            self.x.get::<T>() as f32,
            self.y.get::<T>() as f32,
            self.z.get::<T>() as f32,
        )
    }

    pub fn to_dvec3<T: LengthUnit>(&self) -> DVec3 {
        DVec3::new(self.x.get::<T>(), self.y.get::<T>(), self.z.get::<T>())
    }

    pub fn from_tuple<T: LengthUnit>(tuple: (f64, f64, f64)) -> Self {
        Self {
            x: Length::new::<T>(tuple.0),
            y: Length::new::<T>(tuple.1),
            z: Length::new::<T>(tuple.2),
        }
    }

    pub fn to_tuple<T: LengthUnit>(&self) -> (f64, f64, f64) {
        (self.x.get::<T>(), self.y.get::<T>(), self.z.get::<T>())
    }

    /// Calculates the squared magnitude of the position vector.
    /// This is faster than `length()` as it avoids a square root.
    /// The result is a raw f64 representing the squared base units (meters squared).
    // Note: A more advanced implementation might return a new `Area` measurement type.
    pub fn length_squared(&self) -> f64 {
        self.x.0.powi(2) + self.y.0.powi(2) + self.z.0.powi(2)
    }

    /// Calculates the magnitude (or length) of the position vector.
    pub fn length(&self) -> Length {
        Length(self.length_squared().sqrt())
    }

    /// Returns a unitless `DVec3` representing the direction of this position vector.
    /// Returns `DVec3::ZERO` if the length is zero.
    pub fn normalize(&self) -> DVec3 {
        let length = self.length().0;
        if length == 0.0 {
            return DVec3::ZERO;
        }
        DVec3::new(self.x.0 / length, self.y.0 / length, self.z.0 / length)
    }

    /// Calculates the dot product of two position vectors.
    /// The result is a raw f64 representing the squared base units (meters squared).
    pub fn dot(&self, rhs: Self) -> f64 {
        self.x.0 * rhs.x.0 + self.y.0 * rhs.y.0 + self.z.0 * rhs.z.0
    }

    /// Calculates the cross product of two position vectors.
    /// The result is a new `DPos3` where the components are in squared units,
    /// which isn't physically standard. This is a pragmatic choice for geometric calculations.
    pub fn cross(&self, rhs: Self) -> DPos3 {
        DPos3::from_lengths(
            Length(self.y.0 * rhs.z.0 - self.z.0 * rhs.y.0),
            Length(self.z.0 * rhs.x.0 - self.x.0 * rhs.z.0),
            Length(self.x.0 * rhs.y.0 - self.y.0 * rhs.x.0),
        )
    }
}

impl fmt::Display for DPos3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for DPos3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for DPos3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f64> for DPos3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for DPos3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl From<Vec<Length>> for DPos3
{
    fn from(value: Vec<Length>) -> Self {
        Self {
            x: value[0],
            y: value[1],
            z: value[2],
        }
    }
}

impl From<(Length, Length, Length)> for DPos3
{
    fn from(value: (Length, Length, Length)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl From<DPos3> for Vec<Length>
{
    fn from(value: DPos3) -> Self {
        vec![value.x, value.y, value.z]
    }
}

impl From<DPos3> for (Length, Length, Length)
{
    fn from(value: DPos3) -> Self {
        (value.x, value.y, value.z)
    }
}

impl From<DPos3> for [Length; 3]
{
    fn from(value: DPos3) -> Self {
        [value.x, value.y, value.z]
    }
}

impl LatLon {
    pub fn new(lat: Angle, lon: Angle) -> Self {
        Self { lat, lon }
    }

    pub fn get_radians(&self) -> (f64, f64) {
        (self.lat.get::<radian>(), self.lon.get::<radian>())
    }

    pub fn get_degrees(&self) -> (f64, f64) {
        (self.lat.get::<degree>(), self.lon.get::<degree>())
    }

    pub const ZERO: Self = Self {
        lat: Angle::ZERO,
        lon: Angle::ZERO,
    };
}
