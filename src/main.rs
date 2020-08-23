extern crate sdl2;

//With sdl, textures are image data for gpu, surfaces are image data for cpu
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use std::time::Duration;
use std::env;

use std::io;

mod cpu;
mod register;
mod memory;
mod gpu;

pub struct DebugMode {
    pub run: bool,  //Run until breakpoint
    pub step: bool, //Cycle through each step and poll for input each time
    pub reset: bool, //Resets program counter to 0x100
    pub is_breakpoint: bool, //Tells Input Checker that second token should be u16 address
    pub breakpoints: Vec<u16>,   //Vector containing all breakpoints
}



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let mut da: bool = false;
    let mut debug: bool = false;

    for arg in args {
        
        if arg == "da" {
            da = true;
        }
        else if arg == "debug" {
            debug = true;
        }
        else if arg == "help" {
            println!("da - print rom disassembly to file, debug - run emulator in debug mode");
        }

    }

    if da {
        disassembly();
    }
    else {
        loop {
            let reset: bool = emulate(debug);
            println!("{}", reset);
            if !reset {
                return
            }
        }
    }
}

pub fn disassembly() {
    let mut cpu = cpu::Cpu::new();
    cpu.memory.memory_setup();

    cpu.print_disassembly();
}

pub fn get_debug_input(debug_mode: &mut DebugMode) {

    //Use Cases for Debug Mode
    //r - run until next break point
    //s - step by one cpu cycle
    //b a16 - set break point at specified 16 bit address (Format of 0xABCD)
    //printbreak - printt list of all break points

    loop {
        let mut user_input = String::new();

        println!("Please enter debug command. Type help for a list of commands.");
        io::stdin().read_line(&mut user_input).unwrap();

        let mut iter = user_input.split_ascii_whitespace();

        debug_mode.run = false;
        debug_mode.step = false;
        debug_mode.is_breakpoint = false;
        debug_mode.reset = false;

        let next_input;
        match iter.next() {
            Some(input) => next_input = input,
            None => {println!("No input entered"); continue},
        };
        let mut break_type: &str = "";
        match next_input {
            "reset" => {debug_mode.reset = true; return},
            "r" => {debug_mode.run = true; return},
            "s" => {debug_mode.step = true; return},
            "b" => {break_type = "b"; debug_mode.is_breakpoint = true},
            "cb" => {break_type = "cb"; debug_mode.is_breakpoint = true},
            "help" => println!("r - run, s - step, b 0xABCD - set breakpoint at address ABCD, cb 0xABCD - clear breakpoint at address ABCD, printbreak - print list of breakpoints, reset - reset at 0x0100\n"),
            "printbreak" => {
                for breakpoint in debug_mode.breakpoints.iter().as_ref() {
                    println!("{:#04X}", *breakpoint);
                }
            
            }
            _ => {println!("Invalid Debug Command, Type help for list of commands."); continue},
        };
        if debug_mode.is_breakpoint {
            //Get Breakpoint Address and Add to Breakpoint Vector if not already there
            let address = iter.next().expect("Invalid Debug Command. \"b\" command should be followed with u16 address").trim_start_matches("0x");
            let hex_address: u16;
            match u16::from_str_radix(address,16) {
                Ok(hex_value) => hex_address = hex_value,
                Err(_error) => {println!("Invalid Address. Enter address in format of 0xABCD."); continue},
            };
            if break_type == "b" {
                if !debug_mode.breakpoints.contains(&hex_address) {
                    debug_mode.breakpoints.push(hex_address);
                    println!("Breakpoint set at Address {:#04X}\n", hex_address);
                }
                else {
                    println!("Breakpoint already set at Address {:#04X}\n", hex_address);
                }
            }
            else {
                if !debug_mode.breakpoints.contains(&hex_address) {
                    println!("Breakpoint at Address {:#04X} cannot be removed since it was never set.\n", hex_address);
                }
                else {
                    let index = debug_mode.breakpoints.binary_search(&hex_address).unwrap();
                    debug_mode.breakpoints.remove(index);
                    println!("Breakpoint removed at Address {:#04X}\n", hex_address);
                }
            }
        }
    }
}

