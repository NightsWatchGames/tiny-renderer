use std::collections::HashMap;

use crate::{
    color::Color,
    light::PointLight,
    material::Material,
    math::{Mat4, Vec2, Vec3, Vec4},
    mesh::Vertex,
    texture::TextureStorage,
};

// TODO 使用引用+生命周期
#[derive(Debug, Clone, Default)]
pub struct FragmentShaderPayload {
    pub ori_triangle: [Vertex; 3],
    pub triangle: [Vertex; 3],
    pub barycenter: Vec3,
    pub light: PointLight,
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
        let ori_triangle = payload.ori_triangle;
        let barycenter = payload.barycenter;
        let light = payload.light;
        let material = payload.material;
        // 着色点
        let pos = ori_triangle[0].position * barycenter.x
            + ori_triangle[1].position * barycenter.y
            + ori_triangle[2].position * barycenter.z;
        let texcoord = ori_triangle[0].texcoord.unwrap() * barycenter.x
            + ori_triangle[1].texcoord.unwrap() * barycenter.y
            + ori_triangle[2].texcoord.unwrap() * barycenter.z;

        // 漫反射系数
        let kd = if let Some(texture) = texture_storage.texture_id_map.get(&0) {
            texture.sample(texcoord).to_vec3().extend(1.0)
        } else {
            material.diffuse
        };

        // TODO 处理unwrap / 使用宏简化
        // 法线
        let n = (ori_triangle[0].normal.unwrap() * barycenter.x
            + ori_triangle[1].normal.unwrap() * barycenter.y
            + ori_triangle[2].normal.unwrap() * barycenter.z)
            .normalize();
        // 入射光线向量
        let l = (light.position - pos).normalize();
        // 视线向量
        let v = (Vec3::ZERO - pos).normalize();
        // 半程向量
        let h = (l + v).normalize();
        // 入射光线距离
        let r = (light.position - pos).length();

        // 环境光
        let ambient = material.ambient * light.intensity;
        // 漫反射
        let diffuse = kd * (light.intensity / (r * r)) * n.dot(l).max(0.0);
        // 镜面反射
        let specular = material.specular
            * (light.intensity / (r * r))
            * (n.dot(h).max(0.0).powf(material.shininess));

        Color::from_vec3((ambient + diffuse + specular).truncate())
    })
}
