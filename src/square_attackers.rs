use std::collections::HashMap;

#[derive(Debug)]
struct AttackingPieces {
    map: HashMap<usize, Vec<SquareOccupier>>,
}

impl AttackingPieces {
    fn new() -> Self {
        AttackingPieces {
            map: HashMap::new(),
        }
    }

    fn add_attacker(&mut self, square: usize, attacker: SquareOccupier) {
        self.map.entry(square).or_insert(Vec::new()).push(attacker);
    }

    fn get_attacking_pieces(&self, square: usize) -> Option<&Vec<SquareOccupier>> {
        self.map.get(&square)
    }
}
