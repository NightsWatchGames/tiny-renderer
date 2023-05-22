use crate::camera::Camera;

//// 视口
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    // 视口左上角的坐标
    pub x: i32,
    pub y: i32,
    // 视口的宽高
    pub width: u32,
    pub height: u32,
}

pub struct Renderer {
    pub camera: Camera,
    pub viewport: Viewport,
}
impl Renderer {
    pub fn rasterize_trianlge() {}
}
