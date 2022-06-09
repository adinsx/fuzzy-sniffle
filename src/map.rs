//#![allow(dead_code)]
//#![allow(unused_variables)]
use rand::{distributions::WeightedIndex, prelude::*, Rng};
use std::{cmp, fmt};

#[derive(Copy, Clone)]

pub struct Tile {
    pub solid: bool,
}
impl fmt::Display for Tile {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self.solid { "\u{2588}" } else { " " })
    }
}

#[derive(Default)]
pub struct Map {
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}
impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        Map {
            width,
            height,
            tiles: vec![Tile { solid: true }; (width * height) as usize],
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

pub struct CaveMap {
    pub map: Map,
    edge_repulsion_dist: u32,
    //min_treasures: u32,
    //max_treasures: u32,
    coverage: f32,
}
impl CaveMap {
    pub fn new(width: u32, height: u32) -> CaveMap {
        let mut ret = CaveMap {
            map: Map::new(width, height),
            edge_repulsion_dist: 10,
            coverage: 0.3,
        };
        ret.gen();
        ret
    }
    // Move away from walls more as you get closer to them
    fn default_move_weight(&self, relevant_coord: u32, relevant_size: u32) -> f32 {
        if relevant_coord < self.edge_repulsion_dist {
            1f32 - (self.edge_repulsion_dist - relevant_coord) as f32
                / (self.edge_repulsion_dist - 1) as f32
        } else if relevant_coord > relevant_size - self.edge_repulsion_dist - 1 {
            1f32 + (relevant_coord + self.edge_repulsion_dist + 1 - relevant_size) as f32
                / (self.edge_repulsion_dist - 1) as f32
        } else {
            1f32
        }
    }
    fn default_move_left_weight(&self, x: u32) -> f32 {
        self.default_move_weight(x, self.map.width)
    }
    fn default_move_right_weight(&self, x: u32) -> f32 {
        self.default_move_weight(self.map.width - x - 1, self.map.width)
    }
    fn default_move_up_weight(&self, y: u32) -> f32 {
        self.default_move_weight(y, self.map.height)
    }
    fn default_move_down_weight(&self, y: u32) -> f32 {
        self.default_move_weight(self.map.height - y - 1, self.map.height)
    }
    fn gen(&mut self) {
        let num_floors_to_get =
            (self.coverage * self.map.width as f32 * self.map.height as f32).round() as u32;
        // The distance from the map sides that starts to change the direction chances.
        let start = (
            rand::thread_rng().gen_range(2..self.map.width - 2),
            rand::thread_rng().gen_range(2..self.map.height - 2),
        );
        let mut end = start;
        let mut current_tile_coord = start;

        self.map
            .get_tile_mut(current_tile_coord.0, current_tile_coord.1)
            .unwrap()
            .solid = false;

        #[derive(Debug, Copy, Clone, PartialEq)]
        enum Direction {
            NoDir,
            Up,
            Right,
            Down,
            Left,
        }
        let directions = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        let mut num_floors = 1;
        let mut previous_dir = Direction::NoDir;
        let mut furthest_dist_from_start = 0;

        let mut weights = [1f32; 4];
        let mut rng = thread_rng();

        const LOCK_DIRECTION_BUFFER: u32 = 5;
        let mut lock_direction_remaining = 0;

        while num_floors < num_floors_to_get {
            let next_dir;
            if lock_direction_remaining > 0 && previous_dir != Direction::NoDir {
                if (previous_dir == Direction::Up && current_tile_coord.1 < LOCK_DIRECTION_BUFFER)
                    || (previous_dir == Direction::Right
                        && current_tile_coord.0 > self.map.width - LOCK_DIRECTION_BUFFER - 1)
                    || (previous_dir == Direction::Down
                        && current_tile_coord.1 > self.map.height - LOCK_DIRECTION_BUFFER - 1)
                    || (previous_dir == Direction::Left
                        && current_tile_coord.0 < LOCK_DIRECTION_BUFFER)
                {
                    // We're too close to the edge to continue going in the same direction
                    // Stop the direction lock and continue as normal
                    lock_direction_remaining = 0;
                    continue;
                } else {
                    lock_direction_remaining -= 1;
                    next_dir = previous_dir
                }
            } else if rng.gen_range(0.0f32..1.0f32) < 0.005f32 && previous_dir != Direction::NoDir {
                // special event where a direction is disabled for a random number of times.
                let min_dimension = cmp::min(self.map.width, self.map.height);
                let range = cmp::max(min_dimension / 10, 1)..min_dimension / 4;
                lock_direction_remaining = rng.gen_range(range);
                next_dir = previous_dir;
            } else {
                // reset weights back to default
                weights[0] = self.default_move_up_weight(current_tile_coord.1);
                weights[1] = self.default_move_right_weight(current_tile_coord.0);
                weights[2] = self.default_move_down_weight(current_tile_coord.1);
                weights[3] = self.default_move_left_weight(current_tile_coord.0);

                // if you just moved up, you can't move down etc...
                match previous_dir {
                    Direction::NoDir => { /* first loop, nothing needed */ }
                    Direction::Up => weights[2] = 0f32,
                    Direction::Right => weights[3] = 0f32,
                    Direction::Down => weights[0] = 0f32,
                    Direction::Left => weights[1] = 0f32,
                }

                let dist = WeightedIndex::new(&weights).unwrap();
                next_dir = directions[dist.sample(&mut rng)];
            }

            match next_dir {
                Direction::Up => current_tile_coord.1 -= 1,
                Direction::Right => current_tile_coord.0 += 1,
                Direction::Down => current_tile_coord.1 += 1,
                Direction::Left => current_tile_coord.0 -= 1,
                _ => panic!(),
            }
            previous_dir = next_dir;

            // If it's not already a floor (if it is solid)
            if self
                .map
                .get_tile(current_tile_coord.0, current_tile_coord.1)
                .unwrap()
                .solid
            {
                // Make it a floor
                self.map
                    .get_tile_mut(current_tile_coord.0, current_tile_coord.1)
                    .unwrap()
                    .solid = false;
                num_floors += 1;

                let dist_from_start = ((current_tile_coord.0 as i32 - start.0 as i32).abs()
                    + (current_tile_coord.1 as i32 - start.1 as i32).abs())
                    as u32;
                if dist_from_start > furthest_dist_from_start {
                    end.0 = current_tile_coord.0;
                    end.1 = current_tile_coord.1;
                    furthest_dist_from_start = dist_from_start;
                }
            }
        }
    }
}