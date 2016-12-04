use std::collections::{VecDeque, HashMap};

use rand::{thread_rng, Rng, Rand};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Tile {
    Blue,
    Red,
    Green,
    Yellow,
    Cyan,
    Purple,
}

static ALL_TILES: [Tile; 6] = [Tile::Blue, Tile::Red, Tile::Green, Tile::Yellow, Tile::Cyan, Tile::Purple];

impl Tile {
    pub fn from_str(data: &str) -> Option<Tile> {
        match data {
            "b" => Some(Tile::Blue),
            "r" => Some(Tile::Red),
            "g" => Some(Tile::Green),
            "y" => Some(Tile::Yellow),
            "c" => Some(Tile::Cyan),
            "p" => Some(Tile::Purple),
            _ => None,
        }
    }
}

impl Rand for Tile {
    fn rand<R: Rng>(rng: &mut R) -> Tile {
        *rng.choose(&ALL_TILES).unwrap()
    }
}

// Use these parameters to set the size of the game
// Make sure both values are greater than zero
const ROWS: usize = 10;
const COLS: usize = 64;

pub type Grid = HashMap<(isize, isize), Tile>;

pub struct Pixelated {
    grid: Grid,
}

static DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Pixelated {
    pub fn new() -> Pixelated {
        let mut rng = thread_rng();
        let sample = rng.gen_iter::<Tile>().take(ROWS * COLS);

        let mut grid: Grid = HashMap::new();
        for (i, tile) in sample.enumerate() {
            grid.insert(((i / COLS) as isize, (i % COLS) as isize), tile);
        }

        Pixelated {
            grid: grid,
        }
    }

    pub fn rows() -> usize {
        ROWS
    }

    pub fn cols() -> usize {
        COLS
    }

    pub fn apply_tile(&mut self, tile: Tile) {
        let mut open = VecDeque::new();
        open.push_back((0, 0));

        let original_tile = self.get(&(0, 0)).unwrap();
        while open.len() > 0 {
            let (row, col) = open.pop_front().unwrap();

            self.put_tile(row, col, tile);

            for &(drow, dcol) in DIRECTIONS.iter() {
                let next = (row + drow, col + dcol);
                let next_color = self.get(&next);

                if next_color.is_some() && next_color.unwrap() == original_tile {
                    open.push_back(next);
                }
            }
        }
    }

    pub fn get(&self, position: &(isize, isize)) -> Option<Tile> {
        self.grid.get(position).map(|v| *v)
    }

    fn put_tile(&mut self, row: isize, col: isize, tile: Tile) {
        // All the tiles that should be there, are already there, so any
        // new insertions are errors
        if self.grid.insert((row, col), tile).is_none() {
            panic!("Tile placed in invalid position");
        }
    }
}
