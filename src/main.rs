extern crate rand;
extern crate ansi_term;

mod pixelated;

use std::io::{self, Write};
use std::process::Command;

use ansi_term::Colour;

use pixelated::{Pixelated, Grid, Tile};

static BOX: &'static str = "\u{2588}";

fn main() {
    let mut game = Pixelated::new(10, 64);

    let stdin = io::stdin();

    let mut moves = 0;
    let mut error = None;
    loop {
        clear_screen();

        draw_grid(game.get_grid());
        draw_controls(moves, error.as_ref());
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut buffer = String::new();
        stdin.read_line(&mut buffer).expect("Could not read input");

        let buffer = buffer.trim();
        if buffer == "q" {
            break;
        }

        let tile = Tile::from_str(&buffer);
        if tile.is_none() {
            error = Some(format!("Unrecognized input: '{}'", buffer));
            continue;
        }

        error = None;
        moves += 1;

        game.apply_tile(tile.unwrap());
    }
}

fn draw_grid(grid: &Grid) {
    for row in grid {
        for col in row {
            let tile = paint_str(*col, BOX);
            print!("{}", tile);
        }
        print!("\n");
    }
}

fn draw_controls(moves: u32, error: Option<&String>) {
    println!("");

    print!("{}", control_cell(Tile::Blue, "b"));
    print!(" {}", control_cell(Tile::Red, "r"));
    print!(" {}", control_cell(Tile::Green, "g"));
    print!(" {}", control_cell(Tile::Yellow, "y"));
    print!(" {}", control_cell(Tile::Cyan, "c"));
    print!(" {}", control_cell(Tile::Purple, "p"));
    print!(" quit with q");
    println!("\n");

    println!("Moves: {}", moves);

    if !error.is_none() {
        println!("{}", Colour::Red.paint(error.unwrap().clone()).to_string());
    }

    print!("Enter color: ");
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

fn clear_screen() {
    Command::new("clear").status().expect("Could not clear screen");
}
