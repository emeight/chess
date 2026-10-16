//! Terminal frontend for chess game.
mod board;

use board::Board;

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
    let board = Board::init();

    // game loop
    loop {
        let command = prompt("> ");

        match command.as_str() {
            "exit" => break,
            "quit" => break,
            "" => continue,
            other => match board.get_sq(other) {
                Ok(Some(p)) => println!("{p}"),
                Ok(None) => println!("empty"),
                Err(e) => println!("{e}"),
            },
        }
    }
    println!("Thanks for playing!")
}
