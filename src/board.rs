//! Board representation: squares, pieces, and colors.
use std::collections::HashMap;

/// Chessboard keyed by `(rank, file)`.
/// 
/// Keys are integer tuples `(i8, i8)` such that
/// `(0, 0)` is a1 and `(7, 7)` is h8.
/// 
/// Pieces are stored as `char`s using the FEN convention
/// (uppercase = white, lowercase = black).
pub struct Board {
    squares: HashMap<(i8, i8), char>,
}

// helpers
fn on_board(file: i8, rank: i8) -> bool {
    (0..8).contains(&file) && (0..8).contains(&rank)
}

fn parse_square(s: &str) -> Option<(i8, i8)> {
    let b = s.as_bytes();
    if b.len() != 2 { return None; }
    let file = b[0].checked_sub(b'a')? as i8;
    let rank = b[1].checked_sub(b'1')? as i8;
    on_board(file, rank).then_some((file, rank))
}


impl Board {
    pub fn new() -> Self {
        Board { squares: HashMap::new() }
    }

    /// Initialize the chess board.
    pub fn init() -> Self {
        let mut board = Board::new();
        let back_rank = ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R'];

        for (file, &piece) in back_rank.iter().enumerate() {
            let file = file as i8;
            // white pieces
            board.squares.insert((0, file), piece);
            board.squares.insert((1, file), 'P');
            // black pieces
            board.squares.insert((6, file), 'p');
            board.squares.insert((7, file), piece.to_ascii_lowercase());
        }

        board
    }

    pub fn get(&self, file: i8, rank: i8) -> Option<char> {
        self.squares.get(&(rank, file)).copied()
    }

    pub fn get_sq(&self, s: &str) -> Result<Option<char>, String> {
        let (rank, file) = parse_square(s)
            .ok_or_else(|| format!("invalid square: {s}"))?;
        Ok(self.get(rank, file))
    }
}
