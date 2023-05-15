use image::{ImageBuffer, RgbImage};
use tiny_renderer::line::draw_line;

pub fn main() {
    let mut img: RgbImage = ImageBuffer::new(100, 100);
    // 斜率大于0小于1
    draw_line((10, 10), (90, 60), &mut img, [0, 255, 0]);
    draw_line((90, 60), (10, 10), &mut img, [0, 255, 0]);
    // 斜率大于1
    draw_line((10, 10), (60, 90), &mut img, [0, 255, 0]);
    draw_line((60, 90), (10, 10), &mut img, [0, 255, 0]);
    // 斜率小于0大于-1
    draw_line((10, 90), (90, 40), &mut img, [0, 255, 0]);
    draw_line((90, 40), (10, 90), &mut img, [0, 255, 0]);
    // 斜率小于-1
    draw_line((10, 90), (40, 10), &mut img, [0, 255, 0]);
    draw_line((40, 10), (10, 90), &mut img, [0, 255, 0]);
    // 斜率为0
    draw_line((10, 50), (90, 50), &mut img, [0, 55, 0]);
    // 斜率无穷大
    draw_line((50, 10), (50, 90), &mut img, [0, 55, 0]);
    img.save("screenshots/bresenham_line.png").unwrap();
}
