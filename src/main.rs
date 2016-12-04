extern crate rand;
extern crate ncurses;

mod pixelated;

use std::char::from_u32;

use ncurses::{
    initscr,
    start_color,
    init_pair,
    attron,
    attroff,
    printw,
    refresh,
    getch,
    endwin,
    clear,
    COLOR_WHITE,
    COLOR_BLUE,
    COLOR_RED,
    COLOR_GREEN,
    COLOR_YELLOW,
    COLOR_CYAN,
    COLOR_MAGENTA,
    COLOR_BLACK,
    A_BOLD,
    COLOR_PAIR,
};

use pixelated::{Pixelated, Tile};

const BOX: &'static str = " ";
const BLUE_PAIR: i16 = 1;
const RED_PAIR: i16 = 2;
const GREEN_PAIR: i16 = 3;
const YELLOW_PAIR: i16 = 4;
const CYAN_PAIR: i16 = 5;
const PURPLE_PAIR: i16 = 6;
const ERROR_PAIR: i16 = 7;

fn main() {
    let mut game = Pixelated::new();

    initscr();
    start_color();
    init_pair(BLUE_PAIR, COLOR_WHITE, COLOR_BLUE);
    init_pair(RED_PAIR, COLOR_WHITE, COLOR_RED);
    init_pair(GREEN_PAIR, COLOR_WHITE, COLOR_GREEN);
    init_pair(YELLOW_PAIR, COLOR_WHITE, COLOR_YELLOW);
    init_pair(CYAN_PAIR, COLOR_WHITE, COLOR_CYAN);
    init_pair(PURPLE_PAIR, COLOR_WHITE, COLOR_MAGENTA);
    init_pair(ERROR_PAIR, COLOR_RED, COLOR_BLACK);

    let mut moves = 0;
    let mut error = None;
    loop {
        clear();
        draw_grid(&game);
        draw_controls(moves, error.as_ref());
        refresh();

        let c = from_u32(getch() as u32).expect("Input out of acceptable range");
        if c == 'q' {
            break;
        }
        else if c == '\n' {
            continue;
        }

        let tile = Tile::from_char(&c);
        if tile.is_none() {
            error = Some(format!("Unrecognized input: '{}'", c));
            continue;
        }

        error = None;
        moves += 1;

        game.apply_tile(tile.unwrap());
    }

    endwin();
}

fn draw_grid(game: &Pixelated) {
    for row in 0..Pixelated::rows() {
        for col in 0..Pixelated::cols() {
            let tile = game.get((row as isize, col as isize)).unwrap();
            printw_tile(tile, BOX);
        }
        printw(&format!("\n"));
    }
}

fn draw_controls(moves: u32, error: Option<&String>) {
    printw("\n");

    printw_control_cell(Tile::Blue, "b");
    printw(" ");
    printw_control_cell(Tile::Red, "r");
    printw(" ");
    printw_control_cell(Tile::Green, "g");
    printw(" ");
    printw_control_cell(Tile::Yellow, "y");
    printw(" ");
    printw_control_cell(Tile::Cyan, "c");
    printw(" ");
    printw_control_cell(Tile::Purple, "p");
    printw(" quit with q");
    printw("\n\n");

    printw(&format!("Moves: {}\n", moves));

    if !error.is_none() {
        printw_colored(ERROR_PAIR, &format!("{}\n", error.unwrap()));
    }

    printw("Enter color: ");
}

fn printw_control_cell(tile: Tile, command: &str) {
    attron(A_BOLD());
    printw_tile(tile, &format!(" {} ", command));
    attroff(A_BOLD());
}

fn printw_tile(tile: Tile, text: &str) {
    let pair = match tile {
        Tile::Blue => BLUE_PAIR,
        Tile::Red => RED_PAIR,
        Tile::Green => GREEN_PAIR,
        Tile::Yellow => YELLOW_PAIR,
        Tile::Cyan => CYAN_PAIR,
        Tile::Purple => PURPLE_PAIR,
    };

    printw_colored(pair, text);
}

fn printw_colored(pair: i16, text: &str) {
    attron(COLOR_PAIR(pair));
    printw(text);
    attroff(COLOR_PAIR(pair));
}
