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
    pub bios: [u8; 0x100],
    pub bios_flag: bool,
}

///home/porkchop/programming/rust/rustyroms/gb-test-roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb
impl Memory {
    pub fn new() -> Memory {

        let bios_path = Path::new("/home/porkchop/programming/rust/rustyroms/gameboy.gb");
        let bios_file = fs::read(bios_path).unwrap();
        println!("Bios Length: {}", bios_file.len());
        let mut bios_buffer: [u8; 0x100] = [0; 0x100];

        for (index, instruction) in bios_file.iter().enumerate() {
            let data = *instruction as u8;
            bios_buffer[index] = data;
        }

        let path = Path::new("/home/porkchop/programming/rust/rustyroms/gb-test-roms/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb");
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
            bios: bios_buffer,
            bios_flag: false,
        }
    }

    //Initialize registers to post bootrom values
    pub fn memory_setup(&mut self) {
        self.write_byte(0xFF00, 0xFF); //Joypad Input Register
        self.write_byte(0xFF05, 0x00); //TIMA
        self.write_byte(0xFF06, 0x00); //TMA
        self.write_byte(0xFF07, 0x00); //TAC
        self.write_byte(0xFF10, 0x80); //NR10
        self.write_byte(0xFF11, 0xBF); //NR11
        self.write_byte(0xFF12, 0xF3); //NR12
        self.write_byte(0xFF14, 0xBF); //NR14
        self.write_byte(0xFF16, 0x3F); //NR21
        self.write_byte(0xFF17, 0x00); //NR22
        self.write_byte(0xFF19, 0xBF); //NR24
        self.write_byte(0xFF1A, 0x7F); //NR30
        self.write_byte(0xFF1B, 0xFF); //NR31
        self.write_byte(0xFF1C, 0x9F); //NR32
        self.write_byte(0xFF1E, 0xBF); //NR33
        self.write_byte(0xFF20, 0xFF); //NR41
        self.write_byte(0xFF21, 0x00); //NR42
        self.write_byte(0xFF22, 0x00); //NR43
        self.write_byte(0xFF23, 0xBF); //NR30
        self.write_byte(0xFF24, 0x77); //NR50
        self.write_byte(0xFF25, 0xF3); //NR51
        self.write_byte(0xFF26, 0xF1); //NR52 -0xF0 for Super Gameboy
        self.write_byte(0xFF40, 0x91); //LCDC
        self.write_byte(0xFF42, 0x00); //SCY
        self.write_byte(0xFF43, 0x00); //SCX
        self.write_byte(0xFF45, 0x00); //LYC
        self.write_byte(0xFF47, 0xFC); //BGP
        self.write_byte(0xFF48, 0xFF); //OBP0
        self.write_byte(0xFF49, 0xFF); //OBP1
        self.write_byte(0xFF4A, 0x00); //WY
        self.write_byte(0xFF4B, 0x00); //WX
        self.write_byte(0xFFFF, 0x00); //IE
    }


    pub fn read_byte(&self, address: u16) -> u8 {

        if self.bios_flag && (address < 0x100) {
            return self.bios[address as usize]
        }

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
            0xFF00 => {self.memory[0xFF00] |= 0x0F} //Reset input buttons to unpressed state when input state changes
            //Temporary for Blaarg's Cpu tests
            0xFF01 => {   //Serial Transfer Control
                print!("{}", data as char);
                /*if data == 0x81 {
                    //println!("Wrote character to serial");
                    let result = self.memory[0xFF01];
                    print!("{:#04X}", result);
                }*/
            }
            0xFF42 => self.vram.scroll_y = data,
            0xFF43 => self.vram.scroll_x = data,
            0xFF44 => self.vram.scan_row = 0, //Writing to this register should always reset the row to zero
            0xFF4A => self.vram.window_y = data,
            0xFF4B => self.vram.window_x = data,
            0xFF40 => self.update_lcd_control(),
            0xFF45 => self.vram.lcd_stat = data,
            _ => (),
        }

    }

    //returns true if bit 4 is zero/input is set to direction
    pub fn input_status(&mut self) -> bool {
        let input_reg = self.read_byte(0xFF00);
        (input_reg & 0b0001_0000) == 0
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