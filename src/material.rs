use crate::math::Vec4;

// Blinn-Phong 材质
#[derive(Clone, Debug, Default)]
pub struct Material {
    // 环境光反射系数
    pub ambient: Vec4,
    // 漫反射系数
    pub diffuse: Vec4,
    // 镜面反射系数
    pub specular: Vec4,
    // 镜面反射高光度
    pub shininess: f32,
}
