use crate::{
    camera::Camera,
    color::Color,
    light::PointLight,
    material::Material,
    math::{Mat4, Vec2, Vec3},
    mesh::{Mesh, Vertex},
    shader::{FragmentShader, FragmentShaderPayload, VertexShader},
    texture::TextureStorage,
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

#[derive(Debug, Clone, Copy, Default)]
pub struct RendererSettings {
    pub projection: Projection,
    // 是否绘制线框
    pub wireframe: bool,
    // 是否根据顶点颜色插值填充
    pub vertex_color_interp: bool,
    // 是否采用片段着色
    pub fragment_shading: bool,
}
#[derive(Debug, Clone, Copy, Default)]
pub enum Projection {
    #[default]
    Perspective,
    Orthographic,
}

#[derive(Debug, Clone, Copy)]
pub struct Aabb2d {
    pub min: Vec2,
    pub max: Vec2,
}
impl Aabb2d {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self { min, max }
    }
}

pub struct Renderer {
    pub camera: Camera,
    pub viewport: Viewport,
    pub settings: RendererSettings,
    pub vertex_shader: Option<VertexShader>,
    pub fragment_shader: Option<FragmentShader>,
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
            vertex_shader: None,
            fragment_shader: None,
            frame_buffer: vec![0; pixel_count * 3],
            depth_buffer: vec![std::f32::MIN; pixel_count],
        }
    }

    pub fn draw(
        &mut self,
        meshes: &Vec<Mesh>,
        model_transformation: Mat4,
        light: PointLight,
        texture_storage: &TextureStorage,
    ) {
        for mesh in meshes.iter() {
            for i in 0..mesh.vertices.len() / 3 {
                let mut triangle = [
                    mesh.vertices[i * 3],
                    mesh.vertices[1 + i * 3],
                    mesh.vertices[2 + i * 3],
                ];

                let world_positions: [Vec3; 3] = triangle.map(|v| v.position.to_cartesian_point());

                // 顶点着色
                self.vertex_shading(&mut triangle);

                // 模型变换
                self.apply_model_transformation(&mut triangle, model_transformation);

                // 视图变换
                self.apply_view_transformation(&mut triangle);

                // 背面剔除
                if Self::back_face_cull(
                    triangle.map(|v| v.position.to_cartesian_point()),
                    Vec3::NEG_Z,
                ) {
                    continue;
                }

                // 保存视图空间坐标
                let view_space_positions: [Vec3; 3] =
                    triangle.map(|v| v.position.to_cartesian_point());

                // 投影变换
                self.apply_projection_transformation(&mut triangle);

                // 透视（齐次）除法
                Self::homogeneous_division(&mut triangle);

                // TODO 视椎体裁剪

                // 视口变换
                self.apply_viewport_transformation(&mut triangle);

                // 线框渲染
                if self.settings.wireframe {
                    self.draw_wireframe(&triangle, Color::WHITE);
                }

                // 光栅化
                self.rasterize_trianlge(
                    world_positions,
                    view_space_positions,
                    triangle,
                    &mesh.material,
                    &light,
                    texture_storage,
                );
            }
        }
    }

    pub fn rasterize_trianlge(
        &mut self,
        world_positions: [Vec3; 3],
        view_space_positions: [Vec3; 3],
        triangle: [Vertex; 3],
        material: &Material,
        light: &PointLight,
        texture_storage: &TextureStorage,
    ) {
        // 包围盒
        let aabb2d = bounding_box2d(&triangle.map(|v| Vec2::new(v.position.x, v.position.y)));

        // 光栅化
        for x in aabb2d.min.x as u32..=aabb2d.max.x as u32 {
            for y in aabb2d.min.y as u32..=aabb2d.max.y as u32 {
                // 计算屏幕三角形重心坐标
                let p = Vec2::new(x as f32, y as f32);
                let barycenter = barycentric_2d_triangle(p, &triangle);

                // 判断是否在三角形内
                if Self::inside_triangle(barycenter) {
                    let z = Self::z_interpolation(&triangle, barycenter);
                    let index = (y * self.viewport.width + x) as usize;

                    // 深度测试
                    if z > self.depth_buffer[index] {
                        self.depth_buffer[index] = z;

                        // 透视矫正
                        let barycenter = Self::perspective_correct(&triangle, barycenter);

                        if self.settings.fragment_shading {
                            // 片段着色
                            if let Some(fragment_shader) = &self.fragment_shader {
                                let fragment_shader_payload = FragmentShaderPayload {
                                    triangle,
                                    world_positions,
                                    view_space_positions,
                                    barycenter,
                                    light: light.clone(),
                                    camera_world_position: self.camera.position,
                                    material: material.clone(),
                                    ..Default::default()
                                };
                                let color =
                                    fragment_shader(&fragment_shader_payload, texture_storage);
                                self.draw_pixel(p, color);
                            }
                        } else if self.settings.vertex_color_interp {
                            // 顶点颜色插值
                            if triangle[0].color.is_some()
                                && triangle[1].color.is_some()
                                && triangle[2].color.is_some()
                            {
                                let color = triangle[0].color.unwrap() * barycenter.0
                                    + triangle[1].color.unwrap() * barycenter.1
                                    + triangle[2].color.unwrap() * barycenter.2;
                                self.draw_pixel(p, color);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn vertex_shading(&self, vertices: &mut [Vertex]) {
        if let Some(vertex_shader) = &self.vertex_shader {
            for vertex in vertices.iter_mut() {
                vertex_shader(vertex);
            }
        }
    }

    pub fn apply_model_transformation(&self, vertices: &mut [Vertex], model_transformation: Mat4) {
        for vertex in vertices.iter_mut() {
            vertex.position = model_transformation * vertex.position;
        }
    }

    pub fn apply_view_transformation(&mut self, vertices: &mut [Vertex]) {
        let view_transformation = self.camera.view_transformation();
        for vertex in vertices.iter_mut() {
            vertex.position = view_transformation * vertex.position;
        }
    }

    pub fn apply_projection_transformation(&self, vertices: &mut [Vertex]) {
        let projection_transformation = match self.settings.projection {
            Projection::Perspective => self.camera.frustum.persp_projection_transformation(),
            Projection::Orthographic => self.camera.frustum.ortho_projection_transformation(),
        };
        for vertex in vertices.iter_mut() {
            vertex.position = projection_transformation * vertex.position;
        }
    }

    pub fn apply_viewport_transformation(&self, vertices: &mut [Vertex]) {
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
    }

    pub fn frustum_cull(&self, _triangle: [Vec3; 3]) -> bool {
        todo!()
    }

    pub fn homogeneous_clip() {
        todo!()
    }

    pub fn back_face_cull(triangle: [Vec3; 3], view_direction: Vec3) -> bool {
        // 默认三角形顶点顺序为逆时针
        let normal = (triangle[1] - triangle[0]).cross(triangle[2] - triangle[0]);
        normal.dot(view_direction) > 0.0
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
        self.frame_buffer[index * 3] = (color.r * 255.) as u8;
        self.frame_buffer[index * 3 + 1] = (color.g * 255.) as u8;
        self.frame_buffer[index * 3 + 2] = (color.b * 255.) as u8;
    }

    pub fn draw_wireframe(&mut self, vertices: &[Vertex], color: Color) {
        for i in 0..vertices.len() {
            let p0 = vertices[i].position;
            let p1 = vertices[(i + 1) % vertices.len()].position;
            self.draw_line(Vec2::new(p0.x, p0.y), Vec2::new(p1.x, p1.y), color);
        }
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

    pub fn homogeneous_division(vertexs: &mut [Vertex]) {
        vertexs.iter_mut().for_each(|v| {
            v.position.x /= v.position.w;
            v.position.y /= v.position.w;
            v.position.z /= v.position.w;
            v.position.w = 1.0;
        })
    }

    pub fn inside_triangle((alpha, beta, gamma): (f32, f32, f32)) -> bool {
        alpha > 0.0 && beta > 0.0 && gamma > 0.0
    }

    pub fn z_interpolation(triangle: &[Vertex; 3], (alpha, beta, gamma): (f32, f32, f32)) -> f32 {
        let v0 = triangle[0].position;
        let v1 = triangle[1].position;
        let v2 = triangle[2].position;
        let w_reciprocal = 1.0 / (alpha / v0.w + beta / v1.w + gamma / v2.w);
        let mut z_interpolated = alpha * v0.z / v0.w + beta * v1.z / v1.w + gamma * v2.z / v2.w;
        z_interpolated *= w_reciprocal;
        z_interpolated
    }

    // TODO 理解透视矫正
    pub fn perspective_correct(
        triangle: &[Vertex; 3],
        (alpha, beta, gamma): (f32, f32, f32),
    ) -> (f32, f32, f32) {
        let w0 = triangle[0].position.w.recip() * alpha;
        let w1 = triangle[1].position.w.recip() * beta;
        let w2 = triangle[2].position.w.recip() * gamma;
        let normalizer = 1.0 / (w0 + w1 + w2);
        (w0 * normalizer, w1 * normalizer, w2 * normalizer)
    }

    pub fn clear(&mut self) {
        self.frame_buffer.fill(0);
        self.depth_buffer.fill(f32::MIN);
    }
}

// 2D重心坐标
pub fn barycentric_2d_triangle(p: Vec2, triangle: &[Vertex; 3]) -> (f32, f32, f32) {
    barycentric_2d(
        p,
        Vec2::new(triangle[0].position.x, triangle[0].position.y),
        Vec2::new(triangle[1].position.x, triangle[1].position.y),
        Vec2::new(triangle[2].position.x, triangle[2].position.y),
    )
}
pub fn barycentric_2d(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> (f32, f32, f32) {
    let area_twice = (b - a).cross(c - a);
    let alpha = (b - p).cross(c - p) / area_twice;
    let beta = (c - p).cross(a - p) / area_twice;
    let gamma = (a - p).cross(b - p) / area_twice;
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

// 三角形包围盒
pub fn bounding_box2d(points: &[Vec2]) -> Aabb2d {
    let mut min = Vec2::new(f32::MAX, f32::MAX);
    let mut max = Vec2::new(f32::MIN, f32::MIN);
    for p in points {
        if p.x < min.x {
            min.x = p.x;
        }
        if p.y < min.y {
            min.y = p.y;
        }
        if p.x > max.x {
            max.x = p.x;
        }
        if p.y > max.y {
            max.y = p.y;
        }
    }
    Aabb2d::new(min, max)
}
