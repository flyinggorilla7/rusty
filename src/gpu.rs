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
    vram: [u8; 0x2000],
    render_mode: u8,
    pub render_mode_cycles: u32,
    pub lcd_control: LcdControl, //0xFF40
    pub scroll_y: u8, //0xFF42
    pub scroll_x: u8, //0xFF43
    pub window_y: u8, //0xFF4A
    pub window_x: u8, //0xFF4B
    pub scan_row: u8, //0xFF44
    pub lcd_stat: u8, //0xFF45 Holds value that will interrupt when matched with scan_row
    pub background_palette: u8, //0xFF47
    pub pixel_buffer: [u8; (160*144*3) as usize],
    pub vblank_flag: bool, //Tells emulator loop to update texture
    pub vblank_int: bool, //Interrupt flag for vblank
    pub lcd_stat_int: bool //Interrupt flag for LCD stat register
}

pub struct LcdControl {
    pub background: bool, //Background on when true
    pub sprites: bool, //Sprites on when true
    pub sprite_size: bool, //8x16 when true, 8x8 when false
    pub bg_map: bool, //1 when true, 0 when false
    pub bg_set: bool, //1 when true, 0 when false
    pub window: bool, //Enabled when true
    pub window_map: bool, //1 when true, 0 when false
    pub display: bool, //On when true, Off when false
}

impl LcdControl {
    pub fn new() -> LcdControl {
        LcdControl {
            background: false,
            sprites: false,
            sprite_size: false,
            bg_map: false,
            bg_set: false,
            window: false,
            window_map: false,
            display: false,
        }
    }
}

impl Vram {

    pub fn new() -> Vram {

        let blank_tile = [[PixelColor::Lightest; 8];8];
        let blank_set = [blank_tile; 384];
        Vram {
            tile_set: blank_set,
            vram: [0;0x2000],
            render_mode : 0,
            render_mode_cycles: 0,
            lcd_control: LcdControl::new(),
            scroll_y: 0,
            scroll_x: 0,
            window_y: 0,
            window_x: 0,
            scan_row: 0,
            lcd_stat: 0,
            background_palette: 0,
            pixel_buffer: [0; (160*144*3) as usize],
            vblank_flag: false,
            vblank_int: false,
            lcd_stat_int: false,
        }

    }



    pub fn step(&mut self,) {

        //Check for STAT interrupt
        if self.scan_row == self.lcd_stat {
            self.lcd_stat_int = true;
        }
        else {
            self.lcd_stat_int = false;
        }

        self.vblank_int = false;

        //All clock cycles divided by 4
        match self.render_mode {
            //H-Blank - CPU can access VRAM and OAM
            0 => {
                self.vblank_int = false;
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
                self.vblank_int = true;
                if self.render_mode_cycles >= 114 {
                    self.render_mode_cycles = 0;
                    self.scan_row += 1;

                    if self.scan_row == 154 {
                        self.vblank_flag = true;
                        self.scan_row = 0;
                        self.render_mode = 2;
                    }
                }

            }

            //LCD is reading OAM, CPU cannot access VRAM or OAM
            2 => {
                self.vblank_int = false;
                if self.render_mode_cycles >= 20 {
                    self.render_mode_cycles = 0;
                    self.render_mode = 3;
                }
            }

            //LCD is reading OAM and VRAM, CPU cannot access VRAM, OAM, or Color Palette
            3 => {
                self.vblank_int = false;
                if self.render_mode_cycles >= 43 {
                    self.render_mode_cycles = 0;
                    self.render_mode = 0;

                    //End of mode 3 is treated as end of current scan line
                    self.render_scan();
                }
            }

            _ => {panic!("Invalid Render Mode!")}
        }

        if self.scan_row == self.lcd_stat {self.lcd_stat_int = true;}

    }

    //Fix this so that wrapping works correctly
    pub fn render_scan(&mut self) {
        if self.lcd_control.display {

            let mut map_offset: u16;

            if self.lcd_control.bg_map {
                map_offset = 0x1C00;
            }
            else {
                map_offset = 0x1800;
            }

            //wrapping add
            //EX: scan_row = 0x05, scrolly = 0xFF
            //EX: tile_y = 0x04
            let row_offset = self.scan_row.wrapping_add(self.scroll_y);

            //Current Row In Tile Map
            map_offset += 32 * ((row_offset as u16 >> 3) as u16);
            
            //Next Row In Tile Map
            let map_edge: u16 = map_offset + 32;
            //tile_pixel_y is the row of the current tile_being rendered
            let tile_pixel_y = row_offset & 0x07;

            //line_offset is where scan will start in row
            //this is equivalent to the tile position in the row
            let mut line_offset = self.scroll_x >> 3;

            //tile_pixel_x is the column of the current tile being rendered
            let mut tile_pixel_x = self.scroll_x & 0x07;

            //Obtain index of next tile
            let mut tile_number: u16 = self.vram[(map_offset+line_offset as u16) as usize] as u16;

            //If second tile map is being used, indices are signed
            //Tile set is 384 Tiles
            if (self.lcd_control.bg_set) && (tile_number < 128) {
                tile_number += 256;
            }

            //Read tile from correct tile map
            let mut tile = self.tile_set[tile_number as usize];

            let mut pixel_buffer_offset: u32 = self.scan_row as u32 * 160 * 3;

            for _i in 0..160 {

                let color: u8 = match tile[tile_pixel_y as usize][tile_pixel_x as usize] {
                    PixelColor::Darkest => 0x00,
                    PixelColor::Dark => 0x4D,
                    PixelColor::Light => 0xB3,
                    PixelColor::Lightest => 0xFF,
                };

                self.pixel_buffer[pixel_buffer_offset as usize] = color;
                self.pixel_buffer[(pixel_buffer_offset+1) as usize] = color;
                self.pixel_buffer[(pixel_buffer_offset+2) as usize] = color;
                pixel_buffer_offset += 3;

                tile_pixel_x += 1;

                if tile_pixel_x == 8 {
                    //Start at the first pixel of the next tile
                    tile_pixel_x = 0;

                    //Increase line offset by 1, wrap around to the beginning of the line if greater than 255
                    line_offset = line_offset.wrapping_add(1);

                    //Reset line_offset if the scroll needs to wrap around
                    if map_edge == map_offset + line_offset as u16 {
                        line_offset = 0;
                    }

                    //Get new tile
                    tile_number = self.vram[(map_offset+line_offset as u16) as usize] as u16;
                    if (self.lcd_control.bg_set) && tile_number < 128 {
                        tile_number += 256;
                    }

                    tile = self.tile_set[tile_number as usize];
                }
            }
        }
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

}