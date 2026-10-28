//! Terminal frontend for chess game.
mod board;
mod moves;
mod pieces;

use board::{Board, Color};

use std::io::{self, Write};


fn prompt(message: &str) -> String {
    print!("{message}");
    io::stdout().flush().expect("failed to flush");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to my chess game!");
    let mut board = Board::init();

    // game loop
    loop {
        println!("{board}");
        let dots = if board.side == Color::White { "." } else { "..." };
        let command = prompt(&format!("{}{dots} ", board.fullmove));

        match command.as_str() {
            "exit" | "quit" => break,
            "" => continue,
            other => match board.move_piece(other) {
                Ok(()) => {},
                Err(e) => println!("{e}"),
            },
        }
    }
    println!("Thanks for playing!")
}
