use gltf::mesh::util::{ReadColors, ReadTexCoords};

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
#[derive(Clone, Debug)]
pub struct Mesh {
    // 顶点数据（拓扑类型为Triangles）
    pub vertices: Vec<Vertex>,
}

pub fn load_glft(path: &str) -> Vec<Mesh> {
    let (gltf, buffers, _) = gltf::import(path).unwrap();
    println!("Meshes len: {}", gltf.meshes().len());

    let mut meshes = Vec::new();

    for gltf_mesh in gltf.meshes() {
        let mut mesh = Mesh {
            vertices: Vec::new(),
        };
        println!("Primitives len: {}", gltf_mesh.primitives().len());

        for primitive in gltf_mesh.primitives() {
            if primitive.mode() != gltf::mesh::Mode::Triangles {
                println!("Primitive mode: {:?}", primitive.mode());
                panic!("Only Triangles mode is supported");
            }
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let mut positions: Vec<[f32; 3]> = Vec::new();
            let mut normals: Vec<[f32; 3]> = Vec::new();
            let mut colors: Vec<[u8; 3]> = Vec::new();
            let mut texcoords: Vec<[f32; 2]> = Vec::new();

            for (semantic, _) in primitive.attributes() {
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
                mesh.vertices.push(Vertex {
                    position: vertex_position,
                    normal: vertex_normal,
                    texcoord: vertex_texcoord,
                    color: vertex_color,
                });
            }
        }
        meshes.push(mesh);
    }
    meshes
}

pub fn custom_mesh() -> Mesh {
    Mesh {
        vertices: vec![
            // 三角形1
            Vertex {
                position: Vec3::new(1.0, 0.0, 0.0),
                color: Some(Color::RED),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-1.0, 0.0, 0.0),
                color: Some(Color::RED),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                color: Some(Color::RED),
                ..Default::default()
            },
            // 三角形2
            Vertex {
                position: Vec3::new(1.0, 0.0, 0.0),
                color: Some(Color::BLUE),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-1.0, 0.0, 0.0),
                color: Some(Color::BLUE),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 0.0, -1.0),
                color: Some(Color::BLUE),
                ..Default::default()
            },
            // 三角形3
            Vertex {
                position: Vec3::new(1.0, 0.0, 0.0),
                color: Some(Color::GREEN),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                color: Some(Color::GREEN),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 0.0, -1.0),
                color: Some(Color::GREEN),
                ..Default::default()
            },
            // 三角形4
            Vertex {
                position: Vec3::new(-1.0, 0.0, 0.0),
                color: Some(Color::WHITE),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                color: Some(Color::WHITE),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 0.0, -1.0),
                color: Some(Color::WHITE),
                ..Default::default()
            },
        ],
    }
}

pub fn custom_cube() -> Mesh {
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

    Mesh { vertices }
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
