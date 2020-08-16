use std::fs;
use std::path::Path;
use crate::gpu::Vram;

    //Rom bank 0 -> 0000-3FFF
    //Rom bank 1 -> 4000-7FFF
    //VRAM -> 8000-9FFF
    //External Ram -> A000-BFFF
    //Work RAM bank 0 -> C000-CFFF
    //Work RAM bank 1 -> D000-DFFF
    //Typically not used -> E000-FDFF
    //Sprite Attribute Table -> FE00-FE9F
    //Not Usable -> FEA0-FEFF
    //IO Ports -> FF00-FF7F
    //High RAM -> FF80-FFFE
    //Interrupt Enable Register -> FFFF
pub struct Memory {
    pub rom: [u8; 0x8000],
    pub vram: Vram,
    pub memory: [u8; 65536],
}

///home/porkchop/programming/rust/rustyroms/gb-test-roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb
impl Memory {
    pub fn new() -> Memory {
        let path = Path::new("/home/porkchop/programming/rust/rustyroms/drmario.gb");
        let file = fs::read(path).unwrap();
        println!("File Length: {}", file.len());
        let mut buffer: [u8; 65536] = [0; 65536];
        
        for (index,instruction) in file.iter().enumerate() {
            let data = *instruction as u8;
            buffer[index] = data;
        }
        println!("Cartridge Type: {}", buffer[0x147]);
        println!("ROM Size: {}", buffer[0x148]);
        println!("RAM Size: {}", buffer[0x149]);
        Memory {
            rom: [1u8; 0x8000],
            vram: Vram::new(),
            memory: buffer,
        }
    }


    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.memory[address as usize],
            0x8000..=0x9FFF => self.vram.read_byte(address),
            0xFF42 => self.vram.scroll_y,
            0xFF43 => self.vram.scroll_x,
            0xFF44 => self.vram.scan_row,
            0xFF4A => self.vram.window_y,
            0xFF4B => self.vram.window_x,
            _ => self.memory[address as usize],

        }

    }

    pub fn read_word(&self, address: u16) -> u16 {
        
        let lower: u8 = self.read_byte(address);
        let upper: u8 = self.read_byte(address + 1);

        ((upper as u16) << 8) | (lower as u16)
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        self.memory[address as usize] = data;
        match address {
            0x0000..=0x7FFF => self.memory[address as usize] = data,
            0x8000..=0x9FFF => self.vram.write_byte(address, data),
            0xFF42 => self.vram.scroll_y = data,
            0xFF43 => self.vram.scroll_x = data,
            0xFF44 => self.vram.scan_row = 0, //Writing to this register should always reset the row to zero
            0xFF4A => self.vram.window_y = data,
            0xFF4B => self.vram.window_x = data,
            0xFF40 => self.update_lcd_control(),
            _ => (),
        }

    }

    pub fn update_lcd_control(&mut self) {
        let data = self.memory[0xFF40];
        let bit_mask: u8 = 0b1000_0000;

        self.vram.lcd_control.display = (data & bit_mask) > 0;
        self.vram.lcd_control.window_map = (data & (bit_mask >> 1)) > 0;
        self.vram.lcd_control.window = (data & (bit_mask >> 2)) > 0;
        self.vram.lcd_control.bg_set = (data & (bit_mask >> 3)) > 0;
        self.vram.lcd_control.bg_map = (data & (bit_mask >> 4)) > 0;
        self.vram.lcd_control.sprite_size = (data & (bit_mask >> 5)) > 0;
        self.vram.lcd_control.sprites = (data & (bit_mask >> 6)) > 0;
        self.vram.lcd_control.background = (data & (bit_mask >> 7)) > 0;

    }

    pub fn write_word(&mut self, address: u16, data: u16) {
        let upper: u8 = ((data & 0xFF00) >> 8) as u8;
        let lower: u8 = (data & 0x00FF) as u8;
        
        self.write_byte(address, lower);
        self.write_byte(address+1, upper);
        //println!("Wrote {:#x} to address {:#x}", lower, address);
        //println!("Wrote {:#x} to address {:#x}", upper, address+1);

    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_word() {
        let mut memory = Memory::new();
        memory.write_byte(0x2000, 0xBA);
        memory.write_byte(0x2001, 0xDC);
        assert_eq!(memory.read_word(0x2000), 0xDCBA);
    }

    #[test]
    fn test_write_word() {
        let mut memory = Memory::new();
        memory.write_word(0x6FF0, 0xDCBA);
        assert_eq!(memory.memory[0x6FF0], 0xBA);
        assert_eq!(memory.memory[0x6FF1], 0xDC);
    }

    #[test]
    fn test_lcd_control_update() {
        let mut memory = Memory::new();
        memory.write_byte(0xFF40, 0x82);
        assert_eq!(memory.vram.lcd_control.display, true);
        assert_eq!(memory.vram.lcd_control.window_map, false);
        assert_eq!(memory.vram.lcd_control.sprites, true);
    }




}