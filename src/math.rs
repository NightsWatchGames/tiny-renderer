use std::ops::{Add, Mul, Sub};

//// 二维向量
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    // 点乘
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    // 长度
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    // 归一化
    pub fn normalize(self) -> Self {
        let normalized = self.mul(self.length().recip());
        if !normalized.is_finite() {
            panic!(
                "Vec2::normalize: normalized is not finite: {:?}",
                normalized
            );
        }
        normalized
    }
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
}
impl Add<Vec2> for Vec2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub<Vec2> for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

//// 三维向量
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    // 点乘
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    // 叉乘
    pub fn cross(self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    // 长度
    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }
    // 归一化
    pub fn normalize(self) -> Self {
        let normalized = self.mul(self.length().recip());
        if !normalized.is_finite() {
            panic!(
                "Vec3::normalize: normalized is not finite: {:?}",
                normalized
            );
        }
        normalized
    }
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}
impl Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

//// 2x2按列存储矩阵
pub struct Mat2 {
    pub x_axis: Vec2,
    pub y_axis: Vec2,
}
impl Mat2 {
    // 转置
    pub fn transpose(self) -> Self {
        Self {
            x_axis: Vec2 {
                x: self.x_axis.x,
                y: self.y_axis.x,
            },
            y_axis: Vec2 {
                x: self.x_axis.y,
                y: self.y_axis.y,
            },
        }
    }
}
impl Add<Mat2> for Mat2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
        }
    }
}
impl Sub<Mat2> for Mat2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
        }
    }
}
impl Mul<f32> for Mat2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x_axis: self.x_axis * rhs,
            y_axis: self.y_axis * rhs,
        }
    }
}

//// 3x3按列存储矩阵
pub struct Mat3 {
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
}
impl Mat3 {
    // 转置
    pub fn transpose(self) -> Self {
        Self {
            x_axis: Vec3 {
                x: self.x_axis.x,
                y: self.y_axis.x,
                z: self.z_axis.x,
            },
            y_axis: Vec3 {
                x: self.x_axis.y,
                y: self.y_axis.y,
                z: self.z_axis.y,
            },
            z_axis: Vec3 {
                x: self.x_axis.z,
                y: self.y_axis.z,
                z: self.z_axis.z,
            },
        }
    }
}
impl Add<Mat3> for Mat3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
            z_axis: self.z_axis + rhs.z_axis,
        }
    }
}
impl Sub<Mat3> for Mat3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
            z_axis: self.z_axis - rhs.z_axis,
        }
    }
}
impl Mul<f32> for Mat3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x_axis: self.x_axis * rhs,
            y_axis: self.y_axis * rhs,
            z_axis: self.z_axis * rhs,
        }
    }
}
