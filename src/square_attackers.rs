use std::collections::HashMap;

use crate::pieces::{Player, ChessPiece};

#[derive(Debug, Clone)]
pub struct AttackingPieces {
    by_square: HashMap<usize, Vec<SquareOccupier>>,
}

impl AttackingPieces {
    pub fn new() -> Self {
        AttackingPieces {
            by_square: HashMap::new(),
        }
    }

    pub fn add_attacker(&mut self, square: usize, attacker: SquareOccupier) {
        self.by_square.entry(square).or_insert(Vec::new()).push(attacker);
    }

    pub fn get_attacking_pieces(&self, square: usize) -> Option<&Vec<SquareOccupier>> {
        self.by_square.get(&square)
    }

    pub fn count_attackers_for_player(&self, square: usize, player: Player) -> usize {
        match self.get_attacking_pieces(square) {
            Some(pieces) =>
                pieces.iter()
                    .filter(|p| p.player == player)
                    .count(),
            None => 0
        }
    }

    pub fn is_attacked_by(&self, square: usize, player: Player) -> bool {
        self.count_attackers_for_player(square, player) > 0
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct SquareOccupier {
    square: usize,
    player: Player,
    piece: ChessPiece,
}
