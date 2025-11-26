use crate::{length::{Length, LengthUnit}};
use bevy_math::{DVec3, prelude::*};
use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[cfg_attr(feature = "reflect", derive(bevy_reflect::Reflect))]
#[derive(Clone, Copy, Default, Debug, PartialEq, PartialOrd)]
pub struct DPos3 {
    pub x: Length,
    pub y: Length,
    pub z: Length,
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
        DVec3::new(
            self.x.get::<T>() as f64,
            self.y.get::<T>() as f64,
            self.z.get::<T>() as f64,
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
