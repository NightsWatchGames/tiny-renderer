use std::ops::{Add, Mul};

use crate::math::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl Color {
    pub const BLACK: Self = Self::new(0., 0., 0.);
    pub const RED: Self = Self::new(1., 0., 0.);
    pub const GREEN: Self = Self::new(0., 1., 0.);
    pub const BLUE: Self = Self::new(0., 0., 1.);
    pub const WHITE: Self = Self::new(1., 1., 1.);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn from_vec3(v: Vec3) -> Self {
        assert!(v.x >= 0.0 && v.x <= 1.0);
        assert!(v.y >= 0.0 && v.y <= 1.0);
        assert!(v.z >= 0.0 && v.z <= 1.0);
        Self::new(v.x, v.y, v.z)
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.r, self.g, self.b)
    }
}
impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}
impl Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            (self.r + rhs.r).min(1.0),
            (self.g + rhs.g).min(1.0),
            (self.b + rhs.b).min(1.0),
        )
    }
}
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}
impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, rhs: Color) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}
impl From<[f32; 3]> for Color {
    fn from(v: [f32; 3]) -> Self {
        Color::new(v[0], v[1], v[2])
    }
}
