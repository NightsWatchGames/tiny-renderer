use crate::math::{Vec2, Vec3, Vec4};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
    // 位置坐标
    pub position: Vec3,
    // 法线向量
    pub normal: Vec3,
    // 纹理坐标
    pub texcoord: Vec2,
    // 顶点颜色
    pub color: Vec4,
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
            if let Some(iter) = reader.read_positions() {
                println!("Positions len: {}", iter.len());
                for vertex_position in iter {
                    // println!("{:?}", vertex_position);
                    // TODO
                    mesh.vertices.push(Vertex {
                        position: vertex_position.into(),
                        normal: Vec3::ZERO,
                        texcoord: Vec2::ZERO,
                        color: Vec4::ZERO,
                    });
                }
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
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-1.0, 0.0, 0.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                ..Default::default()
            },
            // 三角形2
            Vertex {
                position: Vec3::new(1.0, 0.0, 0.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(-1.0, 0.0, 0.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            // 三角形3
            Vertex {
                position: Vec3::new(1.0, 0.0, 0.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 0.0, -1.0),
                ..Default::default()
            },
            // 三角形4
            Vertex {
                position: Vec3::new(-1.0, 0.0, 0.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 1.0, 0.0),
                ..Default::default()
            },
            Vertex {
                position: Vec3::new(0.0, 0.0, -1.0),
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
            ..Default::default()
        },
        Vertex {
            position: p1,
            ..Default::default()
        },
        Vertex {
            position: p2,
            ..Default::default()
        },
    ]
}
