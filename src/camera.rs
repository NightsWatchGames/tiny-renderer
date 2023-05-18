use crate::math::{Vec3, Quat, Mat3};

//// 视椎体
pub struct Frustum {
    // 垂直视野（弧度）
    pub fov: f32,
    // 宽高比
    pub aspect: f32,
    // 近平面
    pub near: f32,
    // 远平面
    pub far: f32,
}

//// 相机
pub struct Camera {
    pub frustum: Frustum,
    pub position: Vec3,
    pub rotation: Quat,
}
impl Camera {
    pub fn new(near: f32, far: f32, aspect: f32, fov: f32) -> Self {
        Self {
            frustum: Frustum {
                fov,
                aspect,
                near,
                far,
            },
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        self.look_to(target - self.position, up);
    }
    pub fn look_to(&mut self, direction: Vec3, up: Vec3) {
        let back = -direction.normalize();
        let right = up.cross(back).normalize();
        let up = back.cross(right);
        self.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, back));
    }
}