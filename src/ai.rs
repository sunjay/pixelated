use std::collections::{VecDeque, HashSet};

use pixelated::{Pixelated, Tile, DIRECTIONS};

pub fn plan_move(game: &Pixelated) -> Option<Tile> {
    let original_tile = game.get((0, 0)).unwrap();
    let mut open = VecDeque::new();
    let mut seen = HashSet::new();

    open.push_back((0, 0, original_tile));

    // Try to find the maximum manhattan distance away
    let mut max_distance = 0;
    let mut farthest_tile = None;
    while open.len() > 0 {
        let (row, col, tile) = open.pop_front().unwrap();

        if tile != original_tile {
            let distance = row + col;
            if distance >= max_distance {
                farthest_tile = Some(tile);
                max_distance = distance;
            }

            continue;
        }
        seen.insert((row, col));

        for &(drow, dcol) in DIRECTIONS.iter() {
            let next = (row + drow, col + dcol);
            let next_color = game.get(next);

            if next_color.is_some() && !seen.contains(&next) {
                open.push_front((next.0, next.1, next_color.unwrap()));
            }
        }
    }

    farthest_tile
}
