use crate::math::{Mat4, Quat, Vec3, Vec4};

// 缩放矩阵
pub fn scale_mat4(scale: Vec3) -> Mat4 {
    Mat4 {
        x_axis: Vec4 {
            x: scale.x,
            y: 0.,
            z: 0.,
            w: 0.,
        },
        y_axis: Vec4 {
            x: 0.,
            y: scale.y,
            z: 0.,
            w: 0.,
        },
        z_axis: Vec4 {
            x: 0.,
            y: 0.,
            z: scale.z,
            w: 0.,
        },
        w_axis: Vec4 {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 1.,
        },
    }
}

// 平移矩阵
pub fn translation_mat4(translation: Vec3) -> Mat4 {
    Mat4 {
        x_axis: Vec4 {
            x: 1.,
            y: 0.,
            z: 0.,
            w: 0.,
        },
        y_axis: Vec4 {
            x: 0.,
            y: 1.,
            z: 0.,
            w: 0.,
        },
        z_axis: Vec4 {
            x: 0.,
            y: 0.,
            z: 1.,
            w: 0.,
        },
        w_axis: Vec4 {
            x: translation.x,
            y: translation.y,
            z: translation.z,
            w: 1.,
        },
    }
}

// 旋转
pub fn rotation_quat(axis: Vec3, angle: f32) -> Quat {
    Quat::from_axis_angle(axis, angle)
}
