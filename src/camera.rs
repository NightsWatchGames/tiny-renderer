use crate::{
    math::{Mat3, Mat4, Quat, Vec3, Vec4},
    transform::translation_mat4,
};

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
impl Frustum {
    // 透视投影变换矩阵
    #[rustfmt::skip]
    pub fn projection_transformation(&self) -> Mat4 {
        let persp_to_ortho = Mat4::from_rows_slice(&[
            self.near, 0., 0., 0.,
            0., self.near, 0., 0.,
            0., 0., self.near + self.far, -self.near * self.far,
            0., 0., 1., 0.,
        ]);
        let ortho_translation = Mat4::from_rows_slice(&[
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., -(self.near + self.far) / 2.,
            0., 0., 0., 1.,
        ]);
        let ortho_scale = Mat4::from_rows_slice(&[
            2. / self.width_near(), 0., 0., 0.,
            0., 2. / self.height_near(), 0., 0.,
            0., 0., 2. / (self.near - self.far), 0.,
            0., 0., 0., 1.,
        ]);
        ortho_scale * ortho_translation * persp_to_ortho
    }
    pub fn width_near(&self) -> f32 {
        self.aspect * self.height_near()
    }
    pub fn height_near(&self) -> f32 {
        2.0 * (self.fov / 2.0).tan() * self.near.abs()
    }
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
        // TODO 通过逆矩阵来计算旋转矩阵
        self.rotation = Quat::from_mat3(&Mat3::from_cols(right, up, back));
    }
    // 视图变换矩阵
    pub fn view_transformation(&self) -> Mat4 {
        let rotation_mat4 = self.rotation.to_mat4();
        let translation_mat4 = translation_mat4(-self.position);
        // 先平移再旋转
        rotation_mat4 * translation_mat4
    }
}
