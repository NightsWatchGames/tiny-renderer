use std::collections::HashMap;

use crate::{
    color::Color,
    light::PointLight,
    material::Material,
    math::{Mat4, Vec2, Vec3, Vec4},
    mesh::Vertex,
    texture::TextureStorage,
};

const AMBIENT_LIGHT_INTENSITY: f32 = 0.2;

// TODO 使用引用+生命周期
#[derive(Debug, Clone, Default)]
pub struct FragmentShaderPayload {
    pub triangle: [Vertex; 3],
    pub world_positions: [Vec3; 3],
    pub view_space_positions: [Vec3; 3],
    pub barycenter: (f32, f32, f32),
    pub light: PointLight,
    pub camera_world_position: Vec3,
    pub material: Material,
}

pub type VertexShader = Box<dyn Fn(&mut Vertex)>;
pub type FragmentShader = Box<dyn Fn(&FragmentShaderPayload, &TextureStorage) -> Color>;

#[derive(Debug, Default)]
pub struct Uniforms {
    pub int: HashMap<u32, i32>,
    pub float: HashMap<u32, f32>,
    pub vec2: HashMap<u32, Vec2>,
    pub vec3: HashMap<u32, Vec3>,
    pub vec4: HashMap<u32, Vec4>,
    pub mat4: HashMap<u32, Mat4>,
    pub texture: HashMap<u32, u32>,
}

pub fn phong_shader() -> FragmentShader {
    Box::new(|payload, texture_storage| {
        let world_positions = payload.world_positions;
        let camera_world_position = payload.camera_world_position;
        let triangle = payload.triangle;
        let (alpha, beta, gamma) = payload.barycenter;
        let light = payload.light;
        let material = payload.material;

        // 着色点
        let pos =
            world_positions[0] * alpha + world_positions[1] * beta + world_positions[2] * gamma;
        let texcoord = triangle[0].texcoord.unwrap() * alpha
            + triangle[1].texcoord.unwrap() * beta
            + triangle[2].texcoord.unwrap() * gamma;
        let texcolor = texture_storage
            .texture_id_map
            .get(&0)
            .map(|texture| texture.sample(texcoord));

        // TODO 处理unwrap / 使用宏简化
        // 法线
        let n = (triangle[0].normal.unwrap() * alpha
            + triangle[1].normal.unwrap() * beta
            + triangle[2].normal.unwrap() * gamma)
            .normalize();
        // 入射光线向量
        let l = (light.position - pos).normalize();
        // 视线向量
        let v = (camera_world_position - pos).normalize();
        // 半程向量
        let h = (l + v).normalize();
        // 入射光线距离
        let r = (light.position - pos).length();

        // 环境光
        let ambient = material.ambient * AMBIENT_LIGHT_INTENSITY;
        // 漫反射
        let diffuse = material.diffuse * (light.intensity / (r * r)) * n.dot(l).max(0.0);
        // 镜面反射
        let specular = material.specular
            * (light.intensity / (r * r))
            * (n.dot(h).max(0.0).powf(material.shininess));

        let light = ambient + diffuse + specular;
        let (mut r, mut g, mut b) = if let Some(texcolor) = texcolor {
            (
                light.x * texcolor.r,
                light.y * texcolor.g,
                light.z * texcolor.b,
            )
        } else {
            (light.x, light.y, light.z)
        };
        r = r.clamp(0.0, 1.0);
        g = g.clamp(0.0, 1.0);
        b = b.clamp(0.0, 1.0);
        Color::new(r, g, b)
    })
}
