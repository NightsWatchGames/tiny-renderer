use image::{ImageBuffer, RgbImage};
use tiny_renderer::{line::draw_line, model::load_glft};

pub fn main() {
    let mut img: RgbImage = ImageBuffer::new(100, 100);
    let positions = load_glft("assets/cube.gltf");
    let mut last = positions.first().unwrap().clone();
    println!("last: {:?}", last);
    for pos in positions {
        println!("pos: {:?}", pos);
        draw_line(
            ((last[0] * 50.) as i32, (last[1] * 50.) as i32),
            ((pos[0] * 50.) as i32, (pos[1] * 50.) as i32),
            &mut img,
            [0, 255, 0],
        );
    }
}
