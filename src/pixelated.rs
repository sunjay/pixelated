use std::collections::{VecDeque};

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

pub type Grid = [[Tile; COLS]; ROWS];

pub struct Pixelated {
    grid: Grid,
}

static DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Pixelated {
    pub fn new() -> Pixelated {
        let mut rng = thread_rng();
        let sample = rng.gen_iter::<Tile>().take(ROWS * COLS);

        let mut grid = [[Tile::Red; COLS]; ROWS];
        for (i, tile) in sample.enumerate() {
            grid[i / COLS][i % COLS] = tile;
        }

        Pixelated {
            grid: grid,
        }
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    pub fn apply_tile(&mut self, tile: Tile) {
        let mut open = VecDeque::new();
        open.push_back((0, 0));

        let original_tile = self.get((0, 0)).unwrap();
        while open.len() > 0 {
            let (row, col) = open.pop_front().unwrap();

            self.put_tile(row as usize, col as usize, tile);

            for &(drow, dcol) in DIRECTIONS.iter() {
                let next = (row + drow, col + dcol);
                let next_color = self.get(next);

                if next_color.is_some() && next_color.unwrap() == original_tile {
                    open.push_back(next);
                }
            }
        }
    }

    fn get(&self, (row, col): (isize, isize)) -> Option<Tile> {
        if row < 0 || col < 0 || row >= ROWS as isize || col >= COLS as isize {
            None
        }
        else {
            Some(self.grid[row as usize][col as usize])
        }
    }

    fn put_tile(&mut self, row: usize, col: usize, tile: Tile) {
        self.grid[row][col] = tile;
    }
}
