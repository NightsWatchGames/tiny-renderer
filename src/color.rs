use std::ops::{Add, Mul};

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
