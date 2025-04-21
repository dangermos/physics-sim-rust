use std::{fmt::Display, ops::{Add, Div, Mul}};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}


/// Dot Product of 2 Vectors returns a Scalar,
/// The magnitude of the resultant vector
pub trait DotProduct<Rhs = Self> {
    type Output;
    fn dot(self, rhs: Rhs) -> Self::Output;
}

pub trait Magnitude{
    type Output;
    fn magnitude(self) -> Self::Output;
}

pub trait Normalize {
    type Output;
    fn normalize(self) -> Self::Output;
}

pub trait Angle {
    type Output;
    fn get_angle(self) -> Self::Output;
}

/// Vector Addition
impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        let new_x: f32 = self.x + rhs.x;
        let new_y: f32 = self.y + rhs.y;

        Self {x: new_x,
              y: new_y}
    }
}

/// Scalar Product
impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/// Scalar Division
impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

/// Dot Product
impl DotProduct for Vec2 {
    type Output = f32;
    fn dot(self, rhs: Self) -> Self::Output {
        (self.x * rhs.x) + (self.y * rhs.y)
    }
}

/// Vector Magnitude
impl Magnitude for Vec2 {
    type Output = f32;
    fn magnitude(self) -> Self::Output {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/// Vector Normalization
impl Normalize for Vec2 {
    type Output = Vec2;
    fn normalize(self) -> Self::Output {
        Vec2 {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
        }
    }
}

impl Angle for Vec2 {
    type Output = f32;

    fn get_angle(self) -> Self::Output {
        ((self.y / self.x).atan()).to_degrees()
    }
}







impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
    
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}




#[cfg(test)]
mod tests {
    use super::*;




    #[test]
    fn test_vector_math() {

    }
}







