use chess::ChessMove;

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
    pub following_move: Option<ChessMove>,
}

#[derive(Clone, Copy)]
pub enum Evaluation {
    Exact(Score),
    Beta(Score),
    Alpha(Score),
}

pub struct TranspositionTable {
    //transpositions: HashMap<Hash, TableEntry>,
    transpositions: Vec<Option<TableEntry>>,
    size: usize,
    num_hits: usize,
    num_misses: usize,
}

impl TranspositionTable {
    pub fn new() -> Self {
        let size = 1_000_000;
        let transpositions = vec![None; size];
        let num_hits = 0;
        let num_misses = 0;
        TranspositionTable {
            transpositions,
            size,
            num_hits,
            num_misses,
        }
    }

    pub fn len(&self) -> usize {
        self.transpositions.len()
    }

    pub fn is_empty(&self) -> bool {
        self.transpositions.is_empty()
    }

    pub fn hits(&self) -> usize {
        self.num_hits
    }

    pub fn misses(&self) -> usize {
        self.num_misses
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

        let result = self.transpositions.get(position).unwrap().filter(|p| {
            p.hash == hash
        });

        if result.is_some() {
            self.num_hits += 1;
        } else {
            self.num_misses += 1;
        }

        result
    }
}

impl Default for TranspositionTable {
    fn default() -> Self {
        Self::new()
    }
}
