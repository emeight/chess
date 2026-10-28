//! Move types, move generation, and legality checks.
use crate::board::{Board, Color, Square, on_board};

// helper
fn parse_square(s: &str) -> Option<Square> {
    let b = s.as_bytes();
    if b.len() != 2 { return None; }
    let file = b[0].checked_sub(b'a')? as i8;
    let rank = b[1].checked_sub(b'1')? as i8;
    on_board(file, rank).then_some((file, rank))
}


struct Move {
    pub from: Square,
    pub to: Square,
}

fn parse_move(s: &str) -> Option<Move> {
    // moves are to be provided as "{origin}{destination}" i.e. "e1e2"
    Some(Move {
        from: parse_square(s.get(..2)?)?,
        to: parse_square(s.get(2..)?)?,
    })
}

impl Board {
    pub fn move_piece(&mut self, s: &str) -> Result<(), String> {
        let mv = parse_move(s).ok_or_else(|| format!("invalid move: {s}"))?;
        let piece = self.get(mv.from)
            .ok_or_else(|| format!("no piece on {}", &s[..2]))?;
        if piece.color != self.side {
            return Err(format!("not your piece on {}", &s[..2]));
        }
        self.squares.remove(&mv.from);
        self.squares.insert(mv.to, piece);

        // increment fullmove (turn) if move completed by black
        if self.side == Color::Black { self.fullmove += 1; }
        self.side = self.side.opposite();
        Ok(())
    }
}
