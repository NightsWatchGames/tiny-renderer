use std::primitive;

use gltf::{
    buffer::Data,
    image::Format,
    json::extensions::material,
    mesh::util::{ReadColors, ReadTexCoords},
    Document,
};

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

pub struct Texture {
    pub id: usize,
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub data: Vec<u8>,
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
    pub textures: Vec<Texture>,
}

pub fn load_glft(path: &str) -> Model {
    let (document, buffers, images) = gltf::import(path).unwrap();

    let textures = load_textures(&document, &images);
    println!("Textures len: {}", textures.len());

    let meshes = load_meshes(&document, &buffers);
    Model { meshes, textures }
}

pub fn load_textures(document: &Document, images: &Vec<gltf::image::Data>) -> Vec<Texture> {
    let mut textures = Vec::new();
    for texture in document.textures() {
        let source = texture.source();
        let image = images.get(source.index()).unwrap();
        let texture = Texture {
            id: texture.index(),
            width: image.width,
            height: image.height,
            format: image.format,
            data: image.pixels.clone(),
        };
        println!(
            "Texture id: {:?}, width: {:?}, height: {:?}, format: {:?}, data len: {:?}",
            texture.id,
            texture.width,
            texture.height,
            texture.format,
            texture.data.len()
        );
        textures.push(texture);
    }
    textures
}

pub fn load_meshes(document: &Document, buffers: &Vec<Data>) -> Vec<Mesh> {
    let mut meshes = Vec::new();
    println!("Meshes len: {}", document.meshes().len());

    for gltf_mesh in document.meshes() {
        println!("Primitives len: {}", gltf_mesh.primitives().len());

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

                // println!("{:?}", vertex_position);
                primitive.vertices.push(Vertex {
                    position: vertex_position,
                    normal: vertex_normal,
                    texcoord: vertex_texcoord,
                    color: vertex_color,
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

pub fn custom_cube() -> Model {
    let p0 = Vec3::new(-1.0, 1.0, 1.0);
    let p1 = Vec3::new(1.0, 1.0, 1.0);
    let p2 = Vec3::new(-1.0, -1.0, 1.0);
    let p3 = Vec3::new(1.0, -1.0, 1.0);

    let p4 = Vec3::new(-1.0, 1.0, -1.0);
    let p5 = Vec3::new(1.0, 1.0, -1.0);
    let p6 = Vec3::new(-1.0, -1.0, -1.0);
    let p7 = Vec3::new(1.0, -1.0, -1.0);

    let mut vertices = Vec::new();

    vertices.append(&mut build_trangle(p0, p1, p2));
    vertices.append(&mut build_trangle(p1, p2, p3));

    vertices.append(&mut build_trangle(p0, p1, p4));
    vertices.append(&mut build_trangle(p1, p4, p5));

    vertices.append(&mut build_trangle(p0, p2, p4));
    vertices.append(&mut build_trangle(p2, p4, p6));

    vertices.append(&mut build_trangle(p1, p3, p5));
    vertices.append(&mut build_trangle(p3, p5, p7));

    vertices.append(&mut build_trangle(p2, p3, p6));
    vertices.append(&mut build_trangle(p3, p6, p7));

    vertices.append(&mut build_trangle(p4, p5, p6));
    vertices.append(&mut build_trangle(p5, p6, p7));

    Model {
        meshes: vec![Mesh {
            primitives: vec![Primitive {
                vertices,
                ..Default::default()
            }],
        }],
        textures: Vec::new(),
    }
}

pub fn build_trangle(p0: Vec3, p1: Vec3, p2: Vec3) -> Vec<Vertex> {
    vec![
        Vertex {
            position: p0,
            color: Some(rand_color()),
            ..Default::default()
        },
        Vertex {
            position: p1,
            color: Some(rand_color()),
            ..Default::default()
        },
        Vertex {
            position: p2,
            color: Some(rand_color()),
            ..Default::default()
        },
    ]
}
