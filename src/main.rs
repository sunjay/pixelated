extern crate rand;
extern crate ansi_term;

mod pixelated;
mod ai;

use std::io::{self, Write};
use std::process::Command;
use std::time::Duration;
use std::thread::sleep;

use ansi_term::Colour;

use pixelated::{Pixelated, Tile};

static BOX: &'static str = "\u{2588}";

const ENABLE_AI: bool = true;

fn main() {
    let mut game = Pixelated::new();

    let stdin = io::stdin();

    let mut plan = None;
    if ENABLE_AI {
        plan = ai::plan_moves(&game);
    }

    let mut moves: usize = 0;
    let mut error = None;
    loop {
        clear_screen();

        draw_grid(&game);
        if game.is_complete() {
            draw_completed(moves);
            break;
        }
        else if ENABLE_AI {
            println!("");
            draw_moves(moves);

            let m = match plan {
                None => panic!("No moves planned"),
                Some(ref p) => p.get(moves),
            };
            if m.is_none() {
                panic!("Could not plan AI move");
            }

            sleep(Duration::from_millis(100));
            game.apply_tile(*m.unwrap());

            moves += 1;
            continue;
        }
        else {
            draw_controls();
            draw_moves(moves);
            draw_prompt(error.as_ref());
            flush_stdout();
        }

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

fn draw_grid(game: &Pixelated) {
    for row in 0..Pixelated::rows() {
        for col in 0..Pixelated::cols() {
            let tile = game.get((row as isize, col as isize)).unwrap();
            let tile = paint_str(tile, BOX);
            print!("{}", tile);
        }
        print!("\n");
    }
}

fn draw_completed(moves: usize) {
    println!("");
    draw_moves(moves);
    println!("You did it!");
}

fn draw_controls() {
    println!("");

    print!("{}", control_cell(Tile::Blue, "b"));
    print!(" {}", control_cell(Tile::Red, "r"));
    print!(" {}", control_cell(Tile::Green, "g"));
    print!(" {}", control_cell(Tile::Yellow, "y"));
    print!(" {}", control_cell(Tile::Cyan, "c"));
    print!(" {}", control_cell(Tile::Purple, "p"));
    print!(" quit with q");
    println!("\n");

}

fn draw_prompt(error: Option<&String>) {
    if !error.is_none() {
        println!("{}", Colour::Red.paint(error.unwrap().clone()).to_string());
    }

    print!("Enter color: ");
}

fn draw_moves(moves: usize) {
    println!("Moves: {}", moves);
}

fn flush_stdout() {
    io::stdout().flush().ok().expect("Could not flush stdout");
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
