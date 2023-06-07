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
    // 近平面（距离）
    pub near: f32,
    // 远平面（距离）
    pub far: f32,
}
impl Frustum {
    // 透视投影变换矩阵
    #[rustfmt::skip]
    pub fn persp_projection_transformation(&self) -> Mat4 {
        let near_z = -self.near;
        let far_z = -self.far;
        let persp_to_ortho = Mat4::from_rows_slice(&[
            near_z, 0., 0., 0.,
            0., near_z, 0., 0.,
            0., 0., near_z + far_z, -near_z * far_z,
            0., 0., 1., 0.,
        ]);
        let ortho_translation = Mat4::from_rows_slice(&[
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., -(near_z + far_z) / 2.,
            0., 0., 0., 1.,
        ]);
        let ortho_scale = Mat4::from_rows_slice(&[
            2. / self.width_near(), 0., 0., 0.,
            0., 2. / self.height_near(), 0., 0.,
            0., 0., 2. / (near_z - far_z), 0.,
            0., 0., 0., 1.,
        ]);
        ortho_scale * ortho_translation * persp_to_ortho
    }

    // 正交投影变换矩阵
    #[rustfmt::skip]
    pub fn ortho_projection_transformation(&self) -> Mat4 {
        let near_z = -self.near;
        let far_z = -self.far;
        let ortho_translation = Mat4::from_rows_slice(&[
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., -(near_z + far_z) / 2.,
            0., 0., 0., 1.,
        ]);
        let ortho_scale = Mat4::from_rows_slice(&[
            2. / self.width_near(), 0., 0., 0.,
            0., 2. / self.height_near(), 0., 0.,
            0., 0., 2. / (near_z - far_z), 0.,
            0., 0., 0., 1.,
        ]);
        ortho_scale * ortho_translation
    }

    pub fn width_near(&self) -> f32 {
        self.aspect * self.height_near()
    }

    pub fn height_near(&self) -> f32 {
        2.0 * (self.fov / 2.0).tan() * self.near
    }
}

//// 相机
pub struct Camera {
    pub frustum: Frustum,
    pub position: Vec3,
    pub rotation: Quat,
}
impl Camera {
    pub fn new(near: f32, far: f32, aspect: f32, fov: f32, position: Vec3) -> Self {
        assert!(near > 0.0);
        assert!(far > 0.0);
        Self {
            frustum: Frustum {
                fov,
                aspect,
                near,
                far,
            },
            position,
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
        let rotation_mat4 = self.rotation.inverse().to_mat4();
        let translation_mat4 = translation_mat4(-self.position);
        // 先平移再旋转
        rotation_mat4 * translation_mat4
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation;
    }
    pub fn translate_around(&mut self, point: Vec3, rotation: Quat) {
        self.position = point + rotation * (self.position - point);
    }
    pub fn rotate_around(&mut self, point: Vec3, rotation: Quat) {
        self.translate_around(point, rotation);
        self.rotate(rotation);
    }
}
