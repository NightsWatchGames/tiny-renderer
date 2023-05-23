use crate::{
    camera::Camera,
    math::{Mat4, Vec2, Vec3},
    model::{Mesh, Vertex},
};

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
    // 帧缓冲
    pub frame_buffer: Vec<u8>,
    // 深度缓冲
    pub depth_buffer: Vec<f32>,
}
impl Renderer {
    pub fn draw(&mut self, mesh: &Mesh, model_transformation: Mat4) {
        for i in 0..mesh.vertices.len() / 3 {
            let vertices = [
                mesh.vertices[i * 3],
                mesh.vertices[1 + i * 3],
                mesh.vertices[2 + i * 3],
            ];
            self.rasterize_trianlge(model_transformation, vertices);
        }
    }
    pub fn rasterize_trianlge(&mut self, model_transformation: Mat4, mut vertices: [Vertex; 3]) {
        // 模型变换
        for vertex in vertices.iter_mut() {
            vertex.position =
                (model_transformation * vertex.position.extend(1.0)).to_cartesian_point();
        }

        // 视图变换
        let view_transformation = self.camera.view_transformation();
        for vertex in vertices.iter_mut() {
            vertex.position =
                (view_transformation * vertex.position.extend(1.0)).to_cartesian_point();
        }

        // 投影变换
        let projection_transformation = self.camera.frustum.projection_transformation();
        for vertex in vertices.iter_mut() {
            vertex.position =
                (projection_transformation * vertex.position.extend(1.0)).to_cartesian_point();
        }

        // 视口变换
        for vertex in vertices.iter_mut() {
            vertex.position.x = (vertex.position.x + 1.0) * (self.viewport.width as f32 - 1.0)
                / 2.0
                + self.viewport.x as f32;
            vertex.position.y = (vertex.position.y + 1.0) * (self.viewport.height as f32 - 1.0)
                / 2.0
                + self.viewport.y as f32;
        }
    }
}

// 重心坐标
pub fn barycentric_2d(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> (f32, f32, f32) {
    barycentric_3d(p.extend(0.0), a.extend(0.0), b.extend(0.0), c.extend(0.0))
}
pub fn barycentric_3d(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> (f32, f32, f32) {
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let area_2abc = ab.cross(ac).length();
    let area_2pab = ab.cross(ap).length();
    let area_2pca = ac.cross(ap).length();
    let area_2pbc = area_2abc - area_2pab - area_2pca;
    let alpha = area_2pbc / area_2abc;
    let beta = area_2pca / area_2abc;
    let gamma = 1.0 - alpha - beta;
    (alpha, beta, gamma)
}
