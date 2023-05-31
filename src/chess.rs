#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Chess {
    board: [Option<SquareOccupier>; 64],
    player_turn: Player,
    white_castling_conditions: CastlingConditions,
    black_castling_conditions: CastlingConditions,
}

impl Chess {
    pub fn new() -> Chess {
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
            white_castling_conditions: CastlingConditions {
                king_has_moved: false,
                kingside_rook_has_moved: false,
                queenside_rook_has_moved: false,
            },
            black_castling_conditions: CastlingConditions {
                king_has_moved: false,
                kingside_rook_has_moved: false,
                queenside_rook_has_moved: false,
            },
        }
    }

    fn is_legal_move(&self, action: &ChessAction, last_move: &PieceMove) -> bool {
        match action {
            ChessAction::MovePiece(piece_move) => {
                match self.board[piece_move.from] {
                    Some(occupier) => {
                        match occupier.piece {
                            ChessPiece::King => self.is_legal_king_move(&occupier.player, piece_move),
                            ChessPiece::Queen => self.is_legal_queen_move(&occupier.player, piece_move),
                            ChessPiece::Bishop => self.is_legal_bishop_move(&occupier.player, piece_move),
                            ChessPiece::Knight => self.is_legal_knight_move(&occupier.player, piece_move),
                            ChessPiece::Rook => self.is_legal_rook_move(&occupier.player, piece_move),
                            ChessPiece::Pawn => self.is_legal_pawn_move(&occupier.player, piece_move, last_move),
                        }
                    },
                    None => false,
                }
            },
            ChessAction::CastleKingSide(player) => self.is_legal_kingside_castle(player),
            ChessAction::CastleQueenSide(player) => self.is_legal_queenside_castle(player),
        }
    }

    fn is_legal_kingside_castle(&self, player: &Player) -> bool {
        match player {
            Player::White =>
                !self.white_castling_conditions.king_has_moved &&
                !self.white_castling_conditions.kingside_rook_has_moved &&
                !self.is_square_attacked_by_opponent(ChessSquare::E1.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::F1.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::G1.to_square_index()),
            Player::Black =>
                !self.black_castling_conditions.king_has_moved &&
                !self.black_castling_conditions.kingside_rook_has_moved &&
                !self.is_square_attacked_by_opponent(ChessSquare::E8.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::F8.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::G8.to_square_index()),
        }
    }

    fn is_legal_queenside_castle(&self, player: &Player) -> bool {
        match player {
            Player::White =>
                !self.white_castling_conditions.king_has_moved &&
                !self.white_castling_conditions.queenside_rook_has_moved &&
                !self.is_square_attacked_by_opponent(ChessSquare::E1.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::D1.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::C1.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::B1.to_square_index()),
            Player::Black =>
                !self.black_castling_conditions.king_has_moved &&
                !self.black_castling_conditions.queenside_rook_has_moved &&
                !self.is_square_attacked_by_opponent(ChessSquare::E8.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::D8.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::C8.to_square_index()) &&
                !self.is_square_attacked_by_opponent(ChessSquare::B8.to_square_index()),
        }
    }

    fn is_legal_rook_move(&self, player: &Player, piece_move: &PieceMove) -> bool {
        if Chess::is_square_occupied_by_own_piece(self, player, piece_move.to) {
            false
        } else {
            Chess::is_legal_horizontal_move(self, piece_move) ||
            Chess::is_legal_vertical_move(self, piece_move)
        }
    }

    fn is_legal_bishop_move(&self, player: &Player, piece_move: &PieceMove) -> bool {
        if Chess::is_square_occupied_by_own_piece(self, player, piece_move.to) {
            false
        } else {
            Chess::is_legal_diagonal_move(self, piece_move)
        }
    }

    fn is_legal_knight_move(&self, player: &Player, piece_move: &PieceMove) -> bool {
        let row_diff = Chess::row_diff(piece_move);
        let col_diff = Chess::column_diff(piece_move);

        if (row_diff == 2 && col_diff == 1) || (row_diff == 1 && col_diff == 2) {
            !Chess::is_square_occupied_by_own_piece(self, player, piece_move.to)
        } else {
            false
        }
    }

    fn is_legal_queen_move(&self, player: &Player, piece_move: &PieceMove) -> bool {
        if Chess::is_square_occupied_by_own_piece(self, player, piece_move.to) {
            false
        } else {
            Chess::is_legal_horizontal_move(self, piece_move) ||
            Chess::is_legal_vertical_move(self, piece_move) ||
            Chess::is_legal_diagonal_move(self, piece_move)
        }
    }

    fn is_legal_king_move(&self, player: &Player, piece_move: &PieceMove) -> bool {
        let row_from = piece_move.from / 8;
        let col_from = piece_move.from % 8;
        let row_to = piece_move.to / 8;
        let col_to = piece_move.to % 8;
    
        // Check if the move is within the king's range (adjacent square or castling)
        if (row_to == row_from && (col_to == col_from + 1 || col_to == col_from - 1)) // Horizontal move
            || (col_to == col_from && (row_to == row_from + 1 || row_to == row_from - 1)) // Vertical move
            || (row_to == row_from + 1 && col_to == col_from + 1) // Diagonal move
            || (row_to == row_from - 1 && col_to == col_from + 1) // Diagonal move
            || (row_to == row_from + 1 && col_to == col_from - 1) // Diagonal move
            || (row_to == row_from - 1 && col_to == col_from - 1) // Diagonal move
        {
            if self.is_square_attacked_by_opponent(piece_move.to) {
                // king cannot put himself in check
                return false;
            } else {
                // Check if the target square is unoccupied or contains an opponent's piece
                if let Some(target_occupier) = self.board[piece_move.to] {
                    if target_occupier.player != *player {
                        return true; // Capture is allowed
                    }
                } else {
                    return true; // Move to an empty square
                }
            }
        }
    
        false // Move is not legal for a king
    }
    
    fn is_legal_pawn_move(&self, player: &Player, piece_move: &PieceMove, last_move: &PieceMove) -> bool {
        let from = piece_move.from;
        let to = piece_move.to;
        let board = self.board;

        let row_from = from / 8;
        let col_from = from % 8;
        let row_to = to / 8;
        let col_to = to % 8;

        let one_square_target = if *player == Player::White {
            row_from + 1
        } else {
            row_from - 1
        };
        let two_square_target = if *player == Player::White {
            row_from + 2
        } else {
            row_from - 2
        };
        let starting_row = if *player == Player::White {
            2
        } else {
            7
        };

        // Check if the pawn is moving forward by one square
        if col_from == col_to && row_to == one_square_target && board[to].is_none() {
            return true;
        }

        // Check if the pawn is moving forward by two squares from the starting rank
        if col_from == col_to && row_to == two_square_target && row_from == starting_row
            && board[one_square_target].is_none() && board[to].is_none() {
            return true;
        }

        // Check if the pawn is capturing diagonally
        if (col_to == col_from + 1 || col_to == col_from - 1) && row_to == one_square_target {
            if let Some(occupier) = board[to] {
                if occupier.player != *player {
                    return true;
                }
            } else {
                // Check for en passant capture
                let last_row_from = last_move.from / 8;
                let last_col_from = last_move.from % 8;
                let last_row_to = last_move.to / 8;
                let last_col_to = last_move.to % 8;

                let capture_to_last_col = col_to == last_col_to;
                let pawns_on_same_rank = row_from == last_row_to;
                let last_was_two_square_move = last_row_from.abs_diff(last_row_to) == 2;

                if capture_to_last_col && pawns_on_same_rank && last_was_two_square_move {
                    if let Some(SquareOccupier { player: last_player, piece: ChessPiece::Pawn }) = board[last_move.to] {
                        if last_player != *player {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }
    
    fn is_square_occupied_by_own_piece(&self, player: &Player, square: usize) -> bool {
        match self.board[square] {
            Some(occupier) => occupier.player == *player,
            None => false,
        }
    }

    fn is_legal_diagonal_move(&self, piece_move: &PieceMove) -> bool {
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

    fn is_legal_horizontal_move(&self, piece_move: &PieceMove) -> bool {
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

    fn is_legal_vertical_move(&self, piece_move: &PieceMove) -> bool {
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

    fn is_same_row(piece_move: &PieceMove) -> bool {
        Chess::get_row(piece_move.from) == Chess::get_row(piece_move.to)
    }
    fn is_same_column(piece_move: &PieceMove) -> bool {
        Chess::get_column(piece_move.from) == Chess::get_column(piece_move.to)
    }
    fn row_diff(piece_move: &PieceMove) -> usize {
        let from_row = Chess::get_row(piece_move.from);

        let to_row = Chess::get_row(piece_move.from);

        from_row.abs_diff(to_row)
    }
    fn column_diff(piece_move: &PieceMove) -> usize {
        let from_column = Chess::get_column(piece_move.from);

        let to_column = Chess::get_column(piece_move.to);

        from_column.abs_diff(to_column)
    }
    fn is_square_attacked_by_opponent(&self, square: usize) -> bool {
        true
    }
    fn get_row(square: usize) -> usize {
        square / 8
    }
    fn get_column(square: usize) -> usize {
        square % 8
    }

    fn apply_move(&self, action: &ChessAction) -> MoveResult {
        MoveResult::InvalidMove { reason: "not implemented".to_owned() }
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
struct CastlingConditions {
    king_has_moved: bool,
    kingside_rook_has_moved: bool,
    queenside_rook_has_moved: bool,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        
    }
}