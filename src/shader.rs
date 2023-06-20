use std::collections::HashMap;

use crate::{
    color::Color,
    light::PointLight,
    math::{Mat4, Vec2, Vec3, Vec4},
    mesh::Vertex,
    texture::TextureStorage, material::Material,
};

pub type VertexShader = Box<dyn Fn(&mut Vertex)>;
pub type FragmentShader = Box<dyn Fn(&TextureStorage, &PointLight, &Material, Vec2) -> Color>;

#[derive(Debug, Default)]
pub struct Uniforms {
    pub int: HashMap<u32, i32>,
    pub float: HashMap<u32, f32>,
    pub vec2: HashMap<u32, Vec2>,
    pub vec3: HashMap<u32, Vec3>,
    pub vec4: HashMap<u32, Vec4>,
    pub mat4: HashMap<u32, Mat4>,
    pub texture: HashMap<u32, u32>,
}

pub fn phong_shader() -> FragmentShader {
    Box::new(|texture_storage, light, material, texcoord| {
        // 漫反射
        let kd = material.diffuse;
        
        if let Some(texture) = texture_storage.texture_id_map.get(&0) {
            return texture.sample(texcoord);
        }
        Color::BLACK
    })
}
