pub struct Memory {
    //Rom bank 0 -> 0000-3FFF
    //Rom bank 1 -> 4000-7FFF
    //VRAM -> 8000-9FFF
    //External Ram -> A000-BFFF
    //Work RAM bank 0 -> C000-CFFF
    //Work RAM bank 1 -> D000-DFFF
    //Typically not used -> E000-FDFF
    //Spirte Attribute Table -> FE00-FE9F
    //Not Usable -> FEA0-FEFF
    //IO Ports -> FF00-FF7F
    //High RAM -> FF80-FFFE
    //Interrupt Enable Register -> FFFF
    pub rom: [u8; 0x7FFF],
    pub vram: [u8; 0x1FFF],
    pub memory: [u8; 0xFFFF],

}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom: [1u8; 0x7FFF],
            vram: [1u8; 0x1FFF],
            memory: [1u8; 0xFFFF],
        }
    }


    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    //CHECK ENDIANESS, edit... might be ok now
    pub fn read_word(&self, address: u16) -> u16 {
        let lower: u8 = self.memory[address as usize];
        let upper: u8 = self.memory[(address+1) as usize];
        ((upper as u16) << 8) | (lower as u16) 
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
    }

    pub fn inc_memory_byte(&mut self, address: u16) {
        self.memory[address as usize] += 1;
    }

    pub fn dec_memory_byte(&mut self, address: u16) {
        self.memory[address as usize] -= 1;
    }

    //Check endianess, I think this one is good though
    pub fn write_word(&mut self, address: u16, data: u16) {
        let upper: u8 = ((data & 0xFF00) >> 8) as u8;
        let lower: u8 = (data & 0x00FF) as u8;
        self.memory[address as usize] = lower;
        self.memory[(address+1) as usize] = upper;
    }
}