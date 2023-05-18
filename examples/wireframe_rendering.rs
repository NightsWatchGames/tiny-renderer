use image::{ImageBuffer, RgbImage};
use tiny_renderer::{line::draw_line, model::load_glft};

pub fn main() {
    let width = 100.0;
    let height = 100.0;
    let mut img: RgbImage = ImageBuffer::new(width as u32, height as u32);
    let positions = load_glft("assets/sphere/sphere.gltf");
    let mut last = positions.first().unwrap().clone();
    println!("last: {:?}", last);
    for pos in positions {
        draw_line(
            (
                (last[0] + 1.0) * width / 3.0,
                (last[1] + 1.0) * height / 3.0,
            )
                .into(),
            ((pos[0] + 1.0) * width / 3.0, (pos[1] + 1.0) * height / 3.0).into(),
            &mut img,
            [0, 255, 0],
        );
        last = pos;
    }
    img.save("screenshots/wireframe_rendering.png").unwrap();
}
