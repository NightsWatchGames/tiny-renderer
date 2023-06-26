use crate::math::{Vec3, Vec4};

// Blinn-Phong 材质
#[derive(Clone, Copy, Debug)]
pub struct Material {
    // 环境光反射系数Ka
    pub ambient: Vec3,
    // 漫反射系数Kd
    pub diffuse: Vec3,
    // 镜面反射系数Ks
    pub specular: Vec3,
    // 镜面反射高光度p
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Vec3::new(0.1, 0.1, 0.1),
            diffuse: Vec3::new(0.7, 0.7, 0.7),
            specular: Vec3::new(1.0, 1.0, 1.0),
            shininess: 64.0,
        }
    }
}
