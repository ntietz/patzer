use cozy_chess::Move;

use crate::evaluation::Score;

pub type Hash = u64;

#[derive(Clone, Copy)]
pub struct TableEntry {
    /// Hash (zobrist) of the current position
    pub hash: Hash,

    /// Distance from the root of the search tree
    pub depth: u8,

    /// The score generated on a previous search of this position
    pub eval: Evaluation,

    /// The best move or refutation of this position
    pub following_move: Option<Move>,
}

#[derive(Clone, Copy)]
pub enum Evaluation {
    Exact(Score),
    Beta(Score),
    Alpha(Score),
}

pub struct TranspositionTable {
    transpositions: Vec<Option<TableEntry>>,
    size: usize,
}

impl TranspositionTable {
    pub fn new() -> Self {
        let size = 1_000_000;
        let transpositions = vec![None; size];
        TranspositionTable {
            transpositions,
            size,
        }
    }

    pub fn len(&self) -> usize {
        self.transpositions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transpositions.is_empty()
    }

    pub fn store(&mut self, hash: Hash, depth: u8, eval: Evaluation) {
        let position = hash as usize % self.size;

        let entry = self.transpositions.get_mut(position).unwrap();
        if entry.is_none() || entry.unwrap().depth < depth {
            *entry = Some(TableEntry {
                hash,
                depth,
                eval,
                following_move: None,
            });
        }
    }

    pub fn retrieve(&mut self, hash: Hash) -> Option<TableEntry> {
        let position = hash as usize % self.size;

        let result = self
            .transpositions
            .get(position)
            .unwrap()
            .filter(|p| p.hash == hash);

        result
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}
