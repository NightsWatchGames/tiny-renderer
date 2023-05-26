use std::default;

use crate::{
    camera::Camera,
    math::{Mat4, Vec2, Vec3},
    model::{Mesh, Vertex},
};

//// 视口
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    // 视口左上角的坐标
    pub x: i32,
    pub y: i32,
    // 视口的宽高
    pub width: u32,
    pub height: u32,
}
impl Viewport {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Color {
    pub const BLACK: Self = Self::new(0, 0, 0);
    pub const RED: Self = Self::new(255, 0, 0);
    pub const GREEN: Self = Self::new(0, 255, 0);
    pub const BLUE: Self = Self::new(0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255);

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct RendererSettings {
    pub projection: Projection,
    pub wireframe: bool,
}
#[derive(Debug, Clone, Copy, Default)]
pub enum Projection {
    #[default]
    Perspective,
    Orthographic,
}

pub struct Renderer {
    pub camera: Camera,
    pub viewport: Viewport,
    pub settings: RendererSettings,
    // 帧缓冲
    pub frame_buffer: Vec<u8>,
    // 深度缓冲
    pub depth_buffer: Vec<f32>,
}
impl Renderer {
    pub fn new(camera: Camera, viewport: Viewport, settings: RendererSettings) -> Self {
        let pixel_count = (viewport.width * viewport.height) as usize;
        Self {
            camera,
            viewport,
            settings,
            frame_buffer: vec![0; pixel_count * 3],
            depth_buffer: vec![std::f32::MAX; pixel_count],
        }
    }
    pub fn draw(&mut self, meshes: &Vec<Mesh>, model_transformation: Mat4) {
        for mesh in meshes {
            for i in 0..mesh.vertices.len() / 3 {
                let vertices = [
                    mesh.vertices[i * 3],
                    mesh.vertices[1 + i * 3],
                    mesh.vertices[2 + i * 3],
                ];
                self.rasterize_trianlge(model_transformation, vertices);
            }
        }
    }
    pub fn rasterize_trianlge(&mut self, model_transformation: Mat4, mut vertices: [Vertex; 3]) {
        for vertex in vertices.iter() {
            println!("before model trans, pos: {:?}", vertex.position);
        }
        // 模型变换
        for vertex in vertices.iter_mut() {
            vertex.position =
                (model_transformation * vertex.position.extend(1.0)).to_cartesian_point();
        }
        for vertex in vertices.iter() {
            // println!("after model trans, pos: {:?}", vertex.position);
        }

        // 视图变换
        let view_transformation = self.camera.view_transformation();
        for vertex in vertices.iter_mut() {
            vertex.position =
                (view_transformation * vertex.position.extend(1.0)).to_cartesian_point();
        }
        for vertex in vertices.iter() {
            println!("after view trans, pos: {:?}", vertex.position);
        }

        // TODO 视椎体裁剪

        // 投影变换
        let projection_transformation = match self.settings.projection {
            Projection::Perspective => self.camera.frustum.persp_projection_transformation(),
            Projection::Orthographic => self.camera.frustum.ortho_projection_transformation(),
        };
        for vertex in vertices.iter_mut() {
            vertex.position =
                (projection_transformation * vertex.position.extend(1.0)).to_cartesian_point();
        }
        for vertex in vertices.iter() {
            println!("after proj trans, pos: {:?}", vertex.position);
            assert!(vertex.position.x.abs() <= 1.0);
            assert!(vertex.position.y.abs() <= 1.0);
            assert!(vertex.position.z.abs() <= 1.0);
        }

        // 视口变换
        // TODO 矩阵
        for vertex in vertices.iter_mut() {
            vertex.position.x = (vertex.position.x + 1.0) * (self.viewport.width as f32 - 1.0)
                / 2.0
                + self.viewport.x as f32;
            vertex.position.y = (vertex.position.y + 1.0) * (self.viewport.height as f32 - 1.0)
                / 2.0
                + self.viewport.y as f32;
        }
        for vertex in vertices.iter() {
            println!(
                "draw_line phase, x: {}, y: {}",
                vertex.position.x, vertex.position.y
            );
        }
        if self.settings.wireframe {
            self.draw_line(
                Vec2::new(vertices[0].position.x, vertices[0].position.y),
                Vec2::new(vertices[1].position.x, vertices[1].position.y),
                Color::WHITE,
            );
            self.draw_line(
                Vec2::new(vertices[1].position.x, vertices[1].position.y),
                Vec2::new(vertices[2].position.x, vertices[2].position.y),
                Color::WHITE,
            );
            self.draw_line(
                Vec2::new(vertices[2].position.x, vertices[2].position.y),
                Vec2::new(vertices[0].position.x, vertices[0].position.y),
                Color::WHITE,
            );
        } else {
            todo!("draw triangle")
        }
    }

