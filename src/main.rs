extern crate sdl2;

//With sdl, textures are image data for gpu, surfaces are image data for cpu
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas, CanvasBuilder};
use std::time::Duration;
use sdl2::rect::Rect;
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
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    const GAME_WIDTH:u32 = 160;
    const GAME_HEIGHT:u32 = 144;
    //Set this back to game_width and game_height
    let window = video.window("Game", GAME_WIDTH, GAME_HEIGHT)
        .resizable()
        .maximized()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build()
        .expect("could not make into a canvas");
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGB24, GAME_WIDTH, GAME_HEIGHT)
        .expect("Failed to create texture target.");
    canvas.clear();
    canvas.copy(&texture, None, None).unwrap();

    canvas.present();
    let mut event_pump = sdl.event_pump().unwrap();

    /*//Test for Tile Updates
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
    */

    //CPU cycles, it increments program counter and executes the next instruction
    'running: loop {
        
        if cpu.memory.bios_flag && (cpu.registers.pc == 0x100) {cpu.memory.bios_flag = false;}

        if cpu.registers.pc == 0x000C {
            println!("Finished Clearing VRAM");
        }

        cpu.memory.vram.render_mode_cycles += cpu.cycle() as u32;
        cpu.memory.vram.step();
        //println!("Serial SB: {}", cpu.memory.read_byte(0xFF01));
        //println!("Serial SC: {}", cpu.memory.read_byte(0xFF02));

        if cpu.memory.vram.vblank_flag {
            cpu.memory.vram.vblank_flag = false;
            //Pitch is 160 Pixels * 3 bytes per Pixel
            println!("Screen Refreshed\n");
            texture.update(None, &cpu.memory.vram.pixel_buffer, 160 * 3).expect("Failed to update texture.");
            canvas.copy(&texture, None, None).unwrap();
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

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32/100));
        canvas.present();
    }
}
