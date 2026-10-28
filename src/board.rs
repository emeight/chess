//! Board representation: squares, pieces, and colors.
use std::fmt;
use std::collections::HashMap;
use crate::pieces::{Piece, Kind};

// helpers
pub fn on_board(file: i8, rank: i8) -> bool {
    (0..8).contains(&file) && (0..8).contains(&rank)
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color { White, Black }

impl Color {
    pub fn opposite(self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

/// `(file, rank)` where `(0, 0)` is a1 and `(7, 7)` is h8.
pub type Square = (i8, i8);

/// Chessboard mapping each occupied square to its piece.
pub struct Board {
    pub squares: HashMap<Square, Piece>,
    pub side: Color,
    pub fullmove: u32,
}

impl Board {
    pub fn new() -> Self {
        Board { squares: HashMap::new(), side: Color::White, fullmove: 1 }
    }

    /// Initialize the chess board.
    pub fn init() -> Self {
        let mut board = Board::new();
        let back_rank = [Kind::Rook, Kind::Knight, Kind::Bishop, Kind::Queen,
                                    Kind::King, Kind::Bishop, Kind::Knight, Kind::Rook];

        for (file, &kind) in back_rank.iter().enumerate() {
            let file = file as i8;
            // white pieces
            board.squares.insert((file, 0), Piece { kind, color: Color::White });
            board.squares.insert((file, 1), Piece { kind: Kind::Pawn, color: Color::White });
            // black pieces
            board.squares.insert((file, 6), Piece { kind: Kind::Pawn, color: Color::Black });
            board.squares.insert((file, 7), Piece { kind, color: Color::Black });
        }

        board
    }

    pub fn get(&self, sq: Square) -> Option<Piece> {
        self.squares.get(&sq).copied()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                write!(f, "{} ", self.get((file, rank)).map_or('.', |p| p.to_char()))?;
            }
            writeln!(f)?;
        }
        write!(f, "  a b c d e f g h")
    }
}
