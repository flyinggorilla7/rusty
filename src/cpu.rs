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

    //TODO - Check to make sure order of stack pointer operations is correct

    //push register pair nn onto stack and decrement stack pointer twice
    fn push_word(&mut self, data: u16) {
        self.registers.sp -= 2;
        self.memory.write_word(self.registers.sp, data);
    }

    //pop word and increment stack pointer twice
    fn pop_word(&mut self) -> u16 {
        self.registers.sp += 2;
        self.memory.read_word(self.registers.sp)
    }

    fn add8(&mut self, data: u8) {
        self.registers.a += data;
        //Figure out how to set flags properly
    }

    fn add8_carry(&mut self, data: u8) {
        //Implement
    }

    fn sub8(&mut self, data: u8) {
        //Implement
    }

    fn sub8_carry(&mut self, data: u8) {
        //Implement
    }

    //double check flags for this one
    fn and(&mut self, data: u8) {
        self.registers.a &= data;
        if self.registers.a == 0 {
            self.registers.set_zero();
        }
        self.registers.clear_addsub();
        self.registers.clear_carry();
        self.registers.set_halfcarry();
        
    }

    //double check flags for this one
    fn or(&mut self, data: u8) {
        self.registers.a |= data;
        if self.registers.a == 0 {
            self.registers.set_zero();
        }
        self.registers.clear_addsub();
        self.registers.clear_halfcarry();
        self.registers.clear_carry();
    }

    //double check flags for this one
    fn xor(&mut self, data: u8) {
        self.registers.a ^= data;
        if self.registers.a == 0 {
            self.registers.set_zero();
        }
        self.registers.clear_addsub();
        self.registers.clear_halfcarry();
        self.registers.clear_carry();
    }

    //Fix flags and implementation
    fn cmp(&mut self, data: u8) {
        if self.registers.a - data == 0 {
            self.registers.set_zero();
        }
        else {
            self.registers.clear_zero();
        }
    }

    fn inc(&mut self, data: u8) {
        //Implement
    }

    fn dec(&mut self, data: u8) {
        //Implement
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
            //LD A,(C)
            0xF2 => {self.registers.a = self.memory.read_byte((0xFF00 + self.registers.c) as u16); 2},
            //LD (C), A
            0xE2 => {self.memory.write_byte((0xFF00 + self.registers.c) as u16, self.registers.a); 2},
            //LD A,(HLD)
            0x3A => {self.registers.a = self.memory.read_byte(self.registers.get_hl()); self.registers.set_hl(self.registers.get_hl() - 1); 2},
            //LD (HLD), A
            0x32 => {self.memory.write_byte(self.registers.get_hl(), self.registers.a); self.registers.set_hl(self.registers.get_hl() - 1); 2},
            //LD A, (HLI)
            0x2A => {self.registers.a = self.memory.read_byte(self.registers.get_hl()); self.registers.set_hl(self.registers.get_hl() + 1); 2},
            //LD (HLI), A
            0x22 => {self.memory.write_byte(self.registers.get_hl(), self.registers.a); self.registers.set_hl(self.registers.get_hl() + 1); 2},
            //LDH (n), A
            0xE0 => {self.memory.write_byte(0xFF00 + self.next_byte() as u16, self.registers.a); 3},
            //LDH A, (n)
            0xF0 => {self.registers.a = self.memory.read_byte(0xFF00 + self.next_byte() as u16); 3},

            //16 bit loads
            0x01 => {self.registers.set_bc(self.next_word()); 3},
            0x11 => {self.registers.set_de(self.next_word()); 3},
            0x21 => {self.registers.set_hl(self.next_word()); 3},
            0x31 => {self.registers.sp = self.next_word(); 3},
            0xF9 => {self.registers.sp = self.registers.get_hl(); 2},
            //Check flags for this one
            0xF8 => {self.registers.set_hl(self.registers.sp + self.next_byte() as u16); 3},
            //
            0x08 => {self.memory.write_word(self.next_word(), self.registers.sp); 5},
            //PUSH nn
            0xF5 => {self.push_word(self.registers.get_af()); 4},
            0xC5 => {self.push_word(self.registers.get_bc()); 4},
            0xD5 => {self.push_word(self.registers.get_de()); 4},
            0xE5 => {self.push_word(self.registers.get_hl()); 4},
            //POP nn
            0xF1 => {self.registers.set_af(self.pop_word()); 3},
            0xC1 => {self.registers.set_bc(self.pop_word()); 3},
            0xD1 => {self.registers.set_de(self.pop_word()); 3},
            0xE1 => {self.registers.set_hl(self.pop_word()); 3},

            //8 bit ALU - add n to a
            0x87 => {self.add8(self.registers.a); 1},
            0x80 => {self.add8(self.registers.b); 1},
            0x81 => {self.add8(self.registers.c); 1},
            0x82 => {self.add8(self.registers.d); 1},
            0x83 => {self.add8(self.registers.e); 1},
            0x84 => {self.add8(self.registers.h); 1},
            0x85 => {self.add8(self.registers.l); 1},
            0x86 => {self.add8(self.memory.read_byte(self.registers.get_hl())); 2},
            0xC6 => {self.add8(self.next_byte()); 2},
            //8 bit add n + carry flag to A
            0x8F => {self.add8_carry(self.registers.a); 1},
            0x88 => {self.add8_carry(self.registers.b); 1},
            0x89 => {self.add8_carry(self.registers.c); 1},
            0x8A => {self.add8_carry(self.registers.d); 1},
            0x8B => {self.add8_carry(self.registers.e); 1},
            0x8C => {self.add8_carry(self.registers.h); 1},
            0x8D => {self.add8_carry(self.registers.l); 1},
            0x8E => {self.add8_carry(self.memory.read_byte(self.registers.get_hl())); 2},
            0xCE => {self.add8_carry(self.next_byte()); 2},
            //8 bit subtract n from A
            0x97 => {self.sub8(self.registers.a); 1},
            0x90 => {self.sub8(self.registers.b); 1},
            0x91 => {self.sub8(self.registers.c); 1},
            0x92 => {self.sub8(self.registers.d); 1},
            0x93 => {self.sub8(self.registers.e); 1},
            0x94 => {self.sub8(self.registers.h); 1},
            0x95 => {self.sub8(self.registers.l); 1},
            0x96 => {self.sub8(self.memory.read_byte(self.registers.get_hl())); 2},
            0xD6 => {self.sub8(self.next_byte()); 2},
            //8 bit subtract n from A with carry
            0x9F => {self.sub8_carry(self.registers.a); 1},
            0x98 => {self.sub8_carry(self.registers.b); 1},
            0x99 => {self.sub8_carry(self.registers.c); 1},
            0x9A => {self.sub8_carry(self.registers.d); 1},
            0x9B => {self.sub8_carry(self.registers.e); 1},
            0x9C => {self.sub8_carry(self.registers.h); 1},
            0x9D => {self.sub8_carry(self.registers.l); 1},
            0x9E => {self.sub8_carry(self.memory.read_byte(self.registers.get_hl())); 2},
            0xDE => {self.sub8_carry(self.next_byte()); 2},
            //8 bit AND
            0xA7 => {self.and(self.registers.a); 1},
            0xA0 => {self.and(self.registers.b); 1},
            0xA1 => {self.and(self.registers.c); 1},
            0xA2 => {self.and(self.registers.d); 1},
            0xA3 => {self.and(self.registers.e); 1},
            0xA4 => {self.and(self.registers.h); 1},
            0xA5 => {self.and(self.registers.l); 1},
            0xA6 => {self.and(self.memory.read_byte(self.registers.get_hl())); 2},
            0xE6 => {self.and(self.next_byte()); 2},
            //8 bit OR
            0xB7 => {self.or(self.registers.a); 1},
            0xB0 => {self.or(self.registers.b); 1},
            0xB1 => {self.or(self.registers.c); 1},
            0xB2 => {self.or(self.registers.d); 1},
            0xB3 => {self.or(self.registers.e); 1},
            0xB4 => {self.or(self.registers.h); 1},
            0xB5 => {self.or(self.registers.l); 1},
            0xB6 => {self.or(self.memory.read_byte(self.registers.get_hl())); 2},
            0xF6 => {self.or(self.next_byte()); 2},
            //8 bit subtract n from A with carry
            0xAF => {self.xor(self.registers.b); 1},
            0xA8 => {self.xor(self.registers.a); 1},
            0xA9 => {self.xor(self.registers.c); 1},
            0xAA => {self.xor(self.registers.d); 1},
            0xAB => {self.xor(self.registers.e); 1},
            0xAC => {self.xor(self.registers.h); 1},
            0xAD => {self.xor(self.registers.l); 1},
            0xAE => {self.xor(self.memory.read_byte(self.registers.get_hl())); 2},
            0xEE => {self.xor(self.next_byte()); 2},
            //8 bit compare n with a
            0xBF => {self.cmp(self.registers.b); 1},
            0xB8 => {self.cmp(self.registers.a); 1},
            0xB9 => {self.cmp(self.registers.c); 1},
            0xBA => {self.cmp(self.registers.d); 1},
            0xBB => {self.cmp(self.registers.e); 1},
            0xBC => {self.cmp(self.registers.h); 1},
            0xBD => {self.cmp(self.registers.l); 1},
            0xBE => {self.cmp(self.memory.read_byte(self.registers.get_hl())); 2},
            0xFE => {self.cmp(self.next_byte()); 2},
            //INC register n
            0x3C => {self.registers.a += 1; 1},
            0x04 => {self.registers.b += 1; 1},
            0x0C => {self.registers.c += 1; 1},
            0x14 => {self.registers.d += 1; 1},
            0x1C => {self.registers.e += 1; 1},
            0x24 => {self.registers.h += 1; 1},
            0x2C => {self.registers.l += 1; 1},
            0x34 => {self.memory.inc_memory_byte(self.registers.get_hl()); 3},
            //DEC register n
            0x3D => {self.registers.a -= 1; 1},
            0x05 => {self.registers.b -= 1; 1},
            0x0D => {self.registers.c -= 1; 1},
            0x15 => {self.registers.d -= 1; 1},
            0x1D => {self.registers.e -= 1; 1},
            0x25 => {self.registers.h -= 1; 1},
            0x2D => {self.registers.l -= 1; 1},
            0x35 => {self.memory.dec_memory_byte(self.registers.get_hl()); 3},
            //Add 16 bit

        };

    }



}