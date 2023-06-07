use crate::{
    color::Color,
    math::Vec2,
    model::{Model, Vertex},
};

pub type VertexShader = Box<dyn Fn(&mut Vertex)>;
pub type FragmentShader = Box<dyn Fn(&Model, Vec2) -> Color>;
