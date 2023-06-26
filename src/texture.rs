use std::collections::HashMap;

use gltf::{
    image::Format,
    texture::{MagFilter, MinFilter, WrappingMode},
};

use crate::{color::Color, math::Vec2};

#[derive(Clone, Debug)]
pub struct Sampler {
    pub mag_filter: Option<MagFilter>,
    pub min_filter: Option<MinFilter>,
    pub wrap_s: WrappingMode,
    pub wrap_t: WrappingMode,
}

#[derive(Debug)]
pub struct Texture {
    pub id: usize,
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub data: Vec<u8>,
    pub sampler: Sampler,
}
impl Texture {
    pub fn sample(&self, mut texcoord: Vec2) -> Color {
        if self.sampler.wrap_s != WrappingMode::Repeat
            || self.sampler.wrap_t != WrappingMode::Repeat
        {
            panic!("Unsupported texture wrap mode: {:?}", self.sampler.wrap_s)
        }
        if texcoord.x > 1.0 {
            texcoord.x -= texcoord.x.floor();
        }
        if texcoord.y > 1.0 {
            texcoord.y -= texcoord.y.floor();
        }
        let x = (texcoord.x * (self.width - 1) as f32) as usize;
        let y = (texcoord.y * (self.height - 1) as f32) as usize;

        let index = if self.format == Format::R8G8B8 {
            (y * self.width as usize + x) * 3
        } else if self.format == Format::R8G8B8A8 {
            (y * self.width as usize + x) * 4
        } else {
            panic!("Unsupported texture format: {:?}", self.format);
        };
        Color::new(
            self.data[index] as f32 / 255.,
            self.data[index + 1] as f32 / 255.,
            self.data[index + 2] as f32 / 255.,
        )
    }
}

#[derive(Debug, Default)]
pub struct TextureStorage {
    pub texture_id_map: HashMap<usize, Texture>,
}
