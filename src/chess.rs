#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Chess {
    board: [Option<SquareOccupier>; 64],
    player_turn: Player,
    white_has_castled: bool,
    black_has_castled: bool,
}

impl Chess {
    fn new() -> Chess {
        Chess {
            board: [
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Rook }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Knight }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Bishop }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Queen }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::King }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Bishop }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Knight }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Rook }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::White, piece: ChessPiece::Pawn }),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Rook }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Knight }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Bishop }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::King }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Queen }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Bishop }),
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Knight }),                
                Some(SquareOccupier { player: Player::Black, piece: ChessPiece::Rook }),
            ],
            player_turn: Player::White,
            white_has_castled: false,
            black_has_castled: false,
        }
    }

    fn is_legal_move(&self, action: &ChessAction) -> bool {

    }

    fn is_legal_rook_move(&self, player: Player, piece_move: PieceMove) -> bool {
        if Chess::is_square_occupied_by_own_piece(self, player, piece_move.to) {
            false
        } else {
            Chess::is_legal_horizontal_move(self, piece_move) ||
            Chess::is_legal_vertical_move(self, piece_move)
        }
    }

    fn is_legal_bishop_move(&self, player: Player, piece_move: PieceMove) -> bool {
        if Chess::is_square_occupied_by_own_piece(self, player, piece_move.to) {
            false
        } else {
            Chess::is_legal_diagonal_move(self, piece_move)
        }
    }

    fn is_legal_knight_move(&self, player: Player, piece_move: PieceMove) -> bool {
        let row_diff = Chess::row_diff(piece_move);
        let col_diff = Chess::column_diff(piece_move);

        if (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2) {
            !Chess::is_square_occupied_by_own_piece(self, player, piece_move.to)
        } else {
            false
        }
    }

    fn is_legal_queen_move(&self, player: Player, piece_move: PieceMove) -> bool {
        if Chess::is_square_occupied_by_own_piece(self, player, piece_move.to) {
            false
        } else {
            Chess::is_legal_horizontal_move(self, piece_move) ||
            Chess::is_legal_vertical_move(self, piece_move) ||
            Chess::is_legal_diagonal_move(self, piece_move)
        }
    }

    fn is_legal_king_move(&self, player: Player, piece_move: PieceMove) -> bool {}

    fn is_legal_pawn_move(&self, player: Player, piece_move: PieceMove) -> bool {}

    fn is_square_occupied_by_own_piece(&self, player: Player, square: usize) -> bool {
        match self.board[square] {
            Some(occupier) => occupier.player == player,
            None => false,
        }
    }

    fn is_legal_diagonal_move(&self, piece_move: PieceMove) -> bool {
        let from_row = Chess::get_row(piece_move.from);
        let from_column = Chess::get_column(piece_move.from);

        let to_row = Chess::get_row(piece_move.from);
        let to_column = Chess::get_column(piece_move.to);

        let row_diff = from_row.abs_diff(to_row);
        let column_diff = from_column.abs_diff(to_column);
    
        if row_diff != column_diff {
            return false; // Not on the same diagonal
        }
    
        let row_step: i8 = if from_row < to_row { 1 } else { -1 };
        let col_step: i8 = if from_column < to_column { 1 } else { -1 };
    
        let mut row = from_row as i8 + row_step;
        let mut col = from_column as i8 + col_step;
    
        while row != to_row as i8 || col != to_column as i8 {
            let index = row * 8 + col;
            if self.board[index as usize].is_some() {
                return false; // Square along the diagonal is occupied
            }
            row += row_step;
            col += col_step;
        }
    
        true // No occupied square along the diagonal  
    }

    fn is_legal_horizontal_move(&self, piece_move: PieceMove) -> bool {
        let from_row = piece_move.from / 8;
        let from_col = piece_move.from % 8;
        let to_row = piece_move.to / 8;
        let to_col = piece_move.to % 8;

        if from_row != to_row {
            return false; // Not on the same row
        }

        let col_start = from_col.min(to_col);
        let col_end = from_col.max(to_col);

        for col in (col_start + 1)..col_end {
            let index = from_row * 8 + col;
            if self.board[index].is_some() {
                return false; // Square between the indices is occupied
            }
        }

        true // No occupied square between the indices
    }

    fn is_legal_vertical_move(&self, piece_move: PieceMove) -> bool {
        let from_row = piece_move.from / 8;
        let from_col = piece_move.from % 8;
        let to_row = piece_move.to / 8;
        let to_col = piece_move.to % 8;
    
        if from_col != to_col {
            return false; // Not on the same column
        }
    
        let row_start = from_row.min(to_row);
        let row_end = from_row.max(to_row);
    
        for row in (row_start + 1)..row_end {
            let index = row * 8 + from_col;
            if self.board[index].is_some() {
                return false; // Square between the indices is occupied
            }
        }
    
        true // No occupied square between the indices
    }

    fn is_same_row(piece_move: PieceMove) -> bool {
        Chess::get_row(piece_move.from) == Chess::get_row(piece_move.to)
    }
    fn is_same_column(piece_move: PieceMove) -> bool {
        Chess::get_column(piece_move.from) == Chess::get_column(piece_move.to)
    }
    fn row_diff(piece_move: PieceMove) -> usize {
        let from_row = Chess::get_row(piece_move.from);

        let to_row = Chess::get_row(piece_move.from);

        from_row.abs_diff(to_row)
    }
    fn column_diff(piece_move: PieceMove) -> usize {
        let from_column = Chess::get_column(piece_move.from);

        let to_column = Chess::get_column(piece_move.to);

        from_column.abs_diff(to_column)
    }
    fn is_square_protected(&self, square: usize) -> bool {

    }
    fn get_row(square: usize) -> usize {
        square / 8
    }
    fn get_column(square: usize) -> usize {
        square % 8
    }

    fn apply_move(&self, action: &ChessAction) -> MoveResult {

    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SquareOccupier {
    player: Player,
    piece: ChessPiece,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChessAction {
    MovePiece(PieceMove),
    CastleKingSide(Player),
    CastleQueenSide(Player),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PieceMove {
    from: usize,
    to: usize,
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
    fn to_square_index(&self) -> usize {
        match self {
            ChessSquare::A1 => 0,
            ChessSquare::A2 => 1,
            ChessSquare::A3 => 2,
            ChessSquare::A4 => 3,
            ChessSquare::A5 => 4,
            ChessSquare::A6 => 5,
            ChessSquare::A7 => 6,
            ChessSquare::A8 => 7,
            ChessSquare::B1 => 8,
            ChessSquare::B2 => 9,
            ChessSquare::B3 => 10,
            ChessSquare::B4 => 11,
            ChessSquare::B5 => 12,
            ChessSquare::B6 => 13,
            ChessSquare::B7 => 14,
            ChessSquare::B8 => 15,
            ChessSquare::C1 => 16,
            ChessSquare::C2 => 17,
            ChessSquare::C3 => 18,
            ChessSquare::C4 => 19,
            ChessSquare::C5 => 20,
            ChessSquare::C6 => 21,
            ChessSquare::C7 => 22,
            ChessSquare::C8 => 23,
            ChessSquare::D1 => 24,
            ChessSquare::D2 => 25,
            ChessSquare::D3 => 26,
            ChessSquare::D4 => 27,
            ChessSquare::D5 => 28,
            ChessSquare::D6 => 29,
            ChessSquare::D7 => 30,
            ChessSquare::D8 => 31,
            ChessSquare::E1 => 32,
            ChessSquare::E2 => 33,
            ChessSquare::E3 => 34,
            ChessSquare::E4 => 35,
            ChessSquare::E5 => 36,
            ChessSquare::E6 => 37,
            ChessSquare::E7 => 38,
            ChessSquare::E8 => 39,
            ChessSquare::F1 => 40,
            ChessSquare::F2 => 41,
            ChessSquare::F3 => 42,
            ChessSquare::F4 => 43,
            ChessSquare::F5 => 44,
            ChessSquare::F6 => 45,
            ChessSquare::F7 => 46,
            ChessSquare::F8 => 47,
            ChessSquare::G1 => 40,
            ChessSquare::G2 => 49,
            ChessSquare::G3 => 50,
            ChessSquare::G4 => 51,
            ChessSquare::G5 => 52,
            ChessSquare::G6 => 53,
            ChessSquare::G7 => 54,
            ChessSquare::G8 => 55,
            ChessSquare::H1 => 56,
            ChessSquare::H2 => 57,
            ChessSquare::H3 => 58,
            ChessSquare::H4 => 59,
            ChessSquare::H5 => 60,
            ChessSquare::H6 => 61,
            ChessSquare::H7 => 62,
            ChessSquare::H8 => 63,
        }
    }
}


#[derive(Debug, PartialEq)]
pub enum MoveResult {
    MadeMove { outcome: GameOutcome },
    InvalidMove { reason: String },
}

#[derive(Debug, PartialEq)]
pub enum GameOutcome {
    Unfinished,
    Tie,
    Winner(Player),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn next_turn(&self) -> Player {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}