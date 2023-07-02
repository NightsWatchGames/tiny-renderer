use crate::{
    color::Color,
    material::Material,
    math::{Vec2, Vec3, Vec4},
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
    // 位置坐标（齐次坐标）
    pub position: Vec4,
    // 法线向量
    pub normal: Option<Vec3>,
    // 纹理坐标
    pub texcoord: Option<Vec2>,
    // 顶点颜色
    pub color: Option<Color>,
}
#[derive(Clone, Debug, Default)]
pub struct Mesh {
    // 顶点数据（拓扑类型为Triangles）
    pub vertices: Vec<Vertex>,
    pub material: Material,
}
