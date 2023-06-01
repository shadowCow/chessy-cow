pub type BoardBySquares = [Option<PlayerPiece>; 64];

pub type BoardByPieces = Vec<PlayerPieceOnSquare>;

pub fn board_by_pieces_to_squares(by_pieces: &BoardByPieces) -> BoardBySquares {
    let mut by_squares: [Option<PlayerPiece>; 64] = [None; 64];

    for piece in by_pieces {
        by_squares[piece.square.to_square_index()] = Some(piece.piece);
    }

    by_squares
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessPiece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PlayerPiece {
    pub player: Player,
    pub piece: ChessPiece,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PlayerPieceOnSquare {
    pub piece: PlayerPiece,
    pub square: ChessSquare,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessSquare {
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    C1,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    G1,
    G2,
    G3,
    G4,
    G5,
    G6,
    G7,
    G8,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    H7,
    H8,
}

impl ChessSquare {
    pub fn to_square_index(&self) -> usize {
        match self {
            ChessSquare::A1 => 0,
            ChessSquare::A2 => 8,
            ChessSquare::A3 => 16,
            ChessSquare::A4 => 24,
            ChessSquare::A5 => 32,
            ChessSquare::A6 => 40,
            ChessSquare::A7 => 48,
            ChessSquare::A8 => 56,
            ChessSquare::B1 => 1,
            ChessSquare::B2 => 9,
            ChessSquare::B3 => 17,
            ChessSquare::B4 => 25,
            ChessSquare::B5 => 33,
            ChessSquare::B6 => 41,
            ChessSquare::B7 => 49,
            ChessSquare::B8 => 57,
            ChessSquare::C1 => 2,
            ChessSquare::C2 => 10,
            ChessSquare::C3 => 18,
            ChessSquare::C4 => 26,
            ChessSquare::C5 => 34,
            ChessSquare::C6 => 42,
            ChessSquare::C7 => 50,
            ChessSquare::C8 => 58,
            ChessSquare::D1 => 3,
            ChessSquare::D2 => 11,
            ChessSquare::D3 => 19,
            ChessSquare::D4 => 27,
            ChessSquare::D5 => 35,
            ChessSquare::D6 => 43,
            ChessSquare::D7 => 51,
            ChessSquare::D8 => 59,
            ChessSquare::E1 => 4,
            ChessSquare::E2 => 12,
            ChessSquare::E3 => 20,
            ChessSquare::E4 => 28,
            ChessSquare::E5 => 36,
            ChessSquare::E6 => 44,
            ChessSquare::E7 => 52,
            ChessSquare::E8 => 60,
            ChessSquare::F1 => 5,
            ChessSquare::F2 => 13,
            ChessSquare::F3 => 21,
            ChessSquare::F4 => 29,
            ChessSquare::F5 => 37,
            ChessSquare::F6 => 45,
            ChessSquare::F7 => 53,
            ChessSquare::F8 => 61,
            ChessSquare::G1 => 6,
            ChessSquare::G2 => 14,
            ChessSquare::G3 => 22,
            ChessSquare::G4 => 30,
            ChessSquare::G5 => 38,
            ChessSquare::G6 => 46,
            ChessSquare::G7 => 54,
            ChessSquare::G8 => 62,
            ChessSquare::H1 => 7,
            ChessSquare::H2 => 15,
            ChessSquare::H3 => 23,
            ChessSquare::H4 => 31,
            ChessSquare::H5 => 39,
            ChessSquare::H6 => 47,
            ChessSquare::H7 => 55,
            ChessSquare::H8 => 63,
        }
    }
}
