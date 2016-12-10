use pixelated::{Pixelated, Tile};

pub fn plan_move(game: &Pixelated) -> Option<Tile> {
    let grid = game.get_grid();
    let first = grid[0];
    grid.iter().find(|t| **t != first).map(|t| *t)
}
