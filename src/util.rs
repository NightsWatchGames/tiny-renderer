use rand::Rng;

use crate::{
    color::Color,
    math::{Vec3, Vec4},
    mesh::{Mesh, Vertex},
    texture::TextureStorage,
};

pub fn flip_vertically(frame_buffer: &Vec<u8>, width: usize, height: usize) -> Vec<u8> {
    let mut flipped_frame_buffer = frame_buffer.clone();
    for y in 0..height / 2 {
        for x in 0..width {
            let top_index = (y * width + x) * 3;
            let bottom_index = ((height - y - 1) * width + x) * 3;
            flipped_frame_buffer.swap(top_index, bottom_index);
            flipped_frame_buffer.swap(top_index + 1, bottom_index + 1);
            flipped_frame_buffer.swap(top_index + 2, bottom_index + 2);
        }
    }
    flipped_frame_buffer
}

pub fn rand_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::new(
        rng.gen_range(0.0..=1.0),
        rng.gen_range(0.0..=1.0),
        rng.gen_range(0.0..=1.0),
    )
}

pub fn custom_cube() -> (Vec<Mesh>, TextureStorage) {
    let p0 = Vec3::new(-1.0, 1.0, 1.0).extend(1.0);
    let p1 = Vec3::new(1.0, 1.0, 1.0).extend(1.0);
    let p2 = Vec3::new(-1.0, -1.0, 1.0).extend(1.0);
    let p3 = Vec3::new(1.0, -1.0, 1.0).extend(1.0);

    let p4 = Vec3::new(-1.0, 1.0, -1.0).extend(1.0);
    let p5 = Vec3::new(1.0, 1.0, -1.0).extend(1.0);
    let p6 = Vec3::new(-1.0, -1.0, -1.0).extend(1.0);
    let p7 = Vec3::new(1.0, -1.0, -1.0).extend(1.0);

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

    (
        vec![Mesh {
            vertices,
            ..Default::default()
        }],
        TextureStorage::default(),
    )
}

pub fn build_trangle(p0: Vec4, p1: Vec4, p2: Vec4) -> Vec<Vertex> {
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
