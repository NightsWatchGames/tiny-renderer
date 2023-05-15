use std::{fs, io};

pub fn load_glft(path: &str) -> Vec<[f32; 3]> {
    // let file = fs::File::open(&path).unwrap();
    // let reader = io::BufReader::new(file);
    // let gltf = gltf::Gltf::from_reader(reader).unwrap();
    let (gltf, buffers, _) = gltf::import(path).unwrap();
    let mut vertex_positions = Vec::new();
    for mesh in gltf.meshes() {
        println!("Mesh #{}", mesh.index());
        for primitive in mesh.primitives() {
            println!("- Primitive #{}", primitive.index());
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(iter) = reader.read_positions() {
                for vertex_position in iter {
                    println!("{:?}", vertex_position);
                    vertex_positions.push(vertex_position);
                }
            }
        }
     }
     vertex_positions
    // println!("{:#?}", gltf);
}
