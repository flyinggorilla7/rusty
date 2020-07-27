mod cpu;
mod register;
mod memory;



fn main() {
    println!("Hello, world!");

    let cpu = cpu::Cpu::new();

    println!("Yummy {}", cpu.registers.sp);
}
