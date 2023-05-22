use image::{ImageBuffer, RgbImage};
use tiny_renderer::{line::draw_line, model::load_glft};

pub fn main() {
    let width = 100.0;
    let height = 100.0;
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);
    let meshes = load_glft("assets/sphere/sphere.gltf");
    for mesh in meshes {
        let mut i = 0;
        loop {
            if i > mesh.vertices.len() - 3 {
                break;
            }
            // TODO 改成 draw_triangle
            draw_line(
                (
                    (mesh.vertices[i].position.x + 1.0) * width / 3.0,
                    (mesh.vertices[i].position.y + 1.0) * height / 3.0,
                )
                    .into(),
                (
                    (mesh.vertices[i + 1].position.x + 1.0) * width / 3.0,
                    (mesh.vertices[i + 1].position.y + 1.0) * height / 3.0,
                )
                    .into(),
                &mut img,
                [0, 255, 0],
            );
            draw_line(
                (
                    (mesh.vertices[i + 1].position.x + 1.0) * width / 3.0,
                    (mesh.vertices[i + 1].position.y + 1.0) * height / 3.0,
                )
                    .into(),
                (
                    (mesh.vertices[i + 2].position.x + 1.0) * width / 3.0,
                    (mesh.vertices[i + 2].position.y + 1.0) * height / 3.0,
                )
                    .into(),
                &mut img,
                [0, 255, 0],
            );
            draw_line(
                (
                    (mesh.vertices[i + 2].position.x + 1.0) * width / 3.0,
                    (mesh.vertices[i + 2].position.y + 1.0) * height / 3.0,
                )
                    .into(),
                (
                    (mesh.vertices[i].position.x + 1.0) * width / 3.0,
                    (mesh.vertices[i].position.y + 1.0) * height / 3.0,
                )
                    .into(),
                &mut img,
                [0, 255, 0],
            );

            i += 3;
        }
    }
    img.save("screenshots/wireframe_rendering.png").unwrap();
}
