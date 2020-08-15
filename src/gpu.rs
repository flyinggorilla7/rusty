//Each tile is 8x8 pixels and 16 bytes
//2 bytes are ORed together to get color of each line of pixels

//Background Tile Table starts at $8000-$8FFF (0-255) 
//Window Tile Table starts at $8800-$97FF (-128-127)

//Two 32x32 tile background maps
//Background - $9800-$9BFF - Numbered with unsigned numbers
//Window - $9C00-$9FFF - Numbered with signed numbers
//These tile background maps are organized as 32 rows of 32 bytes each

//FF40 is LCD Control Register

//So How Will We Do This?

const VRAM_START: u16 = 0x8000;
//const VRAM_END: u16   = 0x9FFF;


//Since each pixel is governed by two bits...
//There are 4 possible color shades
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PixelColor {
    Lightest, 
    Light,
    Dark,
    Darkest,  
}


//Each Row can be represented as an array of 8 PixelColor types
//A full tile would then just be an array of 8 rows 
type Tile = [[PixelColor; 8];8];

//The full tile set would then just be an array of 384 tiles
//We will worry about dividing the tile sets into window and
//background later in the code
type TileSet = [Tile; 384];

pub struct Vram {
    tile_set: TileSet, 
    pub tile_map1: [Tile; 1024],
    pub tile_map1_addresses: [u8; 1024],
    pub tile_map2_addresses: [u8; 1024],
    pub tile_map2: [Tile; 1024],
    vram: [u8; 0x2000],
    render_mode: u8,
    render_mode_cycles: u32,
    scan_row: u8,
    pixel_buffer: [u8; (256*256*3) as usize],
}

impl Vram {

    pub fn new() -> Vram {

        let blank_tile = [[PixelColor::Lightest; 8];8];
        let blank_set = [blank_tile; 384];
        Vram {
            tile_set: blank_set,
            tile_map1: [blank_tile; 1024],
            tile_map1_addresses : [0; 1024],
            tile_map2_addresses : [0; 1024],
            tile_map2: [blank_tile; 1024],
            vram: [0;0x2000],
            render_mode : 0,
            render_mode_cycles: 0,
            scan_row: 0,
            pixel_buffer: [0; (256*256*3) as usize],
        }

    }

    pub fn step(&mut self,) {

        //All clock cycles divided by 4
        match self.render_mode {
            //H-Blank - CPU can access VRAM and OAM
            0 => {
                if self.render_mode_cycles >= 51 {
                    self.render_mode_cycles = 0;
                    self.scan_row += 1;

                    if self.scan_row == 144 {
                        self.render_mode = 1;
                        //Write pixel buffer to screen
                    }
                    else {
                        self.render_mode = 2;
                    }
                }
            }

            //V-Blank - CPU can access VRAM and OAM
            1 => {
                if self.render_mode_cycles >= 114 {
                    self.render_mode_cycles = 0;
                    self.scan_row += 1;

                    if self.scan_row == 154 {
                        self.scan_row = 0;
                        self.render_mode = 2;
                    } 
                }

            }

            //LCD is reading OAM, CPU cannot access VRAM or OAM
            2 => {
                if self.render_mode_cycles >= 20 {
                    self.render_mode_cycles = 0;
                    self.render_mode = 3;
                }
            }

            //LCD is reading OAM and VRAM, CPU cannot access VRAM, OAM, or Color Palette
            3 => {
                if self.render_mode_cycles >= 43 {
                    self.render_mode_cycles = 0;
                    self.render_mode = 0;

                    //End of mode 3 is treated as end of current scan line
                    self.render_scan();
                }
            }

            .. => panic!("Invalid Render Mode!");
        }
    }

    pub fn render_scan(&mut self) {
        if lcd_display_enable {

        }
    }

    pub fn update_tile_map(&mut self, mut address: u16, data: u8) {
        if address >= VRAM_START {
            address -= VRAM_START;
        }
        match address {
            //Each of these ranges is 1024 tiles (Or 32 * 32)
            0x1800..=0x1BFF => {
                self.tile_map1_addresses[(address - 0x1800) as usize] = data;
                self.tile_map1[(address - 0x1800) as usize] = self.tile_set[data as usize];
                
            }
            0x1C00..=0x1FFF => {
                self.tile_map2[(address - 0x1C00) as usize] = self.tile_set[(data + 128) as usize];

            }

            _ => {println!("Invalid Tile Map Update");}
        };
    }

