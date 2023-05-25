use fltk::{
    app::set_visual,
    enums::Mode,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};
use rand::Rng;
use tiny_renderer::{
    camera::Camera,
    math::Vec2,
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
        let mut rng = rand::thread_rng();
        for _ in 0..50 {
            renderer.draw_line(
                Vec2::new(
                    rng.gen_range(0.0..WINDOW_WIDTH as f32),
                    rng.gen_range(0.0..WINDOW_WIDTH as f32),
                ),
                Vec2::new(
                    rng.gen_range(0.0..WINDOW_WIDTH as f32),
                    rng.gen_range(0.0..WINDOW_WIDTH as f32),
                ),
                Color::GREEN,
            );
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
