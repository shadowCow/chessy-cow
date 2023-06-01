use crate::{pieces::{PlayerPiece, Player, ChessPiece, ChessSquare, BoardBySquares}, square_attackers::AttackingPieces};

#[derive(Debug, Clone)]
pub struct Chess {
    board: BoardBySquares,
    player_turn: Player,
    white_castling_conditions: CastlingConditions,
    black_castling_conditions: CastlingConditions,
    attackers: AttackingPieces,
}

impl Chess {
    pub fn new() -> Chess {
        Chess {
            board: [
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Rook }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Knight }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Bishop }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Queen }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::King }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Bishop }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Knight }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Rook }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::White, piece: ChessPiece::Pawn }),
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
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Pawn }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Rook }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Knight }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Bishop }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::King }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Queen }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Bishop }),
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Knight }),                
                Some(PlayerPiece { player: Player::Black, piece: ChessPiece::Rook }),
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
            attackers: AttackingPieces::new(),
        }
    }

    pub fn apply_move(&self, action: &ChessAction) -> MoveResult {
        MoveResult::InvalidMove { reason: "not implemented".to_owned() }
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
                    if let Some(PlayerPiece { player: last_player, piece: ChessPiece::Pawn }) = board[last_move.to] {
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
        self.attackers.is_attacked_by(square, self.player_turn.opponent())
    }
    fn get_row(square: usize) -> usize {
        square / 8
    }
    fn get_column(square: usize) -> usize {
        square % 8
    }
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

#[cfg(test)]
mod tests {
    use crate::pieces::{PlayerPieceOnSquare, board_by_pieces_to_squares};

    use super::*;

    fn replay_game(actions: Vec<ChessAction>) -> Chess {
        let game = Chess::new();

        for action in actions {
            game.apply_move(&action);
        }

        game
    }

    #[test]
    fn test_apply_move_scholars_mate() {
        let actions = get_scholars_mate_game();

        let expected_board_by_pieces = vec![
            PlayerPieceOnSquare {
                piece: PlayerPiece { player: Player::White, piece: ChessPiece::King },
                square: ChessSquare::A1,
            },
            PlayerPieceOnSquare {
                piece: PlayerPiece { player: Player::Black, piece: ChessPiece::King },
                square: ChessSquare::A1,
            },
        ];
        let expected_attackers = AttackingPieces::new();
        // expected_attackers.add_attacker(square, attacker)
        
        
        let expected_end_state = Chess {
            board: board_by_pieces_to_squares(&expected_board_by_pieces),
            player_turn: Player::Black,
            white_castling_conditions: CastlingConditions { king_has_moved: false, kingside_rook_has_moved: false, queenside_rook_has_moved: false, },
            black_castling_conditions: CastlingConditions { king_has_moved: false, kingside_rook_has_moved: false, queenside_rook_has_moved: false, },
            attackers: expected_attackers,
        };

        let actual_end_state = replay_game(actions);

        assert_eq!(expected_end_state, actual_end_state);
    }

    #[test]
    fn test_apply_move_test_game_1() {

    }

    #[test]
    fn test_apply_move_test_game_2() {

    }
}

/*
The classic Scholar's Mate (4 move checkmate)
 */
fn get_scholars_mate_game() -> Vec<ChessAction> {
    vec![
        ChessAction::MovePiece(PieceMove {
            from: ChessSquare::E2.to_square_index(),
            to: ChessSquare::E4.to_square_index(),
        }),
        ChessAction::MovePiece(PieceMove {
            from: ChessSquare::E7.to_square_index(),
            to: ChessSquare::E8.to_square_index(),
        }),
        ChessAction::MovePiece(PieceMove {
            from: ChessSquare::F1.to_square_index(),
            to: ChessSquare::C4.to_square_index(),
        }),
        ChessAction::MovePiece(PieceMove {
            from: ChessSquare::B8.to_square_index(),
            to: ChessSquare::C6.to_square_index(),
        }),
        ChessAction::MovePiece(PieceMove {
            from: ChessSquare::D1.to_square_index(),
            to: ChessSquare::F3.to_square_index(),
        }),
        ChessAction::MovePiece(PieceMove {
            from: ChessSquare::B7.to_square_index(),
            to: ChessSquare::B6.to_square_index(),
        }),
        ChessAction::MovePiece(PieceMove {
            from: ChessSquare::F3.to_square_index(),
            to: ChessSquare::F7.to_square_index(),
        }),
    ]
}

/*
John Nunn vs Igor Stohl
Pardubice (1993), Pardubice CZE, rd 6
Caro-Kann Defense: Advance. Short Variation (B12)  ·  1-0
 */
fn get_test_game_1() -> Vec<ChessAction> {
    vec![

    ]
}

/*
John Nunn vs Matthew Sadler
17th Lloyds Bank Masters Open (1993), London ENG, rd 9, Aug-29
Sicilian Defense: Najdorf Variation. English Attack (B90)  ·  1-0
 */
fn get_test_game_2() -> Vec<ChessAction> {
    vec![

    ]
}
