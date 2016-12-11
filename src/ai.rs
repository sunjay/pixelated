use std::collections::{VecDeque};

use pixelated::{Pixelated, Tile};

pub fn plan_moves(game: &Pixelated) -> Option<Vec<Tile>> {
    let mut open = VecDeque::new();
    open.push_back((Vec::new(), game.clone()));

    let mut shortest_solution: Option<Vec<Tile>> = None;
    let mut solutions = 0;
    while !open.is_empty() {
        let (moves, game) = open.pop_front().unwrap();
        println!("{:?}", moves);

        if game.is_complete() {
            shortest_solution = match shortest_solution {
                None => Some(moves),
                Some(ss) => {
                    if moves.len() < ss.len() {
                        solutions += 1;
                        Some(moves)
                    }
                    else {
                        Some(ss)
                    }
                }
            };
            if solutions >= 10 {
                break;
            }
            continue;
        }
        else if moves.len() > 70 {
            continue;
        }

        let last_move = moves.last();
        for &tile in Tile::all().iter() {
            if last_move.is_some() && tile == *last_move.unwrap() {
                continue;
            }

            let mut next_game = game.clone();
            next_game.apply_tile(tile);
            let mut next_moves = moves.clone();
            next_moves.push(tile);

            open.push_back((next_moves, next_game));
        }
    }

    shortest_solution
}
