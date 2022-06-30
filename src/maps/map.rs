use super::tile::Tile;
use std::fmt;

#[derive(Default)]
pub struct Map {
    pub width: u32,
    pub height: u32,
    tiles: Vec<Tile>,
}
impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        Map {
            width,
            height,
            tiles: vec![Tile { wall: true }; (width * height) as usize],
            //..Default::default()
        }
    }
    // Helper to convert from 2d coordinate into 1d Vec index
    fn xy_to_idx(&self, x: u32, y: u32) -> usize {
        let index = (y * self.width + x) as usize;
        // Putting it here so I don't have to keep doing it wherever the function is used
        assert!(index < self.tiles.len());
        index
    }
    pub fn get_tile(&self, x: u32, y: u32) -> Option<&Tile> {
        let index = self.xy_to_idx(x, y);
        self.tiles.get(index)
    }
    pub fn get_tile_mut(&mut self, x: u32, y: u32) -> Option<&mut Tile> {
        let index = self.xy_to_idx(x, y);
        self.tiles.get_mut(index)
    }
    // Allows negative numbers and numbers larger than the size allows and wraps around
    pub fn get_tile_modulo(&self, x: i32, y: i32) -> Option<&Tile> {
        let mod_x = x.rem_euclid(self.width as i32) as u32;
        let mod_y = y.rem_euclid(self.height as i32) as u32;
        self.get_tile(mod_x, mod_y)
    }
    // Allows negative numbers and numbers larger than the size allows and wraps around
    pub fn get_tile_modulo_mut(&mut self, x: i32, y: i32) -> Option<&mut Tile> {
        let mod_x = x.rem_euclid(self.width as i32) as u32;
        let mod_y = y.rem_euclid(self.height as i32) as u32;
        self.get_tile_mut(mod_x, mod_y)
    }
}
impl fmt::Display for Map {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let result = self.get_tile(x, y);
                match result {
                    Some(t) => write!(f, "{}", t)?,
                    None => write!(f, "X")?,
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}
