use gltf::{
    buffer::Data,
    image::Format,
    json::validation::Checked,
    mesh::util::{ReadColors, ReadTexCoords},
    texture::{MagFilter, MinFilter, WrappingMode},
    Document,
};
use std::collections::HashMap;

use crate::{
    color::Color,
    math::{Vec2, Vec3, Vec4},
    util::rand_color,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
    // 位置坐标
    pub position: Vec3,
    // 法线向量
    pub normal: Option<Vec3>,
    // 纹理坐标
    pub texcoord: Option<Vec2>,
    // 顶点颜色
    pub color: Option<Color>,
}
#[derive(Clone, Debug, Default)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

#[derive(Clone, Debug)]
pub struct Sampler {
    pub mag_filter: Option<MagFilter>,
    pub min_filter: Option<MinFilter>,
    pub wrap_s: WrappingMode,
    pub wrap_t: WrappingMode,
}

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

        if self.format != Format::R8G8B8 {
            panic!("Unsupported texture format: {:?}", self.format);
        }
        // 一个颜色占 3 个字节
        let index = (y * self.width as usize + x) * 3;
        Color::new(self.data[index], self.data[index + 1], self.data[index + 2])
    }
}

#[derive(Clone, Debug, Default)]
pub struct Primitive {
    // 顶点数据（拓扑类型为Triangles）
    pub vertices: Vec<Vertex>,
    pub material: Material,
}

#[derive(Clone, Debug, Default)]
pub struct Material {
    pub base_color_texture: Option<usize>,
    pub base_color_factor: Vec4,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub texture_id_map: HashMap<usize, Texture>,
}

pub fn load_glft(path: &str) -> Model {
    let (document, buffers, images) = gltf::import(path).unwrap();

    let textures = load_textures(&document, &images);
    let meshes = load_meshes(&document, &buffers);

    Model {
        meshes,
        texture_id_map: textures
            .into_iter()
            .map(|texture| (texture.id, texture))
            .collect(),
    }
}

pub fn load_textures(document: &Document, images: &Vec<gltf::image::Data>) -> Vec<Texture> {
    let mut textures = Vec::new();
    for texture in document.textures() {
        let source = texture.source();
        let sampler = texture.sampler();
        let image = images.get(source.index()).unwrap();

        let texture = Texture {
            id: texture.index(),
            width: image.width,
            height: image.height,
            format: image.format,
            data: image.pixels.clone(),
            sampler: Sampler {
                mag_filter: sampler.mag_filter(),
                min_filter: sampler.min_filter(),
                wrap_s: sampler.wrap_s(),
                wrap_t: sampler.wrap_t(),
            },
        };
        // println!(
        //     "Texture id: {:?}, width: {:?}, height: {:?}, format: {:?}, data len: {:?}, sampler: {:?}",
        //     texture.id,
        //     texture.width,
        //     texture.height,
        //     texture.format,
        //     texture.data.len(),
        //     texture.sampler
        // );
        textures.push(texture);
    }
    textures
}

pub fn load_meshes(document: &Document, buffers: &Vec<Data>) -> Vec<Mesh> {
    let mut meshes = Vec::new();
    // println!("Meshes len: {}", document.meshes().len());

    for gltf_mesh in document.meshes() {
        // println!("Primitives len: {}", gltf_mesh.primitives().len());

        let mut mesh = Mesh::default();

        for gltf_primitive in gltf_mesh.primitives() {
            let mut primitive = Primitive::default();

            // 顶点数据
            if gltf_primitive.mode() != gltf::mesh::Mode::Triangles {
                println!("Primitive mode: {:?}", gltf_primitive.mode());
                panic!("Only Triangles mode is supported");
            }
            let reader = gltf_primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let mut positions: Vec<[f32; 3]> = Vec::new();
            let mut normals: Vec<[f32; 3]> = Vec::new();
            let mut colors: Vec<[u8; 3]> = Vec::new();
            let mut texcoords: Vec<[f32; 2]> = Vec::new();

            for (semantic, _) in gltf_primitive.attributes() {
                match semantic {
                    gltf::Semantic::Positions => {
                        positions = reader.read_positions().unwrap().collect();
                    }
                    gltf::Semantic::Normals => {
                        normals = reader.read_normals().unwrap().collect();
                    }
                    gltf::Semantic::Colors(set) => {
                        colors = reader.read_colors(set).unwrap().into_rgb_u8().collect();
                    }
                    gltf::Semantic::TexCoords(set) => {
                        texcoords = reader.read_tex_coords(set).unwrap().into_f32().collect();
                    }
                    _ => {}
                }
            }

            let Some(indices) =  reader.read_indices() else {
                continue;
            };
            let indices = indices.into_u32();

            for index in indices {
                let vertex_position: Vec3 = positions.get(index as usize).unwrap().clone().into();
                let vertex_normal: Option<Vec3> =
                    normals.get(index as usize).map(|v| v.clone().into());
                let vertex_texcoord: Option<Vec2> =
                    texcoords.get(index as usize).map(|v| v.clone().into());
                let vertex_color: Option<Color> =
                    colors.get(index as usize).map(|v| v.clone().into());

                // println!("{:?}", vertex_texcoord);
                // println!("{:?}", vertex_position);
                primitive.vertices.push(Vertex {
                    position: vertex_position,
                    normal: vertex_normal,
                    texcoord: vertex_texcoord,
                    // 如果顶点没有颜色，就随机生成一个
                    color: vertex_color.or(Some(rand_color())),
                });
            }

            // 材质
            let mut material = Material::default();
            let gltf_material = gltf_primitive.material();
            material.base_color_factor = gltf_material
                .pbr_metallic_roughness()
                .base_color_factor()
                .into();
            material.base_color_texture = gltf_material
                .pbr_metallic_roughness()
                .base_color_texture()
                .map(|t| t.texture().index());
            primitive.material = material;

            mesh.primitives.push(primitive);
        }
        meshes.push(mesh);
    }
    meshes
}
