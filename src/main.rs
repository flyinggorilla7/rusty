extern crate sdl2;
mod cpu;
mod register;
mod memory;



fn main() {

    let _sdl = sdl2::init().unwrap();

    let mut cpu = cpu::Cpu::new();

    cpu.cycle();

    println!("Yummy {}", cpu.registers.sp);
}
