//! Piece types and char conversion.
use crate::board::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Kind { Pawn, Knight, Bishop, Rook, Queen, King }

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Piece {
    pub kind: Kind,
    pub color: Color,
}

impl Piece {
    pub fn to_char(self) -> char {
        let c = match self.kind {
            Kind::Pawn => 'p',
            Kind::Knight => 'n',
            Kind::Bishop => 'b',
            Kind::Rook => 'r',
            Kind::Queen => 'q',
            Kind::King => 'k',
        };
        match self.color {
            Color::White => c.to_ascii_uppercase(),
            Color::Black => c
        }
    }

    pub fn from_char(c: char) -> Option<Piece> {
        let kind = match c.to_ascii_lowercase() {
            'p' => Kind::Pawn,
            'n' => Kind::Knight,
            'b' => Kind::Bishop,
            'r' => Kind::Rook,
            'q' => Kind::Queen,
            'k' => Kind::King,
            _ => return None,
        };
        let color = if c.is_ascii_uppercase() { Color::White } else { Color::Black };
        Some(Piece{ kind, color })
    }
}
