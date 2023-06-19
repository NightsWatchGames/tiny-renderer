use crate::math::Vec4;

// Blinn-Phong 材质
#[derive(Clone, Debug, Default)]
pub struct Material {
    pub base_color_texture: Option<usize>,
    pub base_color_factor: Vec4,
}
