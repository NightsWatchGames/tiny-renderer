use gltf::{buffer::Data, Document};

use crate::{
    color::Color,
    material::Material,
    math::{Vec2, Vec3},
    mesh::{Mesh, Vertex},
    texture::{Sampler, Texture, TextureStorage},
    util::rand_color,
};

pub fn load_glft(path: &str) -> (Vec<Mesh>, TextureStorage) {
    let (document, buffers, images) = gltf::import(path).unwrap();

    let textures = load_textures(&document, &images);
    let meshes = load_meshes(&document, &buffers);

    (
        meshes,
        TextureStorage {
            texture_id_map: textures
                .into_iter()
                .map(|texture| (texture.id, texture))
                .collect(),
        },
    )
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

        for gltf_primitive in gltf_mesh.primitives() {
            let mut mesh = Mesh::default();

            // 顶点数据
            if gltf_primitive.mode() != gltf::mesh::Mode::Triangles {
                println!("Primitive mode: {:?}", gltf_primitive.mode());
                panic!("Only Triangles mode is supported");
            }
            let reader = gltf_primitive.reader(|buffer| Some(&buffers[buffer.index()]));

            let mut positions: Vec<[f32; 3]> = Vec::new();
            let mut normals: Vec<[f32; 3]> = Vec::new();
            let mut colors: Vec<[f32; 3]> = Vec::new();
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
                        colors = reader.read_colors(set).unwrap().into_rgb_f32().collect();
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
                mesh.vertices.push(Vertex {
                    position: vertex_position.extend(1.0),
                    normal: vertex_normal,
                    texcoord: vertex_texcoord,
                    // 如果顶点没有颜色，就随机生成一个
                    color: vertex_color.or(Some(rand_color())),
                });
            }

            // 材质
            let mut material = Material::default();
            // let gltf_material = gltf_primitive.material();
            mesh.material = material;

            meshes.push(mesh);
        }
    }
    meshes
}
