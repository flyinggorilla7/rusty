pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x11,
            b: 0x00,
            c: 0x00,
            d: 0xFF,
            e: 0x56,
            f: 0x80,
            h: 0x00,
            l: 0x0D,
            //Make sure to put pc back to 100
            pc: 0x0100,
            sp: 0xFFFE,
        }
    }

    pub fn af(&self) -> u16 {
        ((self.a as u16) << 8) | self.f as u16
    }

    pub fn set_af(&mut self, data: u16) {
        self.a = ((data & 0xFF00) >> 8) as u8;
        self.f = (data & 0x00FF) as u8;
    }

    pub fn bc(&self) -> u16 {
        ((self.b as u16) << 8) | self.c as u16
    }

    pub fn set_bc(&mut self, data: u16) {
        self.b = ((data & 0xFF00) >> 8) as u8;
        self.c = (data & 0x00FF) as u8;
    }

    pub fn de(&self) -> u16 {
        ((self.d as u16) << 8) | self.e as u16
    } 

    pub fn set_de(&mut self, data: u16) {
        self.d = ((data & 0xFF00) >> 8) as u8;
        self.e = (data & 0x00FF) as u8;
    }

    pub fn hl(&self) -> u16 {
        ((self.h as u16) << 8) | self.l as u16
    }

    pub fn set_hl(&mut self, data: u16) {
        self.h = ((data & 0xFF00) >> 8) as u8;
        self.l = (data & 0x00FF) as u8; 
    }

    //Flag Register Layout
    //7 - Zero Flag
    //6 - Add/Sub Flag
    //5 - Half Carry Flag
    //4 - Carry Flag
    //3-0 Not used (Always Zero)
    pub fn set_zero(&mut self, value: u8) {
        if value == 1 {
            self.f |= 1u8 << 7;
        }
        else {
            self.f &= !(1u8 << 7);
        }
    }

    pub fn check_zero(&self) -> bool {
        (self.f & (1u8 << 7)) != 0 
    }

    pub fn set_addsub(&mut self, value: u8) {
        if value == 1{
            self.f |= 1u8 << 6;
        }
        else {
            self.f &= !(1u8 << 6);
        }
    }

    pub fn check_addsub(&self) -> bool {
        (self.f & (1u8 << 6)) != 0
    }

    pub fn set_halfcarry(&mut self, value: u8) {
        if value == 1{
            self.f |= 1u8 << 5;
        }
        else {
            self.f &= !(1u8 << 5);
        }
    }

    pub fn check_halfcarry(&self) -> bool {
        (self.f & (1u8 << 5)) != 0
    }

    pub fn set_carry(&mut self, value: u8) {
        if value == 1 {
            self.f |= 1u8 << 4;
        }
        else {
            self.f &= !(1u8 << 4);
        }
    }

    pub fn check_carry(&self) -> bool {
        (self.f & (1u8 << 4)) != 0
    }

}

