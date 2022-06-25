use std::fmt;

#[derive(Copy, Clone)]
pub struct Tile {
    pub wall: bool,
}
impl fmt::Display for Tile {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self.wall { "\u{2588}" } else { " " })
    }
}
