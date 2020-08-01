//Each tile is 8x8 pixels and 16 bytes
//2 bytes are ORed together to get color of each line of pixels

//Background Tile Table starts at $8000-$8FFF (0-255) 
//Window Tile Table starts at $8800-$97FF (-128-127)

//Two 32x32 tile background maps
//Background - $9800-$9BFF - Numbered with unsigned numbers
//Window - $9C00-$9FFF - Numbered with signed numbers
//These tile background maps are organized as 32 rows of 32 bytes each

//So How Will We Do This?

const VRAM_START: u16 = 0x8000;
const VRAM_END: u16   = 0x9FFF;


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
/*pub struct TileRow {
    pixels: [PixelColor; 8]
}

pub struct Tile {
    tile: [TileRow; 8]
}
*/
//The full tile set would then just be an array of 384 tiles
//We will worry about dividing the tile sets into window and
//background later in the code
type TileSet = [Tile; 384];

/*pub struct TileSet {
    tile_set : [Tile; 384]
}
*/

pub struct Vram {
    tile_set: TileSet, 
    tile_map1: [u16; 256],
    tile_map2: [u16; 256],
    vram: [u8; 0x1FFF],
}

impl Vram {

    pub fn new() -> Vram {
        /*let blank_row = TileRow {
            pixels: [PixelColor::Lightest; 8]
        };
        let blank_tile = Tile {
            tile: [blank_row; 8]
        };
        let blank_set = TileSet {
            tile_set : [blank_tile; 384]
        };*/
        let blank_tile = [[PixelColor::Lightest; 8];8];
        let blank_set = [blank_tile; 384];
        Vram {
            tile_set: blank_set,
            tile_map1: [0; 256],
            tile_map2: [0;256],
            vram: [0;0x1FFF],
        }

    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.vram[address as usize]
    }

    pub fn update_tile_map1(&mut self) {

    }

    pub fn update_tile_map2(&mut self) {

    }

    pub fn write_byte(&mut self, address: u16, data: u8) {
        let relative_address = address - VRAM_START;
        self.update_tile(relative_address, data);
        /*match relative_address {
            0x0000..0x1FFF => {self.update_tile(relative_address, data)},
            0x9800..0x9BFF => {self.update_tile_map1},
            0x9C00..0x9FFF

        }*/
    }

    //Obtain corresponsing number of tile 
    pub fn tile_number(address: u16) -> u16 {
        //Example address:800F
        address / 16
    }

    pub fn tile_row(address: u16) -> u16 {
        address & 0x000F
    }

    pub fn update_tile(&mut self, address: u16,  data: u8) {
        //For even address, we want to OR the data with address + 1
        //For odd address, we want to OR the data with address - 1
        let tile_number = Vram::tile_number(address) as usize;
        let tile_row = Vram::tile_row(address) as usize;
        let even;

        self.vram[address as usize] = data;

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
        for pixel in 0..7 {
            pixel_mask >>= pixel;
            let first_mask = first & pixel_mask;
            let second_mask = second & pixel_mask;
            match (first_mask==0, second_mask==0) {
                (true, true) => self.tile_set[tile_number as usize][tile_row as usize][pixel as usize] = PixelColor::Lightest,
                (true, false) => self.tile_set[tile_number as usize][tile_row as usize][pixel as usize] = PixelColor::Light,
                (false, true) => self.tile_set[tile_number as usize][tile_row as usize][pixel as usize] = PixelColor::Dark,
                (false, false) => self.tile_set[tile_number as usize][tile_row as usize][pixel as usize] = PixelColor::Darkest,
            }
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
        assert_eq!(Vram::tile_row(0x800F - VRAM_START),15);
        assert_eq!(Vram::tile_row(0x97F3 - VRAM_START), 3);
        assert_eq!(Vram::tile_row(0x8FF5 - VRAM_START), 5);
    }

    #[test]
    fn test_update_tile() {
        let mut vram = Vram::new();
        vram.write_byte(0x8000, 0xFF);
        println!("{:#b}", vram.vram[0x0000]);
        println!("{:#b}", vram.vram[0x0001]);
        println!("{:?}", vram.tile_set[0][0]);
        assert_eq!(vram.tile_set[0][0][0], PixelColor::Dark);
    }

}