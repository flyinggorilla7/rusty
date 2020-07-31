use crate::register::Registers;
use crate::memory::Memory;

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory,
    pub halted: bool,
    pub stopped: bool,
    pub interrupts_enabled: bool,
}

//CHECK WRAPPING

impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            memory: Memory::new(),
            halted: false,
            stopped: false,
            interrupts_enabled: true,
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

    //Add 8 bit value to register a, set appropriate flags
    fn add8(&mut self, data: u8) {
        if ((self.registers.a & 0xF) + (data & 0xF)) & 0x10 == 0x10 {
            self.registers.set_halfcarry(1);
        }
        else {
            self.registers.set_halfcarry(0);
        }
        if data as u16 + self.registers.a as u16 > 0xFF {
            self.registers.set_carry(1);
        }
        else {
            self.registers.set_carry(0);
        }
        self.registers.a += data;
        if self.registers.a == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_addsub(0);
    }

    //Add 8 bit value + carry, set appropriate values
    fn add8_carry(&mut self, mut data: u8) {
        if self.registers.check_carry() {
            if self.registers.a < data {
                self.registers.a += 1;
            }
            else {
                data += 1;
            }
        }
        if ((self.registers.a & 0xF) + (data & 0xF)) & 0x10 == 0x10 {
            self.registers.set_halfcarry(1);
        }
        else {
            self.registers.set_halfcarry(0);
        }
        if data as u16 + self.registers.a as u16 > 0xFF {
            self.registers.set_carry(1);
        }
        else {
            self.registers.set_carry(0);
        }
        self.registers.a += data;
        if self.registers.a == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_addsub(0);
    }

    //Sub 8 bit value from register a, set appropriate flags
    fn sub8(&mut self, data: u8) {
        if data & 0xF > self.registers.a & 0xF {
            self.registers.set_halfcarry(1);
        }
        else {
            self.registers.set_halfcarry(0);
        }
        if data & 0xFF > self.registers.a & 0xFF {
            self.registers.set_carry(1);
        }
        else {
            self.registers.set_carry(0);
        }
        self.registers.a -= data;
        if self.registers.a == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_addsub(1);
    }

    //Sub 8 bit value and carry, set appropriate flags
    fn sub8_carry(&mut self, mut data: u8) {
        if self.registers.check_carry() {
            if self.registers.a > data {
                self.registers.a -= 1;
            }
            else {
                data -= 1;
            }
        }
        if data & 0xF > self.registers.a & 0xF {
            self.registers.set_halfcarry(1);
        }
        else {
            self.registers.set_halfcarry(0);
        }
        if data & 0xFF > self.registers.a & 0xFF {
            self.registers.set_carry(1);
        }
        else {
            self.registers.set_carry(0);
        }
        self.registers.a -= data;
        if self.registers.a == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_addsub(1);
    }

    //Bitwise AND with register a, store value in a
    fn and(&mut self, data: u8) {
        self.registers.a &= data;
        if self.registers.a == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0)
        }
        self.registers.set_addsub(0);
        self.registers.set_carry(0);
        self.registers.set_halfcarry(1);
        
    }

    //Bitwise OR with register a, store value in a
    fn or(&mut self, data: u8) {
        self.registers.a |= data;
        if self.registers.a == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_addsub(0);
        self.registers.set_halfcarry(0);
        self.registers.set_carry(0);
    }

    //Bitwise XOR with register a, store value in a
    fn xor(&mut self, data: u8) {
        self.registers.a ^= data;
        if self.registers.a == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0)
        }
        self.registers.set_addsub(0);
        self.registers.set_halfcarry(0);
        self.registers.set_carry(0);
    }

    //Fix flags and implementation
    fn cmp(&mut self, data: u8) {
        if self.registers.a - data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
    }

    //INC, check for zero and half-carry flag
    fn inc(&mut self, mut data: u8) -> u8 {
        if((data & 0xF) + (1u8 & 0xF)) & 0x10 == 0x10 {
            self.registers.set_halfcarry(1);
        }
        else {
            self.registers.set_halfcarry(0);
        }
        data += 1;
        if data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_addsub(0);
        data
    }

    fn dec(&mut self, mut data: u8) -> u8 {
        if data & 0xF == 0 {
            self.registers.set_halfcarry(1);
        }
        else {
            self.registers.set_halfcarry(0);
        }
        data -= 1;
        if data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_addsub(1);
        data
    }

    
    fn add_hl(&mut self, data: u16) {
        if ((self.registers.hl() & 0xF0) + (data & 0xF0)) & 0x100 == 0x100 {
            self.registers.set_halfcarry(1);
        }
        else {
            self.registers.set_halfcarry(0);
        }
        if data as u32 + self.registers.a as u32 > 0xFFFF {
            self.registers.set_carry(1);
        }
        else {
            self.registers.set_carry(0);
        }
        self.registers.set_hl(self.registers.hl() + data);
        self.registers.set_addsub(0);
    }
    
    fn add_sp(&mut self, data: u16) {
        if ((self.registers.sp & 0xF0) + (data as u16 & 0xF0)) & 0x100 == 0x100 {
            self.registers.set_halfcarry(1);
        }
        else {
            self.registers.set_halfcarry(0);
        }
        if data as u32 + self.registers.sp as u32 > 0xFFFF {
            self.registers.set_carry(1);
        }
        else {
            self.registers.set_carry(0);
        }
        self.registers.sp += data as u16;
        self.registers.set_zero(0);
        self.registers.set_addsub(0);
    }
    



    pub fn cycle(&mut self) -> u8 {

        let opcode = self.next_byte();

        let cycles: u8 = match opcode {

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
            0x0A => {self.registers.a = self.memory.read_byte(self.registers.bc()); 2},
            0x1A => {self.registers.a = self.memory.read_byte(self.registers.de()); 2},
            0x7E => {self.registers.a = self.memory.read_byte(self.registers.hl()); 2},
            0xFA => {let word = self.next_word(); self.registers.a = self.memory.read_byte(word); 4},
            0x3E => {self.registers.a = self.next_byte(); 2},
            //r1=b
            0x40 => {self.registers.b = self.registers.b; 1},
            0x41 => {self.registers.b = self.registers.c; 1},
            0x42 => {self.registers.b = self.registers.d; 1},
            0x43 => {self.registers.b = self.registers.e; 1},
            0x44 => {self.registers.b = self.registers.h; 1},
            0x45 => {self.registers.b = self.registers.l; 1},
            0x46 => {self.registers.b = self.memory.read_byte(self.registers.hl()); 2},
            0x47 => {self.registers.b = self.registers.a; 1},
            //r1=c
            0x48 => {self.registers.c = self.registers.b; 1},
            0x49 => {self.registers.c = self.registers.c; 1},
            0x4A => {self.registers.c = self.registers.d; 1},
            0x4B => {self.registers.c = self.registers.e; 1},
            0x4C => {self.registers.c = self.registers.h; 1},
            0x4D => {self.registers.c = self.registers.l; 1},
            0x4E => {self.registers.c = self.memory.read_byte(self.registers.hl()); 2},
            0x4F => {self.registers.c = self.registers.a; 1},
            //r1=d
            0x50 => {self.registers.d = self.registers.b; 1},
            0x51 => {self.registers.d = self.registers.c; 1},
            0x52 => {self.registers.d = self.registers.d; 1},
            0x53 => {self.registers.d = self.registers.e; 1},
            0x54 => {self.registers.d = self.registers.h; 1},
            0x55 => {self.registers.d = self.registers.l; 1},
            0x56 => {self.registers.d = self.memory.read_byte(self.registers.hl()); 2},
            0x57 => {self.registers.d = self.registers.a; 1},
            //r1=e
            0x58 => {self.registers.e = self.registers.b; 1},
            0x59 => {self.registers.e = self.registers.c; 1},
            0x5A => {self.registers.e = self.registers.d; 1},
            0x5B => {self.registers.e = self.registers.e; 1},
            0x5C => {self.registers.e = self.registers.h; 1},
            0x5D => {self.registers.e = self.registers.l; 1},
            0x5E => {self.registers.e = self.memory.read_byte(self.registers.hl()); 2},
            0x5F => {self.registers.e = self.registers.a; 1},
            //r1=h
            0x60 => {self.registers.h = self.registers.b; 1},
            0x61 => {self.registers.h = self.registers.c; 1},
            0x62 => {self.registers.h = self.registers.d; 1},
            0x63 => {self.registers.h = self.registers.e; 1},
            0x64 => {self.registers.h = self.registers.h; 1},
            0x65 => {self.registers.h = self.registers.l; 1},
            0x66 => {self.registers.h = self.memory.read_byte(self.registers.hl()); 2},
            0x67 => {self.registers.h = self.registers.a; 1},
            //r1=l
            0x68 => {self.registers.l = self.registers.b; 1},
            0x69 => {self.registers.l = self.registers.c; 1},
            0x6A => {self.registers.l = self.registers.d; 1},
            0x6B => {self.registers.l = self.registers.e; 1},
            0x6C => {self.registers.l = self.registers.h; 1},
            0x6D => {self.registers.l = self.registers.l; 1},
            0x6E => {self.registers.l = self.memory.read_byte(self.registers.hl()); 2},
            0x6F => {self.registers.l = self.registers.a; 1},
            //write 8 bits to memory pointed to by HL
            0x70 => {self.memory.write_byte(self.registers.hl(), self.registers.b); 2},
            0x71 => {self.memory.write_byte(self.registers.hl(), self.registers.c); 2},
            0x72 => {self.memory.write_byte(self.registers.hl(), self.registers.d); 2},
            0x73 => {self.memory.write_byte(self.registers.hl(), self.registers.e); 2},
            0x74 => {self.memory.write_byte(self.registers.hl(), self.registers.h); 2},
            0x75 => {self.memory.write_byte(self.registers.hl(), self.registers.l); 2},
            0x36 => {let byte = self.next_byte(); self.memory.write_byte(self.registers.hl(), byte); 3},
            //write value of a to memory
            0x02 => {self.memory.write_byte(self.registers.bc(), self.registers.a); 2},
            0x12 => {self.memory.write_byte(self.registers.de(), self.registers.a); 2},
            0x77 => {self.memory.write_byte(self.registers.hl(), self.registers.a); 2},
            0xEA => {let word = self.next_word(); self.memory.write_byte(word, self.registers.a); 4},
            //LD A,(C)
            0xF2 => {self.registers.a = self.memory.read_byte(0xFF00 + self.registers.c as u16); 2},
            //LD (C), A
            0xE2 => {self.memory.write_byte(0xFF00 + self.registers.c as u16, self.registers.a); 2},
            //LD A,(HLD)
            0x3A => {self.registers.a = self.memory.read_byte(self.registers.hl()); self.registers.set_hl(self.registers.hl() - 1); 2},
            //LD (HLD), A
            0x32 => {self.memory.write_byte(self.registers.hl(), self.registers.a); self.registers.set_hl(self.registers.hl() - 1); 2},
            //LD A, (HLI)
            0x2A => {self.registers.a = self.memory.read_byte(self.registers.hl()); self.registers.set_hl(self.registers.hl() + 1); 2},
            //LD (HLI), A
            0x22 => {self.memory.write_byte(self.registers.hl(), self.registers.a); self.registers.set_hl(self.registers.hl() + 1); 2},
            //LDH (n), A
            0xE0 => {let byte = self.next_byte(); self.memory.write_byte(0xFF00 + byte as u16, self.registers.a); 3},
            //LDH A, (n)
            0xF0 => {let byte = self.next_byte(); self.registers.a = self.memory.read_byte(0xFF00 + byte as u16); 3},

            //16 bit loads
            0x01 => {let word = self.next_word(); self.registers.set_bc(word); 3},
            0x11 => {let word = self.next_word(); self.registers.set_de(word); 3},
            0x21 => {let word = self.next_word(); self.registers.set_hl(word); 3},
            0x31 => {self.registers.sp = self.next_word(); 3},
            0xF9 => {self.registers.sp = self.registers.hl(); 2},
            //Check flags for this one
            0xF8 => {let byte = self.next_byte(); self.registers.set_hl(self.registers.sp + byte as u16); 3},
            //
            0x08 => {let word = self.next_word(); self.memory.write_word(word, self.registers.sp); 5},
            //PUSH nn
            0xF5 => {self.push_word(self.registers.af()); 4},
            0xC5 => {self.push_word(self.registers.bc()); 4},
            0xD5 => {self.push_word(self.registers.de()); 4},
            0xE5 => {self.push_word(self.registers.hl()); 4},
            //POP nn
            0xF1 => {let word = self.pop_word(); self.registers.set_af(word); 3},
            0xC1 => {let word = self.pop_word(); self.registers.set_bc(word); 3},
            0xD1 => {let word = self.pop_word(); self.registers.set_de(word); 3},
            0xE1 => {let word = self.pop_word(); self.registers.set_hl(word); 3},

            //8 bit ALU - add n to a
            0x87 => {self.add8(self.registers.a); 1},
            0x80 => {self.add8(self.registers.b); 1},
            0x81 => {self.add8(self.registers.c); 1},
            0x82 => {self.add8(self.registers.d); 1},
            0x83 => {self.add8(self.registers.e); 1},
            0x84 => {self.add8(self.registers.h); 1},
            0x85 => {self.add8(self.registers.l); 1},
            0x86 => {self.add8(self.memory.read_byte(self.registers.hl())); 2},
            0xC6 => {let byte = self.next_byte(); self.add8(byte); 2},
            //8 bit add n + carry flag to A
            0x8F => {self.add8_carry(self.registers.a); 1},
            0x88 => {self.add8_carry(self.registers.b); 1},
            0x89 => {self.add8_carry(self.registers.c); 1},
            0x8A => {self.add8_carry(self.registers.d); 1},
            0x8B => {self.add8_carry(self.registers.e); 1},
            0x8C => {self.add8_carry(self.registers.h); 1},
            0x8D => {self.add8_carry(self.registers.l); 1},
            0x8E => {self.add8_carry(self.memory.read_byte(self.registers.hl())); 2},
            0xCE => {let byte = self.next_byte(); self.add8_carry(byte); 2},
            //8 bit subtract n from A
            0x97 => {self.sub8(self.registers.a); 1},
            0x90 => {self.sub8(self.registers.b); 1},
            0x91 => {self.sub8(self.registers.c); 1},
            0x92 => {self.sub8(self.registers.d); 1},
            0x93 => {self.sub8(self.registers.e); 1},
            0x94 => {self.sub8(self.registers.h); 1},
            0x95 => {self.sub8(self.registers.l); 1},
            0x96 => {self.sub8(self.memory.read_byte(self.registers.hl())); 2},
            0xD6 => {let byte = self.next_byte(); self.sub8(byte); 2},
            //8 bit subtract n from A with carry
            0x9F => {self.sub8_carry(self.registers.a); 1},
            0x98 => {self.sub8_carry(self.registers.b); 1},
            0x99 => {self.sub8_carry(self.registers.c); 1},
            0x9A => {self.sub8_carry(self.registers.d); 1},
            0x9B => {self.sub8_carry(self.registers.e); 1},
            0x9C => {self.sub8_carry(self.registers.h); 1},
            0x9D => {self.sub8_carry(self.registers.l); 1},
            0x9E => {self.sub8_carry(self.memory.read_byte(self.registers.hl())); 2},
            0xDE => {let byte = self.next_byte(); self.sub8_carry(byte); 2},
            //8 bit AND
            0xA7 => {self.and(self.registers.a); 1},
            0xA0 => {self.and(self.registers.b); 1},
            0xA1 => {self.and(self.registers.c); 1},
            0xA2 => {self.and(self.registers.d); 1},
            0xA3 => {self.and(self.registers.e); 1},
            0xA4 => {self.and(self.registers.h); 1},
            0xA5 => {self.and(self.registers.l); 1},
            0xA6 => {self.and(self.memory.read_byte(self.registers.hl())); 2},
            0xE6 => {let byte = self.next_byte(); self.and(byte); 2},
            //8 bit OR
            0xB7 => {self.or(self.registers.a); 1},
            0xB0 => {self.or(self.registers.b); 1},
            0xB1 => {self.or(self.registers.c); 1},
            0xB2 => {self.or(self.registers.d); 1},
            0xB3 => {self.or(self.registers.e); 1},
            0xB4 => {self.or(self.registers.h); 1},
            0xB5 => {self.or(self.registers.l); 1},
            0xB6 => {self.or(self.memory.read_byte(self.registers.hl())); 2},
            0xF6 => {let byte = self.next_byte(); self.or(byte); 2},
            //8 bit XOR
            0xAF => {self.xor(self.registers.b); 1},
            0xA8 => {self.xor(self.registers.a); 1},
            0xA9 => {self.xor(self.registers.c); 1},
            0xAA => {self.xor(self.registers.d); 1},
            0xAB => {self.xor(self.registers.e); 1},
            0xAC => {self.xor(self.registers.h); 1},
            0xAD => {self.xor(self.registers.l); 1},
            0xAE => {self.xor(self.memory.read_byte(self.registers.hl())); 2},
            0xEE => {let byte = self.next_byte(); self.xor(byte); 2},
            //8 bit compare n with a
            0xBF => {self.cmp(self.registers.b); 1},
            0xB8 => {self.cmp(self.registers.a); 1},
            0xB9 => {self.cmp(self.registers.c); 1},
            0xBA => {self.cmp(self.registers.d); 1},
            0xBB => {self.cmp(self.registers.e); 1},
            0xBC => {self.cmp(self.registers.h); 1},
            0xBD => {self.cmp(self.registers.l); 1},
            0xBE => {self.cmp(self.memory.read_byte(self.registers.hl())); 2},
            0xFE => {let byte = self.next_byte(); self.cmp(byte); 2},
            //INC register n
            0x3C => {self.registers.a = self.inc(self.registers.a); 1},
            0x04 => {self.registers.b = self.inc(self.registers.b); 1},
            0x0C => {self.registers.c = self.inc(self.registers.c); 1},
            0x14 => {self.registers.d = self.inc(self.registers.d); 1},
            0x1C => {self.registers.e = self.inc(self.registers.e); 1},
            0x24 => {self.registers.h = self.inc(self.registers.h); 1},
            0x2C => {self.registers.l = self.inc(self.registers.l); 1},
            0x34 => {let inc = self.inc(self.memory.read_byte(self.registers.hl())); self.memory.write_byte(self.registers.hl(), inc); 3},
            //DEC register n
            0x3D => {self.registers.a = self.dec(self.registers.a); 1},
            0x05 => {self.registers.b = self.dec(self.registers.b); 1},
            0x0D => {self.registers.c = self.dec(self.registers.c); 1},
            0x15 => {self.registers.d = self.dec(self.registers.d); 1},
            0x1D => {self.registers.e = self.dec(self.registers.e); 1},
            0x25 => {self.registers.h = self.dec(self.registers.h); 1},
            0x2D => {self.registers.l = self.dec(self.registers.l); 1},
            0x35 => {let dec = self.dec(self.memory.read_byte(self.registers.hl())); self.memory.write_byte(self.registers.hl(), dec); 3},
            //Add to HL
            0x09 => {self.add_hl(self.registers.bc()); 2},
            0x19 => {self.add_hl(self.registers.de()); 2},
            0x29 => {self.add_hl(self.registers.hl()); 2},
            0x39 => {self.add_hl(self.registers.sp); 2},
            //Add to SP
            0xE8 => {let byte = self.next_byte(); self.add_sp(byte as u16 ); 4},
            //INC register nn
            0x03 => {self.registers.set_bc(self.registers.bc()+1); 2},
            0x13 => {self.registers.set_de(self.registers.de()+1); 2},
            0x23 => {self.registers.set_hl(self.registers.hl()+1); 2},
            0x33 => {self.registers.sp += 1; 2},
            //DEC register nn
            0x0B => {self.registers.set_bc(self.registers.bc()-1); 2},
            0x1B => {self.registers.set_de(self.registers.de()-1); 2},
            0x2B => {self.registers.set_hl(self.registers.hl()-1); 2},
            0x3B => {self.registers.sp -= 1; 2},
            //Decimal adjust register A
            0x27 => {self.registers.check_halfcarry(); self.registers.check_addsub(); 1}, //Implement
            //CPL Register A
            0x2F => {self.cpl(); 1},
            //CCF
            0x3F => {self.ccf(); 1},
            //SCF
            0x37 => {self.scf(); 1},
            //NOP
            0x0 => {1},
            //HALT - power down cpu until interrupt occurs
            0x76 => {self.halted = true; 1},
            //STOP -halt cpu and lcd display until button pressed
            0x10 => {self.stopped = true; 1},
            //Make sure these two wait until after instruction is 
            //executed to change interrupt status
            //DI 
            0xF3 => {self.interrupts_enabled = false; 1},
            //EI
            0xFB => {self.interrupts_enabled = true; 1},
            //RLCA - rotate A left. old bit 7 to carry flag
            0x07 => {self.rlca(); 1},
            //RLA - rotate A left through Carry flag
            0x17 => {self.rla(); 1},
            //RRCA - rotate A right. old bit 0 to Carry flag
            0x0F => {self.rrca(); 1},
            //RRA - rotate A right through Carry flag
            0x1F => {self.rra(); 1},
            //JP nn
            0xC3 => {self.registers.pc = self.next_word(); 3},
            //JP to nn if coniditon is true
            0xC2 => {if !self.registers.check_zero(){self.registers.pc = self.next_word()}; 3},
            0xCA => {if self.registers.check_zero(){self.registers.pc = self.next_word()}; 3},
            0xD2 => {if !self.registers.check_carry(){self.registers.pc = self.next_word()}; 3},
            0xDA => {if self.registers.check_carry(){self.registers.pc = self.next_word()}; 3},
            //JP to address contained in HL
            0xE9 => {self.registers.pc = self.registers.hl(); 1},
            //JR n - add n to current address and jump to it
            0x18 => {self.registers.pc += self.next_byte() as u16; 2},
            //JR cc,n - add n to current address and jump if flag is set
            0x20 => {if !self.registers.check_zero(){self.registers.pc += self.next_byte() as u16}; 2},
            0x28 => {if self.registers.check_zero(){self.registers.pc += self.next_byte() as u16}; 2},
            0x30 => {if !self.registers.check_carry(){self.registers.pc += self.next_byte() as u16}; 2},
            0x38 => {if self.registers.check_carry(){self.registers.pc += self.next_byte() as u16}; 2},
            //Calls
            //Call nn, push address of next instruction onto stack, then jump to nn
            //Not sure about this one
            0xCD => {self.push_word(self.registers.pc + 1); self.registers.pc = self.next_word(); 3},
            //Call nn if condition is true
            0xC4 => {if !self.registers.check_zero(){self.push_word(self.registers.pc + 1); self.registers.pc = self.next_word(); } 3},
            0xCC => {if self.registers.check_zero(){self.push_word(self.registers.pc + 1); self.registers.pc = self.next_word(); } 3},
            0xD4 => {if !self.registers.check_carry(){self.push_word(self.registers.pc + 1); self.registers.pc = self.next_word(); } 3},
            0xDC => {if self.registers.check_carry(){self.push_word(self.registers.pc + 1); self.registers.pc = self.next_word(); } 3},
            //Restarts - push present address to stack, jump to $0000 + x
            0xC7 => {self.push_word(self.registers.pc); self.registers.pc = 0x00; 8},
            0xCF => {self.push_word(self.registers.pc); self.registers.pc = 0x08; 8},
            0xD7 => {self.push_word(self.registers.pc); self.registers.pc = 0x10; 8},
            0xDF => {self.push_word(self.registers.pc); self.registers.pc = 0x18; 8},
            0xE7 => {self.push_word(self.registers.pc); self.registers.pc = 0x20; 8},
            0xEF => {self.push_word(self.registers.pc); self.registers.pc = 0x28; 8},
            0xF7 => {self.push_word(self.registers.pc); self.registers.pc = 0x30; 8},
            0xFF => {self.push_word(self.registers.pc); self.registers.pc = 0x38; 8},
            //RET 
            0xC9 => {self.registers.pc = self.pop_word(); 2},
            //RET cc
            0xC0 => {if !self.registers.check_zero(){self.registers.pc = self.pop_word(); self.registers.pc = self.next_word(); } 2},
            0xC8 => {if self.registers.check_zero(){self.registers.pc = self.pop_word(); self.registers.pc = self.next_word(); } 2},
            0xD0 => {if !self.registers.check_carry(){self.registers.pc = self.pop_word(); self.registers.pc = self.next_word(); } 2},
            0xD8 => {if self.registers.check_carry(){self.registers.pc = self.pop_word(); self.registers.pc = self.next_word(); } 2},
            //RETI - pop two bytes and jump to address, enable interrupts
            0xD9 => {self.registers.pc = self.pop_word(); self.interrupts_enabled = true; 2},
            //CB
            //CHECK CYCLES FOR THIS ONE
            0xCB => {let byte = self.next_byte(); self.cb_decode(byte) + 1},
            _ => {println!("This opcode has not been implemented!"); 1}
        };
        cycles
    }

    fn rrca(&mut self) {
        let old_carry: u8 = if self.registers.check_carry() {
            1u8
        }
        else {
            0u8
        };
        let new_carry = self.registers.a & 1u8;
        self.registers.a = self.registers.a >> 1;
        self.registers.a |= old_carry << 7;
        self.registers.set_zero(0);
        self.registers.set_carry(new_carry);
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);
    }

    fn rra(&mut self) {
        let new_carry = self.registers.a & 1u8;
        self.registers.a = self.registers.a >> 1;
        self.registers.a |= new_carry << 7;
        self.registers.set_zero(0);
        self.registers.set_carry(new_carry);
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);
    }

    fn rlca(&mut self) {
        let old_carry: u8 = if self.registers.check_carry() {
            1u8
        }
        else {
            0u8
        };
        let new_carry = (self.registers.a & (1u8 << 7)) >> 7;
        self.registers.a = self.registers.a << 1;
        self.registers.a |= old_carry;
        self.registers.set_zero(0);
        self.registers.set_carry(new_carry);
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);
    }

    fn rla(&mut self) {
        let new_carry = self.registers.a & (1u8 << 7) >> 7;
        self.registers.a = self.registers.a << 1;
        self.registers.a |= new_carry;
        self.registers.set_zero(0);
        self.registers.set_carry(new_carry);
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);
    }

    fn rlc(&mut self, mut data: u8) -> u8 {
        let old_carry: u8 = if self.registers.check_carry() {
            1u8
        }
        else {
            0u8
        };
        let new_carry = data & (1u8 << 7) >> 7;
        data = data << 1;
        data |= old_carry;
        if data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_carry(new_carry);
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);
        data
    }

    fn rl(&mut self, mut data: u8) -> u8 {
        let new_carry = data & (1u8 << 7) >> 7;
        data = data << 1;
        data |= new_carry;
        if data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_carry(new_carry);
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);
        data
    }

    fn rrc(&mut self, mut data: u8) -> u8 {
        let old_carry: u8 = if self.registers.check_carry() {
            1u8
        }
        else {
            0u8
        };
        let new_carry = data & 1u8;
        data = data >> 1;
        data |= old_carry << 7;
        if data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_carry(new_carry);
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);
        data
    }

    fn rr(&mut self, mut data: u8) -> u8 {
        let new_carry = data & 1u8;
        data = data << 1;
        data |= new_carry << 7;
        if data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_carry(new_carry);
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);
        data
    }



    //set carry flag
    fn scf(&mut self) {
        self.registers.set_carry(1u8);
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);
    }

    //Complement carry flag, reset h and n flags
    fn ccf(&mut self) {
        let cf = self.registers.check_carry();
        if cf {
            self.registers.set_carry(0);
        }
        else {
            self.registers.set_carry(1);
        }
        self.registers.set_halfcarry(0);
        self.registers.set_addsub(0);

    }


    //complement A register
    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.set_halfcarry(1);
        self.registers.set_addsub(1);
    }

    fn swap(&mut self, data: u8) -> u8 {
        let lower = (data & 0xF0) >> 4;
        let upper = (data & 0x0F) << 4;
        let swapped: u8 = upper | lower;
        if swapped == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_addsub(0);
        self.registers.set_carry(0);
        self.registers.set_halfcarry(0);
        swapped
    }
    //shift n left into Carry, LSB set to 0
    fn sla(&mut self, mut data: u8) -> u8 {
        let new_carry = (data & (1u8 << 7)) >> 7;
        data = data << 1;
        if data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_carry(new_carry);
        self.registers.set_addsub(0);
        self.registers.set_halfcarry(0);
        data
    }

    //shift n right into Carry. MSB doesn't change
    fn sra(&mut self, mut data: u8) -> u8 {
        let msb = (data & (1u8 << 7)) >> 7;
        let new_carry = data & 1u8;
        data = data >> 1;
        data |= msb;
        if data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_carry(new_carry);
        self.registers.set_addsub(0);
        self.registers.set_halfcarry(0);
        data
    }

    //shift n right into Carry. MSB=0
    fn srl(&mut self, mut data: u8) -> u8 {
        let new_carry = data & 1u8;
        data = data >> 1;
        if data == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        }
        self.registers.set_carry(new_carry);
        self.registers.set_addsub(0);
        self.registers.set_halfcarry(0);
        data
    }


    fn cb_decode(&mut self, opcode: u8) -> u8 {
        let cycles = match opcode{
            //SWAP upper and lower nibbles of n
            0x37 => {self.registers.a = self.swap(self.registers.a); 2},
            0x30 => {self.registers.b = self.swap(self.registers.b); 2},
            0x31 => {self.registers.c = self.swap(self.registers.c); 2},
            0x32 => {self.registers.d = self.swap(self.registers.d); 2},
            0x33 => {self.registers.e = self.swap(self.registers.e); 2},
            0x34 => {self.registers.h = self.swap(self.registers.h); 2},
            0x35 => {self.registers.l = self.swap(self.registers.l); 2},
            0x36 => {let address = self.registers.hl(); let swapped = self.swap(self.memory.read_byte(address)); self.memory.write_byte(address, swapped); 4},
            //RLC n - rotate n left. old bit 7 to carry flag
            0x07 => {self.registers.a = self.rlc(self.registers.a); 2},
            0x00 => {self.registers.b = self.rlc(self.registers.b); 2},
            0x01 => {self.registers.c = self.rlc(self.registers.c); 2},
            0x02 => {self.registers.d = self.rlc(self.registers.d); 2},
            0x03 => {self.registers.e = self.rlc(self.registers.e); 2},
            0x04 => {self.registers.h = self.rlc(self.registers.h); 2},
            0x05 => {self.registers.l = self.rlc(self.registers.l); 2},
            0x06 => {let rlc = self.rlc(self.memory.read_byte(self.registers.hl())); self.memory.write_byte(self.registers.hl(), rlc); 4},
            //RL n - rotate n left through carry flag
            0x17 => {self.registers.a = self.rl(self.registers.a); 2},
            0x10 => {self.registers.b = self.rl(self.registers.b); 2},
            0x11 => {self.registers.c = self.rl(self.registers.c); 2},
            0x12 => {self.registers.d = self.rl(self.registers.d); 2},
            0x13 => {self.registers.e = self.rl(self.registers.e); 2},
            0x14 => {self.registers.h = self.rl(self.registers.h); 2},
            0x15 => {self.registers.l = self.rl(self.registers.l); 2},
            0x16 => {let rl = self.rl(self.memory.read_byte(self.registers.hl())); self.memory.write_byte(self.registers.hl(), rl); 4},
            //RRC n - rotate n right, old bit 0 to carry flag
            0x0F => {self.registers.a = self.rrc(self.registers.a); 2},
            0x08 => {self.registers.b = self.rrc(self.registers.b); 2},
            0x09 => {self.registers.c = self.rrc(self.registers.c); 2},
            0x0A => {self.registers.d = self.rrc(self.registers.d); 2},
            0x0B => {self.registers.e = self.rrc(self.registers.e); 2},
            0x0C => {self.registers.h = self.rrc(self.registers.h); 2},
            0x0D => {self.registers.l = self.rrc(self.registers.l); 2},
            0x0E => {let rrc = self.rrc(self.memory.read_byte(self.registers.hl())); self.memory.write_byte(self.registers.hl(), rrc); 4},
            //RR n - rotate n right through carry flag
            0x1F => {self.registers.a = self.rr(self.registers.a); 2},
            0x18 => {self.registers.b = self.rr(self.registers.b); 2},
            0x19 => {self.registers.c = self.rr(self.registers.c); 2},
            0x1A => {self.registers.d = self.rr(self.registers.d); 2},
            0x1B => {self.registers.e = self.rr(self.registers.e); 2},
            0x1C => {self.registers.h = self.rr(self.registers.h); 2},
            0x1D => {self.registers.l = self.rr(self.registers.l); 2},
            0x1E => {let rr = self.rr(self.memory.read_byte(self.registers.hl())); self.memory.write_byte(self.registers.hl(), rr); 4},
            //SLA n - shift n left into carry flag, LSB=0
            0x27 => {self.registers.a = self.sla(self.registers.a); 2},
            0x20 => {self.registers.b = self.sla(self.registers.b); 2},
            0x21 => {self.registers.c = self.sla(self.registers.c); 2},
            0x22 => {self.registers.d = self.sla(self.registers.d); 2},
            0x23 => {self.registers.e = self.sla(self.registers.e); 2},
            0x24 => {self.registers.h = self.sla(self.registers.h); 2},
            0x25 => {self.registers.l = self.sla(self.registers.l); 2},
            0x26 => {let sla = self.sla(self.memory.read_byte(self.registers.hl())); self.memory.write_byte(self.registers.hl(), sla); 4},
            //SRA n - shift n right into carry flag. MSB doesn't change
            0x2F => {self.registers.a = self.sra(self.registers.a); 2},
            0x28 => {self.registers.b = self.sra(self.registers.b); 2},
            0x29 => {self.registers.c = self.sra(self.registers.c); 2},
            0x2A => {self.registers.d = self.sra(self.registers.d); 2},
            0x2B => {self.registers.e = self.sra(self.registers.e); 2},
            0x2C => {self.registers.h = self.sra(self.registers.h); 2},
            0x2D => {self.registers.l = self.sra(self.registers.l); 2},
            0x2E => {let sra = self.sra(self.memory.read_byte(self.registers.hl())); self.memory.write_byte(self.registers.hl(), sra); 4},
            //SRA n - shift n right into carry flag. MSB=0
            0x3F => {self.registers.a = self.srl(self.registers.a); 2},
            0x38 => {self.registers.b = self.srl(self.registers.b); 2},
            0x39 => {self.registers.c = self.srl(self.registers.c); 2},
            0x3A => {self.registers.d = self.srl(self.registers.d); 2},
            0x3B => {self.registers.e = self.srl(self.registers.e); 2},
            0x3C => {self.registers.h = self.srl(self.registers.h); 2},
            0x3D => {self.registers.l = self.srl(self.registers.l); 2},
            0x3E => {let srl = self.srl(self.memory.read_byte(self.registers.hl())); self.memory.write_byte(self.registers.hl(), srl); 4},
            //Test bit 0
            0x40 => {self.check_bit(self.registers.b, 0); 2},
            0x41 => {self.check_bit(self.registers.c, 0); 2},
            0x42 => {self.check_bit(self.registers.d, 0); 2},
            0x43 => {self.check_bit(self.registers.e, 0); 2},
            0x44 => {self.check_bit(self.registers.h, 0); 2},
            0x45 => {self.check_bit(self.registers.l, 0); 2},
            0x46 => {self.check_bit(self.memory.read_byte(self.registers.hl()), 0); 4},
            0x47 => {self.check_bit(self.registers.a, 0); 2},
            //Test bit 1
            0x48 => {self.check_bit(self.registers.b, 1); 2},
            0x49 => {self.check_bit(self.registers.c, 1); 2},
            0x4A => {self.check_bit(self.registers.d, 1); 2},
            0x4B => {self.check_bit(self.registers.e, 1); 2},
            0x4C => {self.check_bit(self.registers.h, 1); 2},
            0x4D => {self.check_bit(self.registers.l, 1); 2},
            0x4E => {self.check_bit(self.memory.read_byte(self.registers.hl()), 1); 4},
            0x4F => {self.check_bit(self.registers.a, 1); 2},
            //Test bit 2
            0x50 => {self.check_bit(self.registers.b, 2); 2},
            0x51 => {self.check_bit(self.registers.c, 2); 2},
            0x52 => {self.check_bit(self.registers.d, 2); 2},
            0x53 => {self.check_bit(self.registers.e, 2); 2},
            0x54 => {self.check_bit(self.registers.h, 2); 2},
            0x55 => {self.check_bit(self.registers.l, 2); 2},
            0x56 => {self.check_bit(self.memory.read_byte(self.registers.hl()), 2); 4},
            0x57 => {self.check_bit(self.registers.a, 2); 2},            
            //Test bit 3
            0x58 => {self.check_bit(self.registers.b, 3); 2},
            0x59 => {self.check_bit(self.registers.c, 3); 2},
            0x5A => {self.check_bit(self.registers.d, 3); 2},
            0x5B => {self.check_bit(self.registers.e, 3); 2},
            0x5C => {self.check_bit(self.registers.h, 3); 2},
            0x5D => {self.check_bit(self.registers.l, 3); 2},
            0x5E => {self.check_bit(self.memory.read_byte(self.registers.hl()), 3); 4},
            0x5F => {self.check_bit(self.registers.a, 3); 2},
            //Test bit 4
            0x60 => {self.check_bit(self.registers.b, 4); 2},
            0x61 => {self.check_bit(self.registers.c, 4); 2},
            0x62 => {self.check_bit(self.registers.d, 4); 2},
            0x63 => {self.check_bit(self.registers.e, 4); 2},
            0x64 => {self.check_bit(self.registers.h, 4); 2},
            0x65 => {self.check_bit(self.registers.l, 4); 2},
            0x66 => {self.check_bit(self.memory.read_byte(self.registers.hl()), 4); 4},
            0x67 => {self.check_bit(self.registers.a, 4); 2},            
            //Test bit 5
            0x68 => {self.check_bit(self.registers.b, 5); 2},
            0x69 => {self.check_bit(self.registers.c, 5); 2},
            0x6A => {self.check_bit(self.registers.d, 5); 2},
            0x6B => {self.check_bit(self.registers.e, 5); 2},
            0x6C => {self.check_bit(self.registers.h, 5); 2},
            0x6D => {self.check_bit(self.registers.l, 5); 2},
            0x6E => {self.check_bit(self.memory.read_byte(self.registers.hl()), 5); 4},
            0x6F => {self.check_bit(self.registers.a, 5); 2},
            //Test bit 6
            0x70 => {self.check_bit(self.registers.b, 6); 2},
            0x71 => {self.check_bit(self.registers.c, 6); 2},
            0x72 => {self.check_bit(self.registers.d, 6); 2},
            0x73 => {self.check_bit(self.registers.e, 6); 2},
            0x74 => {self.check_bit(self.registers.h, 6); 2},
            0x75 => {self.check_bit(self.registers.l, 6); 2},
            0x76 => {self.check_bit(self.memory.read_byte(self.registers.hl()), 6); 4},
            0x77 => {self.check_bit(self.registers.a, 6); 2},            
            //Test bit 7
            0x78 => {self.check_bit(self.registers.b, 7); 2},
            0x79 => {self.check_bit(self.registers.c, 7); 2},
            0x7A => {self.check_bit(self.registers.d, 7); 2},
            0x7B => {self.check_bit(self.registers.e, 7); 2},
            0x7C => {self.check_bit(self.registers.h, 7); 2},
            0x7D => {self.check_bit(self.registers.l, 7); 2},
            0x7E => {self.check_bit(self.memory.read_byte(self.registers.hl()), 7); 4},
            0x7F => {self.check_bit(self.registers.a, 7); 2},
            //Reset bit 0
            0x80 => {self.registers.b &= !(1u8 << 0); 2},
            0x81 => {self.registers.c &= !(1u8 << 0); 2},
            0x82 => {self.registers.d &= !(1u8 << 0); 2},
            0x83 => {self.registers.e &= !(1u8 << 0); 2},
            0x84 => {self.registers.h &= !(1u8 << 0); 2},
            0x85 => {self.registers.l &= !(1u8 << 0); 2},
            0x86 => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl())  & !(1u8 << 0)); 4},
            0x87 => {self.registers.a &= !(1u8 << 0); 2},
            //Reset bit 1
            0x88 => {self.registers.b &= !(1u8 << 1); 2},
            0x89 => {self.registers.c &= !(1u8 << 1); 2},
            0x8A => {self.registers.d &= !(1u8 << 1); 2},
            0x8B => {self.registers.e &= !(1u8 << 1); 2},
            0x8C => {self.registers.h &= !(1u8 << 1); 2},
            0x8D => {self.registers.l &= !(1u8 << 1); 2},
            0x8E => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl())  & !(1u8 << 1)); 4},
            0x8F => {self.registers.a &= !(1u8 << 1); 2},
            //Reset bit 2
            0x90 => {self.registers.b &= !(1u8 << 2); 2},
            0x91 => {self.registers.c &= !(1u8 << 2); 2},
            0x92 => {self.registers.d &= !(1u8 << 2); 2},
            0x93 => {self.registers.e &= !(1u8 << 2); 2},
            0x94 => {self.registers.h &= !(1u8 << 2); 2},
            0x95 => {self.registers.l &= !(1u8 << 2); 2},
            0x96 => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl())  & !(1u8 << 2)); 4},
            0x97 => {self.registers.a &= !(1u8 << 2); 2},
            //Reset bit 3
            0x98 => {self.registers.b &= !(1u8 << 3); 2},
            0x99 => {self.registers.c &= !(1u8 << 3); 2},
            0x9A => {self.registers.d &= !(1u8 << 3); 2},
            0x9B => {self.registers.e &= !(1u8 << 3); 2},
            0x9C => {self.registers.h &= !(1u8 << 3); 2},
            0x9D => {self.registers.l &= !(1u8 << 3); 2},
            0x9E => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl())  & !(1u8 << 3)); 4},
            0x9F => {self.registers.a &= !(1u8 << 3); 2},
            //Reset bit 4
            0xA0 => {self.registers.b &= !(1u8 << 4); 2},
            0xA1 => {self.registers.c &= !(1u8 << 4); 2},
            0xA2 => {self.registers.d &= !(1u8 << 4); 2},
            0xA3 => {self.registers.e &= !(1u8 << 4); 2},
            0xA4 => {self.registers.h &= !(1u8 << 4); 2},
            0xA5 => {self.registers.l &= !(1u8 << 4); 2},
            0xA6 => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl())  & !(1u8 << 4)); 4},
            0xA7 => {self.registers.a &= !(1u8 << 4); 2},
            //Reset bit 5
            0xA8 => {self.registers.b &= !(1u8 << 5); 2},
            0xA9 => {self.registers.c &= !(1u8 << 5); 2},
            0xAA => {self.registers.d &= !(1u8 << 5); 2},
            0xAB => {self.registers.e &= !(1u8 << 5); 2},
            0xAC => {self.registers.h &= !(1u8 << 5); 2},
            0xAD => {self.registers.l &= !(1u8 << 5); 2},
            0xAE => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl())  & !(1u8 << 5)); 4},
            0xAF => {self.registers.a &= !(1u8 << 5); 2},
            //Reset bit 6
            0xB0 => {self.registers.b &= !(1u8 << 6); 2},
            0xB1 => {self.registers.c &= !(1u8 << 6); 2},
            0xB2 => {self.registers.d &= !(1u8 << 6); 2},
            0xB3 => {self.registers.e &= !(1u8 << 6); 2},
            0xB4 => {self.registers.h &= !(1u8 << 6); 2},
            0xB5 => {self.registers.l &= !(1u8 << 6); 2},
            0xB6 => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl())  & !(1u8 << 6)); 4},
            0xB7 => {self.registers.a &= !(1u8 << 6); 2},
            //Reset bit 7
            0xB8 => {self.registers.b &= !(1u8 << 7); 2},
            0xB9 => {self.registers.c &= !(1u8 << 7); 2},
            0xBA => {self.registers.d &= !(1u8 << 7); 2},
            0xBB => {self.registers.e &= !(1u8 << 7); 2},
            0xBC => {self.registers.h &= !(1u8 << 7); 2},
            0xBD => {self.registers.l &= !(1u8 << 7); 2},
            0xBE => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl()) & !(1u8 << 7)); 4},
            0xBF => {self.registers.a &= !(1u8 << 7); 2},
            //Set bit 0
            0xC0 => {self.registers.b |= 1u8 << 0; 2},
            0xC1 => {self.registers.c |= 1u8 << 0; 2},
            0xC2 => {self.registers.d |= 1u8 << 0; 2},
            0xC3 => {self.registers.e |= 1u8 << 0; 2},
            0xC4 => {self.registers.h |= 1u8 << 0; 2},
            0xC5 => {self.registers.l |= 1u8 << 0; 2},
            0xC6 => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl()) | (1u8 << 0)); 4},
            0xC7 => {self.registers.a |= 1u8 << 0; 2},
            //Set bit 1
            0xC8 => {self.registers.b |= 1u8 << 1; 2},
            0xC9 => {self.registers.c |= 1u8 << 1; 2},
            0xCA => {self.registers.d |= 1u8 << 1; 2},
            0xCB => {self.registers.e |= 1u8 << 1; 2},
            0xCC => {self.registers.h |= 1u8 << 1; 2},
            0xCD => {self.registers.l |= 1u8 << 1; 2},
            0xCE => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl()) | (1u8 << 1)); 4},
            0xCF => {self.registers.a |= 1u8 << 1; 2},
            //Set bit 2
            0xD0 => {self.registers.b |= 1u8 << 2; 2},
            0xD1 => {self.registers.c |= 1u8 << 2; 2},
            0xD2 => {self.registers.d |= 1u8 << 2; 2},
            0xD3 => {self.registers.e |= 1u8 << 2; 2},
            0xD4 => {self.registers.h |= 1u8 << 2; 2},
            0xD5 => {self.registers.l |= 1u8 << 2; 2},
            0xD6 => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl()) | (1u8 << 2)); 4},
            0xD7 => {self.registers.a |= 1u8 << 2; 2},
            //Set bit 3
            0xD8 => {self.registers.b |= 1u8 << 3; 2},
            0xD9 => {self.registers.c |= 1u8 << 3; 2},
            0xDA => {self.registers.d |= 1u8 << 3; 2},
            0xDB => {self.registers.e |= 1u8 << 3; 2},
            0xDC => {self.registers.h |= 1u8 << 3; 2},
            0xDD => {self.registers.l |= 1u8 << 3; 2},
            0xDE => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl()) | (1u8 << 3)); 4},
            0xDF => {self.registers.a |= 1u8 << 3; 2},
            //Set bit 4
            0xE0 => {self.registers.b |= 1u8 << 4; 2},
            0xE1 => {self.registers.c |= 1u8 << 4; 2},
            0xE2 => {self.registers.d |= 1u8 << 4; 2},
            0xE3 => {self.registers.e |= 1u8 << 4; 2},
            0xE4 => {self.registers.h |= 1u8 << 4; 2},
            0xE5 => {self.registers.l |= 1u8 << 4; 2},
            0xE6 => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl()) | (1u8 << 4)); 4},
            0xE7 => {self.registers.a |= 1u8 << 4; 2},
            //Set bit 5
            0xE8 => {self.registers.b |= 1u8 << 5; 2},
            0xE9 => {self.registers.c |= 1u8 << 5; 2},
            0xEA => {self.registers.d |= 1u8 << 5; 2},
            0xEB => {self.registers.e |= 1u8 << 5; 2},
            0xEC => {self.registers.h |= 1u8 << 5; 2},
            0xED => {self.registers.l |= 1u8 << 5; 2},
            0xEE => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl()) | (1u8 << 5)); 4},
            0xEF => {self.registers.a |= 1u8 << 5; 2},
            //Set bit 6
            0xF0 => {self.registers.b |= 1u8 << 6; 2},
            0xF1 => {self.registers.c |= 1u8 << 6; 2},
            0xF2 => {self.registers.d |= 1u8 << 6; 2},
            0xF3 => {self.registers.e |= 1u8 << 6; 2},
            0xF4 => {self.registers.h |= 1u8 << 6; 2},
            0xF5 => {self.registers.l |= 1u8 << 6; 2},
            0xF6 => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl()) | (1u8 << 6)); 4},
            0xF7 => {self.registers.a |= 1u8 << 6; 2},
            //Set bit 7
            0xF8 => {self.registers.b |= 1u8 << 7; 2},
            0xF9 => {self.registers.c |= 1u8 << 7; 2},
            0xFA => {self.registers.d |= 1u8 << 7; 2},
            0xFB => {self.registers.e |= 1u8 << 7; 2},
            0xFC => {self.registers.h |= 1u8 << 7; 2},
            0xFD => {self.registers.l |= 1u8 << 7; 2},
            0xFE => {self.memory.write_byte(self.registers.hl(), self.memory.read_byte(self.registers.hl()) | (1u8 << 7)); 4},
            0xFF => {self.registers.a |= 1u8 << 7; 2},
        };
        cycles
    }

    //check if bit is zero
    fn check_bit(&mut self, data: u8, bit: u8) {
        let result = data & (1u8 << bit);
        if result == 0 {
            self.registers.set_zero(1);
        }
        else {
            self.registers.set_zero(0);
        } 
        self.registers.set_addsub(0);
        self.registers.set_halfcarry(1);
    }




}