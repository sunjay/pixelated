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
}
