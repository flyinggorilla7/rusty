mod cpu;
mod register;
mod memory;



fn main() {
    println!("Hello, world!");

    let mut cpu = cpu::Cpu::new();

    cpu.cycle();

    println!("Yummy {}", cpu.registers.sp);
}
