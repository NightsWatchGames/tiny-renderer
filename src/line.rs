use image::RgbImage;

use crate::math::Vec2;

// Bresenham画线算法
pub fn draw_line(p0: Vec2, p1: Vec2, img: &mut RgbImage, color: [u8; 3]) {
    // 线段裁剪
    let clip_result = line_clip(
        p0,
        p1,
        Vec2::ZERO,
        (img.width() as f32 - 1.0, img.height() as f32 - 1.0).into(),
    );
    if clip_result.is_none() {
        return;
    }
    let (p0, p1) = clip_result.unwrap();
    let mut x0 = p0.x as i32;
    let mut y0 = p0.y as i32;
    let mut x1 = p1.x as i32;
    let mut y1 = p1.y as i32;

    // 斜率无穷大
    if x0 == x1 {
        let mut y = y0;
        loop {
            img.put_pixel(x0 as u32, y as u32, image::Rgb(color));
            if y == y1 {
                break;
            }
            y += if y1 > y0 { 1 } else { -1 };
        }
        return;
    }
    // 斜率为0
    if y0 == y1 {
        let mut x = x0;
        loop {
            img.put_pixel(x as u32, y0 as u32, image::Rgb(color));
            if x == x1 {
                break;
            }
            x += if x1 > x0 { 1 } else { -1 };
        }
        return;
    }

    // 交换起始点和终点，使得永远保持从左到右画线的顺序(x1 - x0 > 0)，不影响线段，此时线段只会在一四象限
    if x0 > x1 {
        let tmp_x = x0;
        let tmp_y = y0;
        x0 = x1;
        y0 = y1;
        x1 = tmp_x;
        y1 = tmp_y;
    }

    // 沿y=x对称
    let mut flag0 = false;
    // 沿x轴对称，再沿y=x对称
    let mut flag1 = false;
    // 沿x轴对称
    let mut flag2 = false;

    if y1 - y0 > x1 - x0 {
        // 斜率大于1，沿y=x对称（交换x和y）
        let temp = x0;
        x0 = y0;
        y0 = temp;
        let temp = x1;
        x1 = y1;
        y1 = temp;
        flag0 = true;
    } else if y1 < y0 {
        // 斜率小于0

        // 沿x轴对称
        y0 = -y0;
        y1 = -y1;

        if y1 - y0 > x1 - x0 {
            // 沿x轴对称后斜率大于1，则再沿y=x对称（交换x和y）
            let temp = x0;
            x0 = y0;
            y0 = temp;
            let temp = x1;
            x1 = y1;
            y1 = temp;
            flag1 = true;
        } else {
            // 沿x轴对称后斜率小于1
            flag2 = true;
        }
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let incr_n = 2 * dy;
    let incr_ne = 2 * (dy - dx);
    let mut d = 2 * dy - dx;

    if flag0 {
        img.put_pixel(y0 as u32, x0 as u32, image::Rgb(color));
    } else if flag1 {
        img.put_pixel(y0 as u32, -x0 as u32, image::Rgb(color));
    } else if flag2 {
        img.put_pixel(x0 as u32, -y0 as u32, image::Rgb(color));
    } else {
        img.put_pixel(x0 as u32, y0 as u32, image::Rgb(color));
    }

    let mut y = y0;
    for x in x0 + 1..x1 {
        if d < 0 {
            d += incr_n;
        } else {
            y += 1;
            d += incr_ne;
        }
        if flag0 {
            img.put_pixel(y as u32, x as u32, image::Rgb(color));
        } else if flag1 {
            img.put_pixel(y as u32, -x as u32, image::Rgb(color));
        } else if flag2 {
            img.put_pixel(x as u32, -y as u32, image::Rgb(color));
        } else {
            img.put_pixel(x as u32, y as u32, image::Rgb(color));
        }
    }
}

// Cohen-Sutherland线段裁剪算法
const INSIDE: u8 = 0; // 0000
const LEFT: u8 = 1; // 0001
const RIGHT: u8 = 2; // 0010
const BOTTOM: u8 = 4; // 0100
const TOP: u8 = 8; // 1000
fn compute_out_code(p: &Vec2, min: &Vec2, max: &Vec2) -> u8 {
    let horizontal_code = if p.x < min.x {
        LEFT
    } else if p.x > max.x {
        RIGHT
    } else {
        INSIDE
    };
    let vertical_code = if p.y < min.y {
        BOTTOM
    } else if p.y > max.y {
        TOP
    } else {
        INSIDE
    };
    horizontal_code | vertical_code
}
pub fn line_clip(
    mut p0: Vec2,
    mut p1: Vec2,
    rect_min: Vec2,
    rect_max: Vec2,
) -> Option<(Vec2, Vec2)> {
    let mut out_code0 = compute_out_code(&p0, &rect_min, &rect_max);
    let mut out_code1 = compute_out_code(&p1, &rect_min, &rect_max);

    loop {
        if out_code0 & out_code1 != 0 {
            // 两个点在inside外面的同一侧
            return None;
        } else if out_code0 | out_code1 == 0 {
            // 两个点都在inside内
            return Some((p0, p1));
        }

        // 至少有一个outcode在inside外面
        let out_code = if out_code0 > out_code1 {
            out_code0
        } else {
            out_code1
        };

        // 找到与矩形相交的边界
        let mut p = Vec2::ZERO;
        if out_code & TOP != 0 {
            p.x = p0.x + (p1.x - p0.x) * (rect_max.y - p0.y) / (p1.y - p0.y);
            p.y = rect_max.y;
        } else if out_code & BOTTOM != 0 {
            p.x = p0.x + (p0.x - p0.x) * (rect_min.y - p0.y) / (p1.y - p0.y);
            p.y = rect_min.y;
        } else if out_code & RIGHT != 0 {
            p.x = rect_max.x;
            p.y = p0.y + (p1.y - p0.y) * (rect_max.x - p0.x) / (p1.x - p0.x);
        } else if out_code & LEFT != 0 {
            p.x = rect_min.x;
            p.y = p0.y + (p1.y - p0.y) * (rect_min.x - p0.x) / (p1.x - p0.x);
        }

        // 用相交的边界点替换原来的点
        if out_code == out_code0 {
            p0.x = p.x;
            p0.y = p.y;
            out_code0 = compute_out_code(&p0, &rect_min, &rect_max);
        } else {
            p1.x = p.x;
            p1.y = p.y;
            out_code1 = compute_out_code(&p1, &rect_min, &rect_max);
        }
    }
}
