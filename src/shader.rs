use crate::{color::Color, model::Vertex};

pub trait VertexShader {
    fn shade(&self, vertex: Vertex) -> Vertex;
}
pub trait FragmentShader {
    fn shade(&self) -> Color;
}
