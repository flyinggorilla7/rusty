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



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    emulate();
}

pub fn emulate() {
    let mut cpu = cpu::Cpu::new();
    let mut cycle_count: u32 = 0;    
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    const GAME_WIDTH:u32 = 160;
    const GAME_HEIGHT:u32 = 144;
    const SCALE: u32 = 5;
    //Set this back to game_width and game_height
    let window = video.window("Game", GAME_WIDTH*SCALE, GAME_HEIGHT*SCALE)
        .resizable()
        .maximized()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build()
        .expect("could not make into a canvas");
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
        .expect("Failed to create texture target.");
    canvas.clear();
    canvas.copy(&texture, None, Rect::new(0,0,256,256)).unwrap();

    canvas.present();
    let mut event_pump = sdl.event_pump().unwrap();
    //256 pixels * 256 pixels * 3 RGB values for each pixel
    let mut pixel_buffer: [u8; (256*256*3) as usize] = [0; (256*256*3) as usize];

            //Test for Tile Updates
        cpu.memory.vram.write_byte(0x8010, 0xFF);
        cpu.memory.vram.write_byte(0x8011, 0xFF);
        cpu.memory.vram.write_byte(0x8012, 0xFF);
        cpu.memory.vram.write_byte(0x8013, 0xFF);
        cpu.memory.vram.write_byte(0x8014, 0xFF);
        cpu.memory.vram.write_byte(0x8015, 0xFF);
        cpu.memory.vram.write_byte(0x8016, 0xFF);
        cpu.memory.vram.write_byte(0x8017, 0xFF);
        cpu.memory.vram.write_byte(0x8018, 0xFF);
        cpu.memory.vram.write_byte(0x8019, 0xFF);
        cpu.memory.vram.write_byte(0x801A, 0xFF);
        cpu.memory.vram.write_byte(0x801B, 0xFF);
        cpu.memory.vram.write_byte(0x801C, 0xFF);
        cpu.memory.vram.write_byte(0x801D, 0xFF);
        cpu.memory.vram.write_byte(0x801E, 0xFF);
        cpu.memory.vram.write_byte(0x801F, 0xFF);

        cpu.memory.vram.update_tile_map(0x1800, 0x01);
        cpu.memory.vram.update_tile_map(0x1801, 0x01);
        cpu.memory.vram.update_tile_map(0x1801, 0x01);
        cpu.memory.vram.update_tile_map(0x1820, 0x01);
    //CPU cycles, it increments program counter and executes the next instruction
    'running: loop {
        println!("Program Counter: {:#x}", cpu.registers.pc);
        //cycle_count += cpu.cycle() as u32;


        //Tile map and Tile set update automatically when they are written to
        //Pixel Buffer also needs to be updated
        let mut index: u32 = 0;
        if cycle_count >= 0 {
            /*println!("LCD Display Enable: {}",cpu.memory.lcd_display_enable());
            println!("Tile Data Select: {}",cpu.memory.tile_data_select());
            println!("Window Tile Display: {}",cpu.memory.window_tile_display());*/

            for tile_offset in 0..=31 {
                for tile_row in 0..=7 {
                    for tile in 0..=31 {
                        for tile_col in 0..=7 {
                            match cpu.memory.vram.tile_map1[tile_offset*32 + tile][tile_row][tile_col] {
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
                            }
                            index += 3;
                        }
                    }
                }
            }


            //Update screen
            let scrollx = cpu.memory.scrollx() as i32;
            let scrolly = cpu.memory.scrolly() as i32;

            //Pitch is 256 Pixels * 3 bytes per Pixel * SCALE
            texture.update(None, &pixel_buffer, 256 * 3).expect("Failed to update texture.");
            canvas.copy(&texture, Rect::new(0,0,GAME_WIDTH,GAME_HEIGHT), None).unwrap();
            //canvas.copy(&texture, None, Rect::new(scrollx, scrolly,GAME_HEIGHT*SCALE,GAME_HEIGHT*SCALE)).unwrap();
            cycle_count = 0;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                _ => {},
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32));
        canvas.present();
    }
}
