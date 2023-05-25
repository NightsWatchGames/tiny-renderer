use std::ops::{Add, Mul, Neg, Sub};

//// 二维向量
#[derive(Debug, Clone, Copy, Default)]
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
        assert!(normalized.is_finite());
        normalized
    }
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }
    pub fn extend(self, z: f32) -> Vec3 {
        Vec3::new(self.x, self.y, z)
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
impl From<Vec2> for (f32, f32) {
    fn from(v: Vec2) -> Self {
        (v.x, v.y)
    }
}
impl From<(f32, f32)> for Vec2 {
    fn from(v: (f32, f32)) -> Self {
        Vec2::new(v.0, v.1)
    }
}

//// 三维向量
#[derive(Debug, Clone, Copy, Default)]
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
        assert!(normalized.is_finite());
        normalized
    }
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
    pub fn is_normalized(self) -> bool {
        (self.length() - 1.0).abs() < 1e-4
    }
    pub fn extend(self, w: f32) -> Vec4 {
        Vec4::new(self.x, self.y, self.z, w)
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
impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl From<Vec3> for (f32, f32, f32) {
    fn from(v: Vec3) -> Self {
        (v.x, v.y, v.z)
    }
}
impl From<[f32; 3]> for Vec3 {
    fn from(v: [f32; 3]) -> Self {
        Vec3::new(v[0], v[1], v[2])
    }
}

//// 四维向量
#[derive(Debug, Clone, Copy, Default)]
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
    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }
    // 归一化
    pub fn normalize(self) -> Self {
        let normalized = self.mul(self.length().recip());
        assert!(normalized.is_finite());
        normalized
    }
    pub fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }
    pub fn is_normalized(self) -> bool {
        (self.length_squared() - 1.0).abs() <= 1e-4
    }
    pub fn to_cartesian_point(self) -> Vec3 {
        assert!(self.w != 0.0);
        Vec3::new(self.x / self.w, self.y / self.w, self.z / self.w)
    }
    pub fn to_cartesian_vector(self) -> Vec3 {
        assert!(self.w == 0.0);
        Vec3::new(self.x, self.y, self.z)
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
#[derive(Clone, Copy, Debug)]
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
impl Mul<Mat3> for Mat3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x_axis: self * rhs.x_axis,
            y_axis: self * rhs.y_axis,
            z_axis: self * rhs.z_axis,
        }
    }
}

//// 4x4按列存储矩阵
#[derive(Debug, Clone, Copy)]
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
    pub const fn from_rows_slice(slice: &[f32]) -> Self {
        assert!(slice.len() >= 16);
        Self {
            x_axis: Vec4::new(slice[0], slice[4], slice[8], slice[12]),
            y_axis: Vec4::new(slice[1], slice[5], slice[9], slice[13]),
            z_axis: Vec4::new(slice[2], slice[6], slice[10], slice[14]),
            w_axis: Vec4::new(slice[3], slice[7], slice[11], slice[15]),
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
impl Mul<Mat4> for Mat4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            x_axis: self * rhs.x_axis,
            y_axis: self * rhs.y_axis,
            z_axis: self * rhs.z_axis,
            w_axis: self * rhs.w_axis,
        }
    }
}

//// 四元数
#[derive(Copy, Clone, Debug)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Quat {
    pub const ZERO: Self = Self::from_xyzw(0.0, 0.0, 0.0, 0.0);
    // 恒等四元数（无旋转）
    pub const IDENTITY: Self = Self::from_xyzw(0.0, 0.0, 0.0, 1.0);

    pub const fn from_vec4(v: Vec4) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: v.w,
        }
    }
    pub const fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    // 共轭
    pub fn conjugate(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: self.w,
        }
    }
    // 四元数的逆
    pub fn inverse(self) -> Self {
        assert!(self.is_normalized());
        self.conjugate()
    }
    // 绕轴旋转
    pub fn from_axis_angle(axis: Vec3, angle: f32) -> Self {
        assert!(axis.is_normalized());
        let (s, c) = (angle * 0.5).sin_cos();
        let v = axis * s;
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: c,
        }
    }
    // TODO 三维旋转矩阵->四元数 参考glam
    pub fn from_mat3(mat: &Mat3) -> Self {
        // Based on https://github.com/microsoft/DirectXMath `XM$quaternionRotationMatrix`
        let (m00, m01, m02) = mat.x_axis.into();
        let (m10, m11, m12) = mat.y_axis.into();
        let (m20, m21, m22) = mat.z_axis.into();
        if m22 <= 0.0 {
            // x^2 + y^2 >= z^2 + w^2
            let dif10 = m11 - m00;
            let omm22 = 1.0 - m22;
            if dif10 <= 0.0 {
                // x^2 >= y^2
                let four_xsq = omm22 - dif10;
                let inv4x = 0.5 / four_xsq.sqrt();
                Self::from_xyzw(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                // y^2 >= x^2
                let four_ysq = omm22 + dif10;
                let inv4y = 0.5 / four_ysq.sqrt();
                Self::from_xyzw(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            // z^2 + w^2 >= x^2 + y^2
            let sum10 = m11 + m00;
            let opm22 = 1.0 + m22;
            if sum10 <= 0.0 {
                // z^2 >= w^2
                let four_zsq = opm22 - sum10;
                let inv4z = 0.5 / four_zsq.sqrt();
                Self::from_xyzw(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                // w^2 >= z^2
                let four_wsq = opm22 + sum10;
                let inv4w = 0.5 / four_wsq.sqrt();
                Self::from_xyzw(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }
    // 四元数转换为旋转矩阵（齐次坐标）
    pub fn to_mat4(self) -> Mat4 {
        assert!(self.is_normalized());
        let (x, y, z, w) = (self.x, self.y, self.z, self.w);
        let (xx, yy, zz) = (x * x, y * y, z * z);
        let (xy, xz, yz) = (x * y, x * z, y * z);
        let (wx, wy, wz) = (w * x, w * y, w * z);
        Mat4::from_cols(
            Vec4::new(1.0 - 2.0 * (yy + zz), 2.0 * (xy + wz), 2.0 * (xz - wy), 0.0),
            Vec4::new(2.0 * (xy - wz), 1.0 - 2.0 * (xx + zz), 2.0 * (yz + wx), 0.0),
            Vec4::new(2.0 * (xz + wy), 2.0 * (yz - wx), 1.0 - 2.0 * (xx + yy), 0.0),
            Vec4::W,
        )
    }
    pub fn length(self) -> f32 {
        Vec4::new(self.x, self.y, self.z, self.w).length()
    }
    pub fn length_squared(self) -> f32 {
        Vec4::new(self.x, self.y, self.z, self.w).length_squared()
    }
    pub fn is_normalized(self) -> bool {
        Vec4::new(self.x, self.y, self.z, self.w).is_normalized()
    }
}
impl Add<Quat> for Quat {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl Sub<Quat> for Quat {
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
impl Mul<f32> for Quat {
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
impl Mul<Quat> for Quat {
    type Output = Quat;
    fn mul(self, rhs: Quat) -> Self::Output {
        Self::Output {
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y + self.y * rhs.w + self.z * rhs.x - self.x * rhs.z,
            z: self.w * rhs.z + self.z * rhs.w + self.x * rhs.y - self.y * rhs.x,
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
        }
    }
}
impl Mul<Vec3> for Quat {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let q = Quat::from_vec4(Vec4::new(rhs.x, rhs.y, rhs.z, 0.0));
        let self_inv = self.inverse();
        let v = self * q * self_inv;
        Vec3::new(v.x, v.y, v.z)
    }
}
