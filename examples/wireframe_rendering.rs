use fltk::{
    app::set_visual,
    enums::Mode,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};
use tiny_renderer::{
    camera::Camera,
    math::Vec3,
    model::{custom_cube, custom_mesh, load_glft},
    renderer::{Color, Renderer, Viewport},
    transform::translation_mat4,
};

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 720;

pub fn main() {
    let app = fltk::app::App::default();
    let mut wind = Window::new(
        100,
        100,
        WINDOW_WIDTH as i32,
        WINDOW_HEIGHT as i32,
        "wireframe rendering",
    );

    // let meshes = load_glft("assets/cube/cube.gltf");
    let meshes = vec![custom_cube()];
    let mesh_pos = Vec3::new(0.0, 0.0, -10.0);
    let model_transformation = translation_mat4(mesh_pos);

    let mut camera = Camera::new(
        -5.0,
        -1000.0,
        WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
        60.0f32.to_radians(),
        // Vec3::ZERO
        Vec3::new(3.0, 3.0, 0.0),
    );
    camera.look_at(mesh_pos, Vec3::Y);

    let viewport = Viewport::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut renderer = Renderer::new(camera, viewport);

    wind.draw(move |_| {
        renderer.clear();
        renderer.draw(&meshes, model_transformation);
        fltk::draw::draw_image(
            &renderer.frame_buffer,
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
    app.run().unwrap();
}
