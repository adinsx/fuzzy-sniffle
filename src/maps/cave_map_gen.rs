use super::map::Map;
use rand::{distributions::WeightedIndex, prelude::*, Rng};
use std::cmp;

pub fn cave_map_gen(map: &mut Map, floor_coverage: f32, edge_repulsion_dist: u32) {
    let width = map.width;
    let height = map.height;
    // Move away from walls more as you get closer to them
    let default_move_weight = |relevant_coord: u32, relevant_size: u32| -> f32 {
        if relevant_coord < edge_repulsion_dist {
            1f32 - (edge_repulsion_dist - relevant_coord) as f32 / (edge_repulsion_dist - 1) as f32
        } else if relevant_coord > relevant_size - edge_repulsion_dist - 1 {
            1f32 + (relevant_coord + edge_repulsion_dist + 1 - relevant_size) as f32
                / (edge_repulsion_dist - 1) as f32
        } else {
            1f32
        }
    };
    let default_move_left_weight = |x: u32| -> f32 { default_move_weight(x, width) };
    let default_move_right_weight = |x: u32| -> f32 { default_move_weight(width - x - 1, width) };
    let default_move_up_weight = |y: u32| -> f32 { default_move_weight(y, height) };
    let default_move_down_weight = |y: u32| -> f32 { default_move_weight(height - y - 1, height) };
    let num_wall_neighbors = |map: &Map, x: u32, y: u32| -> u32 {
        let i_x = x as i32;
        let i_y = y as i32;

        map.get_tile_modulo(i_x - 1, i_y - 1).unwrap().wall as u32 +
        map.get_tile_modulo(i_x - 1, i_y).unwrap().wall as u32 +
        map.get_tile_modulo(i_x - 1, i_y + 1).unwrap().wall as u32 +
        map.get_tile_modulo(i_x, i_y - 1).unwrap().wall as u32 +
        // Don't include yourself!
        map.get_tile_modulo(i_x, i_y + 1).unwrap().wall as u32 +
        map.get_tile_modulo(i_x + 1, i_y - 1).unwrap().wall as u32 +
        map.get_tile_modulo(i_x + 1, i_y).unwrap().wall as u32 +
        map.get_tile_modulo(i_x + 1, i_y + 1).unwrap().wall as u32
    };
    // With (x,y) being the top left corner, figure out if we have either one these patterns
    // _ = not wall
    // O = wall
    //
    // O_
    // _O which returns \
    //
    // _O
    // O_ which returns /
    //
    // no match returns empty string
    let is_part_of_checkerboard = |map: &Map, x: u32, y: u32| -> &str {
        let mut ret = "";
        let i_x = x as i32;
        let i_y = y as i32;
        // nw ne
        // sw se
        let nw = map.get_tile_modulo(i_x, i_y).unwrap().wall;
        let ne = map.get_tile_modulo(i_x + 1, i_y).unwrap().wall;
        let sw = map.get_tile_modulo(i_x, i_y + 1).unwrap().wall;
        let se = map.get_tile_modulo(i_x + 1, i_y + 1).unwrap().wall;

        if nw == se && ne == sw && nw != ne {
            // We've matched one of the patterns, now figure out which one
            ret = if nw { "\\" } else { "/" };
        }
        ret
    };

    //==================== GENERATION ====================

    // We've got our necessary helper closures, now start actually generating
    let num_floors_to_get = (floor_coverage * width as f32 * height as f32).round() as u32;
    // The distance from the map sides that starts to change the direction chances.
    let start = (
        rand::thread_rng().gen_range(2..width - 2),
        rand::thread_rng().gen_range(2..height - 2),
    );
    let mut end = start;
    let mut current_tile_coord = start;

    map.get_tile_mut(current_tile_coord.0, current_tile_coord.1)
        .unwrap()
        .wall = false;

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
                    && current_tile_coord.0 > width - LOCK_DIRECTION_BUFFER - 1)
                || (previous_dir == Direction::Down
                    && current_tile_coord.1 > height - LOCK_DIRECTION_BUFFER - 1)
                || (previous_dir == Direction::Left && current_tile_coord.0 < LOCK_DIRECTION_BUFFER)
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
            let min_dimension = cmp::min(width, height);
            let range = cmp::max(min_dimension / 10, 1)..min_dimension / 4;
            lock_direction_remaining = rng.gen_range(range);
            next_dir = previous_dir;
        } else {
            // reset weights back to default
            weights[0] = default_move_up_weight(current_tile_coord.1);
            weights[1] = default_move_right_weight(current_tile_coord.0);
            weights[2] = default_move_down_weight(current_tile_coord.1);
            weights[3] = default_move_left_weight(current_tile_coord.0);

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
        if map
            .get_tile(current_tile_coord.0, current_tile_coord.1)
            .unwrap()
            .wall
        {
            // Make it a floor
            map.get_tile_mut(current_tile_coord.0, current_tile_coord.1)
                .unwrap()
                .wall = false;
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

    //==================== POST PROCESS ====================

    // Start and end can't be too close to edge. For start this is already done based
    // on it's initial possible values but end might be near an edge and need fixed
    end.0 = end.0.clamp(2, width - 3);
    end.1 = end.1.clamp(2, height - 3);

    // Pad start and end with surrounding floors.
    for x in -1..2 {
        for y in -1..2 {
            map.get_tile_mut((start.0 as i32 + x) as u32, (start.1 as i32 + y) as u32)
                .unwrap()
                .wall = false;
            map.get_tile_mut((end.0 as i32 + x) as u32, (end.1 as i32 + y) as u32)
                .unwrap()
                .wall = false;
        }
    }

    let mut recheck = true;
    while recheck {
        recheck = false;
        for x in 0..width {
            for y in 0..height {
                // First check for and remove checkboard patterns
                let checkered = is_part_of_checkerboard(map, x, y);
                match checkered {
                    // If it's part of a checkerboard, turn the walls to floor
                    "\\" => {
                        map.get_tile_modulo_mut(x as i32, y as i32).unwrap().wall = false;
                        map.get_tile_modulo_mut((x + 1) as i32, (y + 1) as i32)
                            .unwrap()
                            .wall = false;
                        recheck = true;
                    }
                    "/" => {
                        map.get_tile_modulo_mut((x + 1) as i32, y as i32)
                            .unwrap()
                            .wall = false;
                        map.get_tile_modulo_mut(x as i32, (y + 1) as i32)
                            .unwrap()
                            .wall = false;
                        recheck = true;
                    }
                    "" => { /* not a checkerboard, nothing to do */ }
                    _ => panic!(),
                }
                // Then check for and remove walls with less than 2 neighbors
                let tile = map.get_tile_modulo(x as i32, y as i32).unwrap();
                if tile.wall && num_wall_neighbors(map, x, y) < 2 {
                    map.get_tile_modulo_mut(x as i32, y as i32).unwrap().wall = false;
                    recheck = true;
                }
            }
        }
    }
}
