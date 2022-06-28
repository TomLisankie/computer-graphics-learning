extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;

const LENGTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn put_pixel(canvas: &mut Canvas<Window>, color: Color, position: Point) {
    // `position` is on a zero-centered coordinate system
    let new_x: i32 = ((LENGTH / 2) as i32) + position.x();
    let new_y: i32 = ((HEIGHT / 2) as i32) + position.y();
    canvas.set_draw_color(color);
    canvas.draw_point(Point::new(new_x, new_y)).unwrap()
}

pub fn draw_rectangle(
    canvas: &mut Canvas<Window>,
    color: Color,
    top_left: Point,
    length: i32,
    width: i32,
) {
    canvas.set_draw_color(color);
    for i in 0..width {
        for j in 0..length {
            put_pixel(
                canvas,
                color,
                Point::new(j + top_left.x(), i + top_left.y()),
            )
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", LENGTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i: u32 = 0;
    'running: loop {
        i += 1;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(0, 0, LENGTH, HEIGHT)).unwrap();
        let new_pos = i % LENGTH;
        put_pixel(
            &mut canvas,
            Color::RGB(255, 0, 0),
            Point::new(new_pos as i32, 40),
        );
        put_pixel(
            &mut canvas,
            Color::RGB(0, 255, 0),
            Point::new(new_pos as i32, 280),
        );
        put_pixel(
            &mut canvas,
            Color::RGB(0, 0, 255),
            Point::new(new_pos as i32, 580),
        );
        draw_rectangle(
            &mut canvas,
            Color::RGB(255, 0, 0),
            Point::new(100, 50),
            400,
            200,
        );
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }
    }
}