    // 绘制像素点
    pub fn draw_pixel(&mut self, p0: Vec2, color: Color) {
        let x = p0.x as i32;
        let y = p0.y as i32;
        if x < self.viewport.x
            || x >= self.viewport.x + self.viewport.width as i32
            || y < self.viewport.y
            || y >= self.viewport.y + self.viewport.height as i32
        {
            println!("pixel out of viewport");
            return;
        }
        // 以viewport左下角为原点
        let (x, y) = (x - self.viewport.x, y - self.viewport.y);
        let index = (y * self.viewport.width as i32 + x) as usize;
        self.frame_buffer[index * 3] = color.r;
        self.frame_buffer[index * 3 + 1] = color.g;
        self.frame_buffer[index * 3 + 2] = color.b;
    }

    // Bresenham画线算法
    pub fn draw_line(&mut self, p0: Vec2, p1: Vec2, color: Color) {
        // 线段裁剪
        let clip_result = line_clip(
            p0,
            p1,
            Vec2::ZERO,
            Vec2::new(self.viewport.width as f32, self.viewport.height as f32),
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
                self.draw_pixel(Vec2::new(x0 as f32, y as f32), color);
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
                self.draw_pixel(Vec2::new(x as f32, y0 as f32), color);
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
            self.draw_pixel(Vec2::new(y0 as f32, x0 as f32), color);
        } else if flag1 {
            self.draw_pixel(Vec2::new(y0 as f32, -x0 as f32), color);
        } else if flag2 {
            self.draw_pixel(Vec2::new(x0 as f32, -y0 as f32), color);
        } else {
            self.draw_pixel(Vec2::new(x0 as f32, y0 as f32), color);
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
                self.draw_pixel(Vec2::new(y as f32, x as f32), color);
            } else if flag1 {
                self.draw_pixel(Vec2::new(y as f32, -x as f32), color);
            } else if flag2 {
                self.draw_pixel(Vec2::new(x as f32, -y as f32), color);
            } else {
                self.draw_pixel(Vec2::new(x as f32, y as f32), color);
            }
        }
    }

    pub fn clear(&mut self) {
        self.frame_buffer.fill(0);
        self.depth_buffer.fill(f32::MAX);
    }
}

// 重心坐标
pub fn barycentric_2d(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> (f32, f32, f32) {
    barycentric_3d(p.extend(0.0), a.extend(0.0), b.extend(0.0), c.extend(0.0))
}
pub fn barycentric_3d(p: Vec3, a: Vec3, b: Vec3, c: Vec3) -> (f32, f32, f32) {
    let ab = b - a;
    let ac = c - a;
    let ap = p - a;
    let area_2abc = ab.cross(ac).length();
    let area_2pab = ab.cross(ap).length();
    let area_2pca = ac.cross(ap).length();
    let area_2pbc = area_2abc - area_2pab - area_2pca;
    let alpha = area_2pbc / area_2abc;
    let beta = area_2pca / area_2abc;
    let gamma = 1.0 - alpha - beta;
    (alpha, beta, gamma)
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
