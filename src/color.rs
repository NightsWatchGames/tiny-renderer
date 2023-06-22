use std::ops::{Add, Mul};

use crate::math::{Vec3, Vec4};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Color {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_vec3(mut v: Vec3) -> Self {
        // TODO 颜色归一化
        let max = v.x.max(v.y).max(v.z);
        if max > 1.0 {
            v.x /= max;
            v.y /= max;
            v.z /= max;
        }
        // assert!(v.x >= 0.0 && v.x <= 1.0);
        // assert!(v.y >= 0.0 && v.y <= 1.0);
        // assert!(v.z >= 0.0 && v.z <= 1.0);
        Self::new(
            (v.x * 255.0) as u8,
            (v.y * 255.0) as u8,
            (v.z * 255.0) as u8,
        )
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3::new(
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
        )
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
            self.r.saturating_add(rhs.r),
            self.g.saturating_add(rhs.g),
            self.b.saturating_add(rhs.b),
        )
    }
}
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(
            (self.r as f32 * rhs) as u8,
            (self.g as f32 * rhs) as u8,
            (self.b as f32 * rhs) as u8,
        )
    }
}
impl From<[u8; 3]> for Color {
    fn from(v: [u8; 3]) -> Self {
        Color::new(v[0], v[1], v[2])
    }
}
