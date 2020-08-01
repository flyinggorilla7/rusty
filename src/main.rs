extern crate sdl2;

use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, CanvasBuilder};
use std::time::Duration;
use sdl2::rect::{Point, Rect};

mod cpu;
mod register;
mod memory;
mod gpu;



fn main() {

    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("Game", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build()
        .expect("could not make into a canvas");
    let creator = canvas.texture_creator();
    let mut texture = creator.create_texture_target(PixelFormatEnum::RGBA8888, 400, 300)
        .expect("Failed to create texture target.");
    canvas.set_draw_color(Color::RGB(255,255,255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                _ => {},
            }
        }

        canvas.with_texture_canvas(&mut texture, |texture_canvas| {
            texture_canvas.clear();
            texture_canvas.set_draw_color(Color::RGBA(50,50,50,255));
            texture_canvas.fill_rect(Rect::new(0, 0, 400, 300)).expect("could not fill rect");
            //texture_canvas.draw_point(Point::new(400, 300))
              //  .expect("error drawing point");
        }).expect("Failed to draw point.");

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    
    let mut cpu = cpu::Cpu::new();

    cpu.cycle();

    println!("Yummy {}", cpu.registers.sp);
}
