extern crate sdl2;

//With sdl, textures are image data for gpu, surfaces are image data for cpu
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, CanvasBuilder};
use std::time::Duration;
use sdl2::surface::Surface;
use sdl2::rect::{Point, Rect};
use std::env;

mod cpu;
mod register;
mod memory;
mod gpu;
mod emulator;



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    emulate();
}

pub fn emulate() {
    let mut cpu = cpu::Cpu::new();
    let mut cycle_count = 0;    
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    //1200 is width
    //600 is height
    let window = video.window("Game", 160, 144)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build()
        .expect("could not make into a canvas");
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        .expect("Failed to create texture target.");
    texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
        for y in 0..256 {
            for x in 0..256 {
                let offset = y*pitch + x*3;
                //0x92
                buffer[offset] = 0xFF;
                buffer[offset+1] = 0x55;
                buffer[offset+2] = 0x83;
            }
        }
    }).expect("Unable to modify texture pixel data.");

    canvas.clear();
    canvas.copy(&texture, None, Rect::new(0,0,256,256)).unwrap();

    canvas.present();
    let mut event_pump = sdl.event_pump().unwrap();
    //256 pixels * 256 pixels * 3 RGB values for each pixel
    let mut pixel_buffer: [u8; 256*256*3] = [0; 256*256*3];

    //CPU cycles, it increments program counter and executes the next instruction
    'running: loop {

        cycle_count += cpu.cycle();

        //Tile map and Tile set update automatically when they are written to
        //Pixel Buffer also needs to be updated
        let mut index: u32 = 0;
        for tile in cpu.memory.vram.tile_map1.iter() {
            for row in tile {
                for pixel in row {
                    match pixel {
                        gpu::PixelColor::Lightest => {
                            pixel_buffer[index as usize] = 0xFF;
                            pixel_buffer[(index+1) as usize] = 0xFF;
                            pixel_buffer[(index+2) as usize] = 0xFF;
                        }
                        gpu::PixelColor::Light => {
                            pixel_buffer[index as usize] = 0xB3;
                            pixel_buffer[(index+1) as usize] = 0xB3;
                            pixel_buffer[(index+2) as usize] = 0xB3;
                        }
                        gpu::PixelColor::Dark => {
                            pixel_buffer[index as usize] = 0x4D;
                            pixel_buffer[(index+1) as usize] = 0x4D;
                            pixel_buffer[(index+2) as usize] = 0x4D;
                        }
                        gpu::PixelColor::Darkest => {
                            pixel_buffer[index as usize] = 0x00;
                            pixel_buffer[(index + 1) as usize] = 0x00;
                            pixel_buffer[(index + 2) as usize] = 0x00;
                        }
                    };
                    index += 3;
                }
            }

        }

        //Update screen
        let scrollx = cpu.memory.scrollx() as i32;
        let scrolly = cpu.memory.scrolly() as i32;

        //Pitch is 256 Pixels * 3 bytes per Pixel
        texture.update(None, &pixel_buffer, 256 * 3).expect("Failed to update texture.");
        canvas.copy(&texture, None, Rect::new(scrollx, scrolly,160,144)).unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                _ => {},
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        canvas.present();
    }
}
