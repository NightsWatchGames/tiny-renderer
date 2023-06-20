use crate::math::Vec3;

// 点光源
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    pub position: Vec3,
    // 光强度
    pub intensity: f32,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            position: Vec3::new(100.0, 100.0, -100.0),
            intensity: 1000.0,
        }
    }
}
