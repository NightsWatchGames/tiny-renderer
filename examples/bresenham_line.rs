use image::{ImageBuffer, RgbImage};
use tiny_renderer::line::draw_line;

pub fn main() {
    let mut img: RgbImage = ImageBuffer::new(100, 100);
    // 斜率大于0小于1
    draw_line((10., 10.).into(), (90., 60.).into(), &mut img, [0, 255, 0]);
    draw_line((90., 60.).into(), (10., 10.).into(), &mut img, [0, 255, 0]);
    // 斜率大于1
    draw_line((10., 10.).into(), (60., 90.).into(), &mut img, [0, 255, 0]);
    draw_line((60., 90.).into(), (10., 10.).into(), &mut img, [0, 255, 0]);
    // 斜率小于0大于-1
    draw_line((10., 90.).into(), (90., 40.).into(), &mut img, [0, 255, 0]);
    draw_line((90., 40.).into(), (10., 90.).into(), &mut img, [0, 255, 0]);
    // 斜率小于-1
    draw_line((10., 90.).into(), (40., 10.).into(), &mut img, [0, 255, 0]);
    draw_line((40., 10.).into(), (10., 90.).into(), &mut img, [0, 255, 0]);
    // 斜率为0
    draw_line((10., 50.).into(), (90., 50.).into(), &mut img, [0, 55, 0]);
    // 斜率无穷大
    draw_line((50., 10.).into(), (50., 90.).into(), &mut img, [0, 55, 0]);
    img.save("screenshots/bresenham_line.png").unwrap();
}
