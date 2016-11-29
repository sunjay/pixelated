extern crate rand;
extern crate ansi_term;

mod pixelated;

use ansi_term::Colour;

use pixelated::{Pixelated, Grid, Tile};

static BOX: &'static str = "\u{2588}";

fn main() {
    let game = Pixelated::new(10, 16);

    draw_grid(game.get_grid());
}

fn draw_grid(grid: &Grid) {
    for row in grid {
        for col in row {
            let tile = match *col {
                Tile::Blue => Colour::Blue.paint(BOX),
                Tile::Red => Colour::Red.paint(BOX),
                Tile::Green => Colour::Green.paint(BOX),
                Tile::Yellow => Colour::Yellow.paint(BOX),
                Tile::Cyan => Colour::Cyan.paint(BOX),
                Tile::Purple => Colour::Purple.paint(BOX),
            }.to_string();
            print!("{}", tile);
        }
        print!("\n");
    }
}
