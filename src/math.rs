use std::ops::{Add, Mul, Sub};

//// 二维向量
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const NAN: Self = Self::splat(f32::NAN);
    pub const X: Self = Self::new(1.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0);
    pub const NEG_X: Self = Self::new(-1.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }
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
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const NAN: Self = Self::splat(f32::NAN);
    pub const X: Self = Self::new(1.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0);
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v, z: v }
    }
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

//// 四维向量
#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vec4 {
    pub const ZERO: Self = Self::splat(0.0);
    pub const ONE: Self = Self::splat(1.0);
    pub const NEG_ONE: Self = Self::splat(-1.0);
    pub const NAN: Self = Self::splat(f32::NAN);
    pub const X: Self = Self::new(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = Self::new(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = Self::new(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const NEG_X: Self = Self::new(-1.0, 0.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self::new(0.0, -1.0, 0.0, 0.0);
    pub const NEG_Z: Self = Self::new(0.0, 0.0, -1.0, 0.0);
    pub const NEG_W: Self = Self::new(0.0, 0.0, 0.0, -1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    pub const fn splat(v: f32) -> Self {
        Self {
            x: v,
            y: v,
            z: v,
            w: v,
        }
    }
    // 点乘
    pub fn dot(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
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
                "Vec4::normalize: normalized is not finite: {:?}",
                normalized
            );
        }
        normalized
    }
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }
}
impl Add<Vec4> for Vec4 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl Sub<Vec4> for Vec4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl Mul<f32> for Vec4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

//// 2x2按列存储矩阵
pub struct Mat2 {
    pub x_axis: Vec2,
    pub y_axis: Vec2,
}
impl Mat2 {
    pub const ZERO: Self = Self::from_cols(Vec2::ZERO, Vec2::ZERO);
    // 单位矩阵
    pub const IDENTITY: Self = Self::from_cols(Vec2::X, Vec2::Y);

    pub const fn from_cols(x_axis: Vec2, y_axis: Vec2) -> Self {
        Self { x_axis, y_axis }
    }
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
    // TODO 逆矩阵
    pub fn inverse(self) -> Self {
        todo!()
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
impl Mul<Vec2> for Mat2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Self::Output {
            x: self.x_axis.x * rhs.x + self.y_axis.x * rhs.y,
            y: self.x_axis.y * rhs.x + self.y_axis.y * rhs.y,
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
    pub const ZERO: Self = Self::from_cols(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
    // 单位矩阵
    pub const IDENTITY: Self = Self::from_cols(Vec3::X, Vec3::Y, Vec3::Z);

    pub const fn from_cols(x_axis: Vec3, y_axis: Vec3, z_axis: Vec3) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
        }
    }
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
    // TODO 逆矩阵
    pub fn inverse(self) -> Self {
        todo!()
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
impl Mul<Vec3> for Mat3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            x: self.x_axis.x * rhs.x + self.y_axis.x * rhs.y + self.z_axis.x * rhs.z,
            y: self.x_axis.y * rhs.x + self.y_axis.y * rhs.y + self.z_axis.y * rhs.z,
            z: self.x_axis.z * rhs.x + self.y_axis.z * rhs.y + self.z_axis.z * rhs.z,
        }
    }
}

//// 4x4按列存储矩阵
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}
impl Mat4 {
    pub const ZERO: Self = Self::from_cols(Vec4::ZERO, Vec4::ZERO, Vec4::ZERO, Vec4::ZERO);
    // 单位矩阵
    pub const IDENTITY: Self = Self::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);

    pub const fn from_cols(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }
    // 转置
    pub fn transpose(self) -> Self {
        Self {
            x_axis: Vec4 {
                x: self.x_axis.x,
                y: self.y_axis.x,
                z: self.z_axis.x,
                w: self.w_axis.x,
            },
            y_axis: Vec4 {
                x: self.x_axis.y,
                y: self.y_axis.y,
                z: self.z_axis.y,
                w: self.w_axis.y,
            },
            z_axis: Vec4 {
                x: self.x_axis.z,
                y: self.y_axis.z,
                z: self.z_axis.z,
                w: self.w_axis.z,
            },
            w_axis: Vec4 {
                x: self.x_axis.w,
                y: self.y_axis.w,
                z: self.z_axis.w,
                w: self.w_axis.w,
            },
        }
    }
    // TODO 逆矩阵
    pub fn inverse(self) -> Self {
        todo!()
    }
}
impl Add<Mat4> for Mat4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x_axis: self.x_axis + rhs.x_axis,
            y_axis: self.y_axis + rhs.y_axis,
            z_axis: self.z_axis + rhs.z_axis,
            w_axis: self.w_axis + rhs.w_axis,
        }
    }
}
impl Sub<Mat4> for Mat4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            x_axis: self.x_axis - rhs.x_axis,
            y_axis: self.y_axis - rhs.y_axis,
            z_axis: self.z_axis - rhs.z_axis,
            w_axis: self.w_axis - rhs.w_axis,
        }
    }
}
impl Mul<f32> for Mat4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        Self {
            x_axis: self.x_axis * rhs,
            y_axis: self.y_axis * rhs,
            z_axis: self.z_axis * rhs,
            w_axis: self.w_axis * rhs,
        }
    }
}
impl Mul<Vec4> for Mat4 {
    type Output = Vec4;
    fn mul(self, rhs: Vec4) -> Self::Output {
        Self::Output {
            x: self.x_axis.x * rhs.x
                + self.y_axis.x * rhs.y
                + self.z_axis.x * rhs.z
                + self.w_axis.x * rhs.w,
            y: self.x_axis.y * rhs.x
                + self.y_axis.y * rhs.y
                + self.z_axis.y * rhs.z
                + self.w_axis.y * rhs.w,
            z: self.x_axis.z * rhs.x
                + self.y_axis.z * rhs.y
                + self.z_axis.z * rhs.z
                + self.w_axis.z * rhs.w,
            w: self.x_axis.w * rhs.x
                + self.y_axis.w * rhs.y
                + self.z_axis.w * rhs.z
                + self.w_axis.w * rhs.w,
        }
    }
}

//// TODO 四元数
#[derive(Copy, Clone, Debug)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