    //Read Byte in VRAM
    pub fn read_byte(&self, mut address: u16) -> u8 {
        if address >= VRAM_START {
            address -= VRAM_START;
        }
        self.vram[address as usize]
    }

    //Write Byte in VRAM
    pub fn write_byte(&mut self, mut address: u16, data: u8) {
        if address >= VRAM_START {
            address -= VRAM_START;
        }
        self.vram[address as usize] = data;
        //println!("VRAM write {:#04X} to address: {:#04X}", data, address);
        if address < 0x1800 {
            self.update_tile(address, data);
            self.refresh_tile_map();
        }
        else {
            self.update_tile_map(address, data);
        }
    }

    //Used to refresh tile map for any tiles that got modified
    pub fn refresh_tile_map(&mut self) {
        for (index, address) in self.tile_map1_addresses.iter().enumerate() {
            self.tile_map1[index] = self.tile_set[*address as usize];
        }
    } 

    //Obtain corresponsing number of tile 
    pub fn tile_number(mut address: u16) -> u16 {
        if address >= 0x8000 {
            address -= VRAM_START;
        }
        address / 16
    }

    pub fn tile_row(address: u16) -> u16 {
        (address & 0x000F) / 2
    }

    pub fn update_tile(&mut self, address: u16,  data: u8) {
        //For even address, we want to OR the data with address + 1
        //For odd address, we want to OR the data with address - 1
        let tile_number = Vram::tile_number(address) as usize;
        let tile_row = Vram::tile_row(address) as usize;
        let even;

        if address % 2 == 1 {
           even = false;
        }
        else {
            even = true;
        }
        //tile_set[x] -> tile number
        //tile_set[x][y] -> row
        //tile_set[x][y][z] -> pixel
        let first;
        let second;
        if even {
            first = data;
            second = self.read_byte(address + 1);
        }
        else {
            first = self.read_byte(address - 1);
            second = data;
        }
        let mut pixel_mask = 1u8 << 7;
        for pixel in 0..8 {
            let first_mask = first & pixel_mask;
            let second_mask = second & pixel_mask;
            match (first_mask==0, second_mask==0) {
                (true, true) => self.tile_set[tile_number as usize][tile_row as usize][pixel as usize] = PixelColor::Lightest,
                (true, false) => self.tile_set[tile_number as usize][tile_row as usize][pixel as usize] = PixelColor::Light,
                (false, true) => self.tile_set[tile_number as usize][tile_row as usize][pixel as usize] = PixelColor::Dark,
                (false, false) => self.tile_set[tile_number as usize][tile_row as usize][pixel as usize] = PixelColor::Darkest,
            }
            pixel_mask >>= 1;
        }
    }

}


#[cfg(test)]
mod tests {
    
    use super::*;


    #[test]
    fn test_tile_number() {        
        assert_eq!(Vram::tile_number(0x800F - VRAM_START),0);
        assert_eq!(Vram::tile_number(0x97F3 - VRAM_START),383);
        assert_eq!(Vram::tile_number(0x8FF5 - VRAM_START),255);
    }

    #[test]
    fn test_tile_row() {
        assert_eq!(Vram::tile_row(0x800F - VRAM_START), 7);
        assert_eq!(Vram::tile_row(0x97F3 - VRAM_START), 1);
        assert_eq!(Vram::tile_row(0x8FF5 - VRAM_START), 2);
    }

    #[test]
    fn test_update_tile() {
        let mut vram = Vram::new();
        vram.write_byte(0x8000, 0xFF);
        vram.write_byte(0x801E, 0xFF);
        vram.write_byte(0x801F, 0xFF);
        assert_eq!(vram.tile_set[0][0][0], PixelColor::Dark);
        assert_eq!(vram.tile_set[1][7], [PixelColor::Darkest, PixelColor::Darkest, PixelColor::Darkest, PixelColor::Darkest,
            PixelColor::Darkest,PixelColor::Darkest,PixelColor::Darkest,PixelColor::Darkest]);     
    }

    #[test]
    fn test_update_tile_map() {
        let mut vram = Vram::new();
        vram.write_byte(0x801E, 0xFF);
        vram.write_byte(0x801F, 0xFF);
        //Put Tile 1 in Position 0 of Tile Map
        vram.write_byte(0x1800, 0x01);
        assert_eq!(vram.tile_map1[0], vram.tile_set[1]);

        //This test will check to see if the tile_map will update automatically after a tile is updated
        vram.write_byte(0x801E, 0x50);
        vram.write_byte(0x801F, 0x50);
        assert_eq!(vram.tile_map1[0], vram.tile_set[1]);
    }

}