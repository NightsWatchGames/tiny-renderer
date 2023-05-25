use fltk::{
    app::set_visual,
    enums::Mode,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};
use tiny_renderer::{
    camera::Camera,
    model::load_glft,
    renderer::{Color, Renderer, Viewport},
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
        "bresenham line",
    );

    let meshes = load_glft("assets/sphere/sphere.gltf");
    let camera = Camera::new(
        -1.0,
        -1000.0,
        WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
        30.0f32.to_radians(),
    );
    let viewport = Viewport::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut renderer = Renderer::new(camera, viewport);
    wind.draw(move |_| {
        renderer.clear();
        for mesh in meshes.clone() {
            let mut i = 0;
            loop {
                if i > mesh.vertices.len() - 3 {
                    break;
                }
                // TODO 改成 draw_triangle
                renderer.draw_line(
                    (
                        (mesh.vertices[i].position.x + 1.0) * WINDOW_WIDTH as f32 / 3.0,
                        (mesh.vertices[i].position.y + 1.0) * WINDOW_HEIGHT as f32 / 3.0,
                    )
                        .into(),
                    (
                        (mesh.vertices[i + 1].position.x + 1.0) * WINDOW_WIDTH as f32 / 3.0,
                        (mesh.vertices[i + 1].position.y + 1.0) * WINDOW_HEIGHT as f32 / 3.0,
                    )
                        .into(),
                    Color::GREEN,
                );
                renderer.draw_line(
                    (
                        (mesh.vertices[i + 1].position.x + 1.0) * WINDOW_WIDTH as f32 / 3.0,
                        (mesh.vertices[i + 1].position.y + 1.0) * WINDOW_HEIGHT as f32 / 3.0,
                    )
                        .into(),
                    (
                        (mesh.vertices[i + 2].position.x + 1.0) * WINDOW_WIDTH as f32 / 3.0,
                        (mesh.vertices[i + 2].position.y + 1.0) * WINDOW_HEIGHT as f32 / 3.0,
                    )
                        .into(),
                    Color::GREEN,
                );
                renderer.draw_line(
                    (
                        (mesh.vertices[i + 2].position.x + 1.0) * WINDOW_WIDTH as f32 / 3.0,
                        (mesh.vertices[i + 2].position.y + 1.0) * WINDOW_HEIGHT as f32 / 3.0,
                    )
                        .into(),
                    (
                        (mesh.vertices[i].position.x + 1.0) * WINDOW_WIDTH as f32 / 3.0,
                        (mesh.vertices[i].position.y + 1.0) * WINDOW_HEIGHT as f32 / 3.0,
                    )
                        .into(),
                    Color::GREEN,
                );

                i += 3;
            }
        }
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