pub fn emulate(debug: bool) -> bool {
    let mut cpu = cpu::Cpu::new();
    cpu.memory.memory_setup();
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

    let mut debug_mode = DebugMode {
        run: false,
        step: false,
        reset: false,
        is_breakpoint: false,
        breakpoints: Vec::new(),
    };

    //Test for Tile Updates
    /*cpu.memory.vram.write_byte(0x8010, 0xFF);
    cpu.memory.vram.write_byte(0x8011, 0x00);
    cpu.memory.vram.write_byte(0x8012, 0xFF);
    cpu.memory.vram.write_byte(0x8013, 0x00);
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

    cpu.memory.vram.write_byte(0x9800, 1);
    cpu.memory.vram.write_byte(0x9801, 1);
    cpu.memory.vram.write_byte(0x9802, 1);
    cpu.memory.vram.write_byte(0x9810, 1);
    cpu.memory.vram.write_byte(0x9818, 1);
    cpu.memory.write_byte(0xFF40, 0x80);*/
    

    //CPU cycles, it increments program counter and executes the next instruction
    'running: loop {
        
        if cpu.memory.bios_flag && (cpu.registers.pc == 0x100) {cpu.memory.bios_flag = false;}
        
        cpu.debug = false;
        if debug {
            if debug_mode.breakpoints.contains(&cpu.registers.pc) {
                println!("Breakpoint At {:#04X} Reached.", cpu.registers.pc);
                cpu.debug = true;
                get_debug_input(&mut debug_mode);
            }
            else if debug_mode.reset {
                return true
            }
            else if debug_mode.step {
                cpu.debug = true;
                get_debug_input(&mut debug_mode);
            }
            else if !debug_mode.run {
                cpu.debug = true;
                get_debug_input(&mut debug_mode);
            }

        }


        cpu.memory.vram.render_mode_cycles += cpu.cycle() as u32;
        //cpu.memory.vram.render_mode_cycles += 4;
        cpu.memory.vram.step();
        //println!("Serial SB: {}", cpu.memory.read_byte(0xFF01));
        //println!("Serial SC: {}", cpu.memory.read_byte(0xFF02));

        if cpu.memory.vram.vblank_flag {
            cpu.memory.vram.vblank_flag = false;
            //Pitch is 160 Pixels * 3 bytes per Pixel
            //println!("Scroll Value: {}", cpu.memory.vram.scroll_x);
            cpu.memory.vram.scroll_x = cpu.memory.vram.scroll_x.wrapping_add(1);
            texture.update(None, &cpu.memory.vram.pixel_buffer, 160 * 3).expect("Failed to update texture.");
            canvas.copy(&texture, None, None).unwrap();
            canvas.present();

            //true - direction/bit 4 = 0
            let input_status = cpu.memory.input_status();

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    }

                    _ => {},
                }
                if input_status {
                    match event {
                        //Up
                        Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                            cpu.memory.memory[0xFF00] &= !(1u8 << 2);
                        }
                        Event::KeyUp { keycode: Some(Keycode::Up), .. } => {
                            cpu.memory.memory[0xFF00] |= 1u8 << 2;
                        }

                        //Down
                        Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                            cpu.memory.memory[0xFF00] &= !(1u8 << 3);
                        }
                        Event::KeyUp { keycode: Some(Keycode::Down), .. } => {
                            cpu.memory.memory[0xFF00] |= 1u8 << 3;
                        }

                        //Left
                        Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                            cpu.memory.memory[0xFF00] &= !(1u8 << 1);
                        }
                        Event::KeyUp { keycode: Some(Keycode::Left), .. } => {
                            cpu.memory.memory[0xFF00] |= 1u8 << 1;
                        }

                        //Right
                        Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                            cpu.memory.memory[0xFF00] &= !(1u8 << 0);
                        }
                        Event::KeyUp { keycode: Some(Keycode::Right), .. } => {
                            cpu.memory.memory[0xFF00] |= 1u8 << 0;
                        }

                        _ => {}
                    }
                }
                else {
                    match event {
                        //T - Start
                        Event::KeyDown { keycode: Some(Keycode::T), .. } => {
                            cpu.memory.memory[0xFF00] &= !(1u8 << 3);
                        }
                        Event::KeyUp { keycode: Some(Keycode::T), .. } => {
                            cpu.memory.memory[0xFF00] |= 1u8 << 3;
                        }

                        //Y - Select
                        Event::KeyDown { keycode: Some(Keycode::Y), .. } => {
                            cpu.memory.memory[0xFF00] &= !(1u8 << 2);
                        }
                        Event::KeyUp { keycode: Some(Keycode::Y), .. } => {
                            cpu.memory.memory[0xFF00] |= 1u8 << 2;
                        }

                        //S - Button B
                        Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                            cpu.memory.memory[0xFF00] &= !(1u8 << 1);
                        }
                        Event::KeyUp { keycode: Some(Keycode::S), .. } => {
                            cpu.memory.memory[0xFF00] |= 1u8 << 1;
                        }

                        //A - Button A
                        Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                            cpu.memory.memory[0xFF00] &= !(1u8 << 0);
                        }
                        Event::KeyUp { keycode: Some(Keycode::A), .. } => {
                            cpu.memory.memory[0xFF00] |= 1u8 << 0;
                        }

                        _ => {}               
                    }
                }
            }
        }

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32/1000000));
    }
    return false;
}

pub fn poll_input() {
    
}
