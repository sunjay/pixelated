extern crate rand;
extern crate ansi_term;

mod pixelated;

use std::io::{self, Read};

use ansi_term::Colour;

use pixelated::{Pixelated, Grid, Tile};

static BOX: &'static str = "\u{2588}";

fn main() {
    let game = Pixelated::new(10, 16);

    loop {
        draw_grid(game.get_grid());

        draw_controls();
        break;
    }
}

fn draw_grid(grid: &Grid) {
    for row in grid {
        print!(" ");
        for col in row {
            let tile = paint_str(*col, BOX);
            print!("{}", tile);
        }
        print!("\n");
    }
}

fn draw_controls() {
    println!("");

    print!(" {}", control_cell(Tile::Blue, "b"));
    print!(" {}", control_cell(Tile::Red, "r"));
    print!(" {}", control_cell(Tile::Green, "g"));
    print!(" {}", control_cell(Tile::Yellow, "y"));
    print!(" {}", control_cell(Tile::Cyan, "c"));
    print!(" {}", control_cell(Tile::Purple, "p"));

    println!("\n");
}

fn control_cell(tile: Tile, command: &str) -> String {
    Colour::White.bold().on(tile_colour(tile)).paint(format!(" {} ", command)).to_string()
}

fn paint_str(tile: Tile, data: &str) -> String {
    tile_colour(tile).paint(data).to_string()
}

fn tile_colour(tile: Tile) -> Colour {
    match tile {
        Tile::Blue => Colour::Blue,
        Tile::Red => Colour::Red,
        Tile::Green => Colour::Green,
        Tile::Yellow => Colour::Yellow,
        Tile::Cyan => Colour::Cyan,
        Tile::Purple => Colour::Purple,
    }
}
