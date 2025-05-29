pub struct Timeline {
    starting_board: [[Piece; 8]; 8],
    moves: Vec<Move>,
}

impl Timeline {}

pub enum Piece {
    None,
    Pawn(Side),
    Bishop(Side),
    Knight(Side),
    Rook(Side),
    Queen(Side),
    King(Side),
}

pub enum Side {
    White,
    Black,
}

pub struct Move {
    pub from: (u8, u8, u16),
    pub to: (u8, u8, u16),
}

impl Move {
    pub fn is_valid(&self) -> bool {
        return self.from.0 < 8 &&
               self.from.1 < 8 &&
               self.to.0 < 8 &&
               self.to.1 < 8;
    }
}