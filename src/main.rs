use image::{ImageBuffer, RgbImage};

fn main() {
    let mut img: RgbImage = ImageBuffer::new(100, 100);
    // 斜率大于0小于1
    line((10, 10), (90, 60), &mut img, [0, 255, 0]);
    line((90, 60), (10, 10), &mut img, [0, 255, 0]);
    // 斜率大于1
    line((10, 10), (60, 90), &mut img, [0, 255, 0]);
    line((60, 90), (10, 10), &mut img, [0, 255, 0]);
    // 斜率小于0大于-1
    line((10, 90), (90, 40), &mut img, [0, 255, 0]);
    line((90, 40), (10, 90), &mut img, [0, 255, 0]);
    // 斜率小于-1
    line((10, 90), (40, 10), &mut img, [0, 255, 0]);
    line((40, 10), (10, 90), &mut img, [0, 255, 0]);
    // 斜率为0
    line((10, 50), (90, 50), &mut img, [0, 55, 0]);
    // 斜率无穷大
    line((50, 10), (50, 90), &mut img, [0, 55, 0]);
    img.save("test.png").unwrap();
}

// Bresenham画线算法
fn line(p0: (i32, i32), p1: (i32, i32), img: &mut RgbImage, color: [u8; 3]) {
    let (mut x0, mut y0) = p0;
    let (mut x1, mut y1) = p1;

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