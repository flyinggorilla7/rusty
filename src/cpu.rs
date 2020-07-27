use crate::register::Registers;
use crate::memory::Memory;

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
}

const OP_CYCLES: [u32; 256] = [
    1, 3, 2, 2, 1, 1, 2, 1, 5, 2, 2, 2, 1, 1, 2, 1, // 0
    0, 3, 2, 2, 1, 1, 2, 1, 3, 2, 2, 2, 1, 1, 2, 1, // 1
    2, 3, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 2
    2, 3, 2, 2, 3, 3, 3, 1, 2, 2, 2, 2, 1, 1, 2, 1, // 3
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 4
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 5
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 6
    2, 2, 2, 2, 2, 2, 0, 2, 1, 1, 1, 1, 1, 1, 2, 1, // 7
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 8
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // 9
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // a
    1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1, 1, 1, 2, 1, // b
    2, 3, 3, 4, 3, 4, 2, 4, 2, 4, 3, 0, 3, 6, 2, 4, // c
    2, 3, 3, 0, 3, 4, 2, 4, 2, 4, 3, 0, 3, 0, 2, 4, // d
    3, 3, 2, 0, 0, 4, 2, 4, 4, 1, 4, 0, 0, 0, 2, 4, // e
    3, 3, 2, 1, 0, 4, 2, 4, 3, 2, 4, 1, 0, 0, 2, 4, // f
];

impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: Memory::new(),
        }

    }

    //Fetch next byte and increase program counter by one
    fn next_byte(&mut self) -> u8 {
        let data = self.memory.read_byte(self.registers.pc);
        self.registers.pc += 1;
        data
    }

    fn next_word(&mut self) -> u16 {
        let data = self.memory.read_word(self.registers.pc);
        self.registers.pc += 2;
        data
    }

    fn decode_op(&mut self) {

        let opcode = self.next_byte();

        let cycles:u8 = match opcode {

            //load n with immediate 8 bit value
            0x06 => {self.registers.b = self.next_byte(); 2}, //load b with n
            0x0E => {self.registers.c = self.next_byte(); 2}, //load c with n
            0x16 => {self.registers.d = self.next_byte(); 2}, //load d with n
            0x1E => {self.registers.e = self.next_byte(); 2}, //load e with n
            0x26 => {self.registers.h = self.next_byte(); 2}, //load h with n
            0x2E => {self.registers.l = self.next_byte(); 2}, //load l with n
            
            //put value r2 into r1
            //r1=a
            0x7F => {self.registers.a = self.registers.a; 1},
            0x78 => {self.registers.a = self.registers.b; 1},
            0x79 => {self.registers.a = self.registers.c; 1}, 
            0x7A => {self.registers.a = self.registers.d; 1}
            0x7B => {self.registers.a = self.registers.e; 1},
            0x7C => {self.registers.a = self.registers.h; 1},
            0x7D => {self.registers.a = self.registers.l; 1},
            0x0A => {self.registers.a = self.memory.read_byte(self.registers.get_bc()); 2},
            0x1A => {self.registers.a = self.memory.read_byte(self.registers.get_de()); 2},
            0x7E => {self.registers.a = self.memory.read_byte(self.registers.get_hl()); 2},
            0xFA => {self.registers.a = self.memory.read_byte(self.next_word()); 4},
            0x3E => {self.registers.a = self.next_byte(); 2},
            //r1=b
            0x40 => {self.registers.b = self.registers.b; 1},
            0x41 => {self.registers.b = self.registers.c; 1},
            0x42 => {self.registers.b = self.registers.d; 1},
            0x43 => {self.registers.b = self.registers.e; 1},
            0x44 => {self.registers.b = self.registers.h; 1},
            0x45 => {self.registers.b = self.registers.l; 1},
            0x46 => {self.registers.b = self.memory.read_byte(self.registers.get_hl()); 2},
            0x47 => {self.registers.b = self.registers.a; 1},
            //r1=c
            0x48 => {self.registers.c = self.registers.b; 1},
            0x49 => {self.registers.c = self.registers.c; 1},
            0x4A => {self.registers.c = self.registers.d; 1},
            0x4B => {self.registers.c = self.registers.e; 1},
            0x4C => {self.registers.c = self.registers.h; 1},
            0x4D => {self.registers.c = self.registers.l; 1},
            0x4E => {self.registers.c = self.memory.read_byte(self.registers.get_hl()); 2},
            0x4F => {self.registers.c = self.registers.a; 1},
            //r1=d
            0x50 => {self.registers.d = self.registers.b; 1},
            0x51 => {self.registers.d = self.registers.c; 1},
            0x52 => {self.registers.d = self.registers.d; 1},
            0x53 => {self.registers.d = self.registers.e; 1},
            0x54 => {self.registers.d = self.registers.h; 1},
            0x55 => {self.registers.d = self.registers.l; 1},
            0x56 => {self.registers.d = self.memory.read_byte(self.registers.get_hl()); 2},
            0x57 => {self.registers.d = self.registers.a; 1},
            //r1=e
            0x58 => {self.registers.e = self.registers.b; 1},
            0x59 => {self.registers.e = self.registers.c; 1},
            0x5A => {self.registers.e = self.registers.d; 1},
            0x5B => {self.registers.e = self.registers.e; 1},
            0x5C => {self.registers.e = self.registers.h; 1},
            0x5D => {self.registers.e = self.registers.l; 1},
            0x5E => {self.registers.e = self.memory.read_byte(self.registers.get_hl()); 2},
            0x5F => {self.registers.e = self.registers.a; 1},
            //r1=h
            0x60 => {self.registers.h = self.registers.b; 1},
            0x61 => {self.registers.h = self.registers.c; 1},
            0x62 => {self.registers.h = self.registers.d; 1},
            0x63 => {self.registers.h = self.registers.e; 1},
            0x64 => {self.registers.h = self.registers.h; 1},
            0x65 => {self.registers.h = self.registers.l; 1},
            0x66 => {self.registers.h = self.memory.read_byte(self.registers.get_hl()); 2},
            0x67 => {self.registers.h = self.registers.a; 1},
            //r1=l
            0x68 => {self.registers.l = self.registers.b; 1},
            0x69 => {self.registers.l = self.registers.c; 1},
            0x6A => {self.registers.l = self.registers.d; 1},
            0x6B => {self.registers.l = self.registers.e; 1},
            0x6C => {self.registers.l = self.registers.h; 1},
            0x6D => {self.registers.l = self.registers.l; 1},
            0x6E => {self.registers.l = self.memory.read_byte(self.registers.get_hl()); 2},
            0x6F => {self.registers.l = self.registers.a; 1},
            //write 8 bits to memory pointed to by HL
            0x70 => {self.memory.write_byte(self.registers.get_hl(), self.registers.b); 2},
            0x71 => {self.memory.write_byte(self.registers.get_hl(), self.registers.c); 2},
            0x72 => {self.memory.write_byte(self.registers.get_hl(), self.registers.d); 2},
            0x73 => {self.memory.write_byte(self.registers.get_hl(), self.registers.e); 2},
            0x74 => {self.memory.write_byte(self.registers.get_hl(), self.registers.h); 2},
            0x75 => {self.memory.write_byte(self.registers.get_hl(), self.registers.l); 2},
            0x36 => {self.memory.write_byte(self.registers.get_hl(), self.next_byte()); 3},
            //write value of a to memory
            0x02 => {self.memory.write_byte(self.registers.get_bc(), self.registers.a); 2},
            0x12 => {self.memory.write_byte(self.registers.get_de(), self.registers.a); 2},
            0x77 => {self.memory.write_byte(self.registers.get_hl(), self.registers.a); 2},
            0xEA => {self.memory.write_byte(self.next_word(), self.registers.a); 4},


        };

    }



}