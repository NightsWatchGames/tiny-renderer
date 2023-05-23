use crate::math::{Vec2, Vec3, Vec4};

#[derive(Clone, Copy, Debug)]
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
