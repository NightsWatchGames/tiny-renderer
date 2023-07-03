use fltk::{
    app::{event_key_down, set_visual},
    enums::{Key, Mode},
    prelude::{DisplayExt, GroupExt, WidgetBase, WidgetExt},
    text,
    window::Window,
};
use tiny_renderer::{
    camera::Camera,
    color::Color,
    light::PointLight,
    loader::load_glft,
    math::{Quat, Vec3},
    renderer::{Renderer, RendererSettings, Viewport},
    shader::phong_shader,
    transform::translation_mat4,
    util::{custom_cube, flip_vertically, rand_color},
};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 720;

pub fn main() {
    println!(
        "
        F1: toggle wireframe rendering
        F2: toggle vertex color interpolation
        F3: toggle fragment shading
        F4: toggle projection
        W/A/S/D/Q/E: move camera
    "
    );
    let app = fltk::app::App::default();
    let mut wind = Window::new(
        100,
        100,
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32,
        "rendering",
    );

    // let (meshes, texture_storage) = load_glft("assets/cube/cube.gltf");
    // let (meshes, texture_storage) = load_glft("assets/monkey/monkey.gltf");
    let (meshes, texture_storage) = load_glft("assets/box-textured/BoxTextured.gltf");
    // let (meshes, texture_storage) = load_glft("assets/sphere/sphere.gltf");
    // let (meshes, texture_storage) = load_glft("assets/suzanne/Suzanne.gltf");
    // let (meshes, texture_storage) = load_glft("assets/cornell-box.gltf");
    // let (meshes, texture_storage) = custom_cube();
    let model_pos = Vec3::new(0.0, 0.0, 0.0);
    let model_transformation = translation_mat4(model_pos);

    let light = PointLight {
        position: Vec3::new(-5.0, 5.0, 5.0),
        intensity: 100.0,
    };

    let mut camera = Camera::new(
        5.0,
        1000.0,
        WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
        60.0f32.to_radians(),
        Vec3::new(1.0, 2.0, 3.0),
    );
    camera.look_at(model_pos, Vec3::Y);

    let viewport = Viewport::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let settings = RendererSettings {
        wireframe: false,
        vertex_color_interp: false,
        fragment_shading: true,
        ..Default::default()
    };
    let mut renderer = Renderer::new(camera, viewport, settings);
    renderer.vertex_shader = Some(Box::new(|vertex| {}));
    renderer.fragment_shader = Some(phong_shader());

    wind.draw(move |_| {
        if event_key_down(Key::F1) {
            renderer.settings.wireframe = !renderer.settings.wireframe;
        }
        if event_key_down(Key::F2) {
            renderer.settings.vertex_color_interp = !renderer.settings.vertex_color_interp;
        }
        if event_key_down(Key::F3) {
            renderer.settings.fragment_shading = !renderer.settings.fragment_shading;
        }
        if event_key_down(Key::F4) {
            renderer.settings.projection = match renderer.settings.projection {
                tiny_renderer::renderer::Projection::Perspective => {
                    tiny_renderer::renderer::Projection::Orthographic
                }
                tiny_renderer::renderer::Projection::Orthographic => {
                    tiny_renderer::renderer::Projection::Perspective
                }
            };
        }
        if event_key_down(Key::from_char('A')) {
            renderer
                .camera
                .rotate_around(model_pos, Quat::from_axis_angle(Vec3::Y, -0.1))
        }
        if event_key_down(Key::from_char('D')) {
            renderer
                .camera
                .rotate_around(model_pos, Quat::from_axis_angle(Vec3::Y, 0.1))
        }
        if event_key_down(Key::from_char('W')) {
            renderer
                .camera
                .rotate_around(model_pos, Quat::from_axis_angle(Vec3::X, 0.1))
        }
        if event_key_down(Key::from_char('S')) {
            renderer
                .camera
                .rotate_around(model_pos, Quat::from_axis_angle(Vec3::X, -0.1))
        }
        if event_key_down(Key::from_char('Q')) {
            renderer
                .camera
                .rotate_around(model_pos, Quat::from_axis_angle(Vec3::Z, 0.1))
        }
        if event_key_down(Key::from_char('E')) {
            renderer
                .camera
                .rotate_around(model_pos, Quat::from_axis_angle(Vec3::Z, -0.1))
        }

        renderer.clear();
        renderer.draw(&meshes, model_transformation, light, &texture_storage);
        fltk::draw::draw_image(
            &flip_vertically(
                &renderer.frame_buffer,
                WINDOW_WIDTH as usize,
                WINDOW_HEIGHT as usize,
            ),
            0,
            0,
            WINDOW_WIDTH as i32,
            WINDOW_HEIGHT as i32,
            fltk::enums::ColorDepth::Rgb8,
        )
        .unwrap();
    });

    wind.end();
    set_visual(Mode::Rgb).unwrap();
    wind.show();

    fltk::app::add_idle3(move |_| {
        wind.redraw();
    });

    app.run().unwrap();
}
