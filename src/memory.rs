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
        if (address < 0x8000) || (address > 0x9FFF) {
            //println!("Read {:#x} at address {:#x}", self.memory[address as usize], address);
            self.memory[address as usize]
        }
        else {
            //println!("Read {:#x} at address {:#x}", self.vram.read_byte(address), address);
            self.vram.read_byte(address)
        }
    }

    //CHECK ENDIANESS, edit... might be ok now
    pub fn read_word(&self, address: u16) -> u16 {
        if (address < 0x8000) || (address > 0x9FFF) {
            //println!("Read Lower {:#x} at address {:#x}", self.memory[address as usize], address);
            //println!("Read Upper {:#x} at address {:#x}", self.memory[(address+1) as usize], address+1);

            let lower: u8 = self.memory[address as usize];
            let upper: u8 = self.memory[(address+1) as usize];
            ((upper as u16) << 8) | (lower as u16)
        }
        else {
            let lower: u8 = self.vram.read_byte(address);
            let upper: u8 = self.vram.read_byte(address+1);
            ((upper as u16) << 8) | (lower as u16)
        } 
    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        if (address < 0x8000) || (address > 0x9FFF) {
            self.memory[address as usize] = data;
        }
        else {
            self.vram.write_byte(address, data)
        }
        //println!("Wrote {:#x} to address {:#x}", data, address);
    }

    //Check endianess, I think this one is good though
    pub fn write_word(&mut self, address: u16, data: u16) {
        let upper: u8 = ((data & 0xFF00) >> 8) as u8;
        let lower: u8 = (data & 0x00FF) as u8;
        if (address < 0x8000) || (address > 0x9FFF) {
            self.memory[address as usize] = lower;
            self.memory[(address+1) as usize] = upper;
        }
        else {
            self.vram.write_byte(address, lower);
            self.vram.write_byte(address+1, upper);
        }
        //println!("Wrote {:#x} to address {:#x}", lower, address);
        //println!("Wrote {:#x} to address {:#x}", upper, address+1);

    }

    //0xFF40 - Bit 7
    pub fn lcd_display_enable(&self) -> bool {
        self.memory[0xFF40] & (1u8 << 7) != 0
    }

    //0xFF40 - Bit 6: false -> 9800, true -> 9C00
    pub fn window_tile_display(&self) -> bool {
        self.memory[0xFF40] & (1u8 << 6) != 0
    }

    //0xFF40 Bit 5: false -> window display disabled
    pub fn window_display_enable(&self) -> bool {
        self.memory[0xFF40] & (1u8 << 5) != 0
    }

    //0xFF40 Bit 4: false -> 8800-97FF selected
    pub fn tile_data_select(&self) -> bool {
        self.memory[0xFF40] & (1u8 << 4) != 0
    }

    //0xFF40 Bit 3: false -> 9800 - 9BFF
    pub fn bg_tile_display(&self) -> bool {
        self.memory[0xFF40] & (1u8 << 3) != 0
    }

    //0xFF40 Bit 2: false -> 8x8 sprites
    pub fn sprite_size(&self) -> bool {
        self.memory[0xFF40] & (1u8 << 2) != 0
    }

    //0xFF40 Bit 1: false -> sprite disabled
    pub fn sprite_enable(&self) -> bool {
        self.memory[0xFF40] & (1u8 << 1) != 0
    }

    //0xFF40 Bit 0: false -> BG display disabled
    pub fn bg_display_enable(&self) -> bool {
        self.memory[0xFF40] & (1u8 << 0) != 0
    }

    //Specifies position in BG pixels map to display at upper left
    pub fn scrolly(&self) -> u8 {
        self.memory[0xFF42]
    }
    pub fn scrollx(&self) -> u8 {
        self.memory[0xFF43]
    }

    pub fn set_ly(&mut self, row: u8) {
        self.write_byte(0xFF44, row);
    }

    pub fn ly(&self) -> u8 {
        self.memory[0xFF44]
    }

    //Specifies position in Windows map to display at upper left
    pub fn windowy(&self) -> u8 {
        self.memory[0xFF4A]
    }
    pub fn windowx(&self) -> u8 {
        self.memory[0xFF4B]
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcd_display_enable() {
        let mut memory = Memory::new();
        memory.write_byte(0xFF40, 0xFF);
        assert_eq!(memory.lcd_display_enable(), true);
        memory.write_byte(0xFF40, 0x00);
        assert_eq!(memory.lcd_display_enable(), false);
    }
    
    #[test]
    fn test_window_tile_map_display() {
        let mut memory = Memory::new();
        memory.write_byte(0xFF40, 0xFF);
        assert_eq!(memory.window_tile_display(), true);
        memory.write_byte(0xFF40, 0x00);
        assert_eq!(memory.window_tile_display(), false);
    }

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




}