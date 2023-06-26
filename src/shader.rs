use std::collections::HashMap;

use crate::{
    color::Color,
    light::PointLight,
    material::Material,
    math::{Mat4, Vec2, Vec3, Vec4},
    mesh::Vertex,
    texture::TextureStorage,
};

const AMBIENT_LIGHT_INTENSITY: f32 = 2.0;

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
        let texcolor = texture_storage
            .texture_id_map
            .get(&0)
            .map(|texture| texture.sample(texcoord));

        // 漫反射系数
        let kd = if let Some(texcolor) = texcolor {
            texcolor.to_vec3()
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
        let ambient = material.ambient * AMBIENT_LIGHT_INTENSITY;
        // println!("ambient: {:?}", ambient);
        // 漫反射
        let diffuse = kd * (light.intensity / (r * r)) * n.dot(l).max(0.0);
        if diffuse.x < 0.00001 {
            // println!("diffuse: {:?}", diffuse);
        }
        // println!("diffuse: {:?}", diffuse);
        // 镜面反射
        let specular = material.specular
            * (light.intensity / (r * r))
            * (n.dot(h).max(0.0).powf(material.shininess));
        // println!("specular: {:?}", specular);

        // let mut result = ambient + diffuse + specular;
        let mut result = if let Some(texcolor) = texcolor {
            (Color::from_vec3(ambient) * texcolor).to_vec3() + diffuse
        } else {
            ambient + diffuse
        };
        // println!("result: {:?}", result);
        result.x = result.x.clamp(0.0, 1.0);
        result.y = result.y.clamp(0.0, 1.0);
        result.z = result.z.clamp(0.0, 1.0);
        Color::from_vec3(result)
    })
}
