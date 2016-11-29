use std::collections::{VecDeque, HashSet};

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

pub type Grid = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Clone)]
pub struct Pixelated {
    grid: Grid,
    rows: usize,
    cols: usize,
}

static DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Pixelated {
    pub fn new(rows: usize, cols: usize) -> Pixelated {
        assert!(rows > 0 && cols > 0, "Must have non-zero rows and columns");

        let mut rng = thread_rng();
        let sample = rng.gen_iter::<Tile>().take(rows * cols);

        let mut grid = vec![Vec::new()];
        for (i, tile) in sample.enumerate() {
            grid.last_mut().unwrap().push(tile);
            if (i + 1) % cols == 0 {
                grid.push(Vec::new());
            }
        }

        Pixelated {
            grid: grid,
            rows: rows,
            cols: cols,
        }
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }

    pub fn apply_tile(&mut self, tile: Tile) {
        let mut open = VecDeque::new();
        open.push_back((0, 0));

        let mut seen = HashSet::new();

        let original_tile = self.get((0, 0)).unwrap();
        while open.len() > 0 {
            let (row, col) = open.pop_front().unwrap();
            seen.insert((row, col));

            self.put_tile(row as usize, col as usize, tile);

            for &(drow, dcol) in DIRECTIONS.iter() {
                let next = (row + drow, col + dcol);
                if seen.contains(&next) {
                    continue;
                }

                let next_color = self.get(next);
                if next_color.is_some() && next_color.unwrap() == original_tile {
                    open.push_back(next);
                }
            }
        }
    }

    fn get(&self, (row, col): (isize, isize)) -> Option<Tile> {
        if row < 0 || col < 0 || row >= self.rows as isize || col >= self.cols as isize {
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
