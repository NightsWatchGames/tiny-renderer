use crate::{color::Color, math::Vec2, model::Vertex, texture::TextureStorage};

pub type VertexShader = Box<dyn Fn(&mut Vertex)>;
pub type FragmentShader = Box<dyn Fn(&TextureStorage, Vec2) -> Color>;
