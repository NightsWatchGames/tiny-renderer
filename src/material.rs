use crate::math::Vec4;

// Blinn-Phong 材质
#[derive(Clone, Copy, Debug)]
pub struct Material {
    // 环境光反射系数Ka
    pub ambient: Vec4,
    // 漫反射系数Kd
    pub diffuse: Vec4,
    // 镜面反射系数Ks
    pub specular: Vec4,
    // 镜面反射高光度p
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            ambient: Vec4::new(0.1, 0.1, 0.1, 1.0),
            diffuse: Vec4::new(0.7, 0.7, 0.7, 1.0),
            specular: Vec4::new(1.0, 1.0, 1.0, 1.0),
            shininess: 64.0,
        }
    }
}
