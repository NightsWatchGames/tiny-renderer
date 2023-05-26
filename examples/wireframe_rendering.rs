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
    renderer::{Color, Renderer, RendererSettings, Viewport},
    transform::translation_mat4,
    util::flip_vertically,
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
    let mesh_pos = Vec3::new(0.0, 0.0, 0.0);
    let model_transformation = translation_mat4(mesh_pos);

    let mut camera = Camera::new(
        -5.0,
        -1000.0,
        WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
        60.0f32.to_radians(),
        // Vec3::ZERO
        Vec3::new(3.0, 4.0, 5.0),
        // Vec3::new(5.0, 8.0, 0.0),
    );
    camera.look_at(mesh_pos, Vec3::Y);

    let viewport = Viewport::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let settings = RendererSettings {
        wireframe: true,
        ..Default::default()
    };
    let mut renderer = Renderer::new(camera, viewport, settings);

    wind.draw(move |_| {
        renderer.clear();
        renderer.draw(&meshes, model_transformation);
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
    app.run().unwrap();
}
