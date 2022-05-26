use rand::random;
use std::fmt;

pub struct Tile {
    pub solid: bool,
}
impl fmt::Display for Tile {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self.solid { "\u{2588}" } else { " " })
    }
}

pub struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}
impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        let mut ret = Map {
            width,
            height,
            tiles: Vec::with_capacity((width * height) as usize),
        };
        // Where the Map is current randomly intialized
        for _ in 0..(width * height) as usize {
            ret.tiles.push(Tile { solid: random() });
        }
        ret
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
