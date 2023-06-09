use fltk::{
    app::set_visual,
    enums::Mode,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window,
};
use rand::Rng;
use tiny_renderer::{
    camera::Camera,
    color::Color,
    math::{Vec2, Vec3},
    renderer::{Renderer, RendererSettings, Viewport},
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
        "bresenham line",
    );

    let camera = Camera::new(
        -1.0,
        -1000.0,
        WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
        30.0f32.to_radians(),
        Vec3::ZERO,
    );
    let viewport = Viewport::new(0, 0, WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut renderer = Renderer::new(camera, viewport, RendererSettings::default());
    wind.draw(move |_| {
        renderer.clear();
        let mut rng = rand::thread_rng();
        renderer.draw_line(
            Vec2::new(10.0, WINDOW_HEIGHT as f32 / 2.0),
            Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32 / 2.0),
            Color::GREEN,
        );
        renderer.draw_line(
            Vec2::new(WINDOW_WIDTH as f32 / 2.0, 10.0),
            Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32),
            Color::GREEN,
        );
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
