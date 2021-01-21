use std::fmt;

const WHITE_SYMBOLS: [u8; 16] = [b'K', b'Q', b'R', b'R', b'B', b'B', b'N', b'N', b'P', b'P', b'P', b'P', b'P', b'P', b'P', b'P'];
const BLACK_SYMBOLS: [u8; 16] = [b'k', b'q', b'r', b'r', b'b', b'b', b'n', b'n', b'p', b'p', b'p', b'p', b'p', b'p', b'p', b'p'];

pub struct Board {
    /// king, queen, rooks, bishops, knights, pawns
    pub white_pieces: [u8; 16],
    pub black_pieces: [u8; 16],
}

impl Board {
    pub fn new() -> Board {
        Board {
            white_pieces: [0x04, 0x03, 0x00, 0x07, 0x02, 0x05, 0x01, 0x06, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17],
            black_pieces: [0x74, 0x73, 0x70, 0x77, 0x72, 0x75, 0x71, 0x76, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67],
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut repr = String::from("--------\n--------\n--------\n--------\n--------\n--------\n--------\n--------\n").into_bytes();

        for (idx, &loc) in self.white_pieces.iter().enumerate() {
            if on_board(loc) {
                let rank = (loc & 0xF0) >> 4;
                let file = loc & 0x0F;
                repr[((7-rank)*9 + file) as usize] = WHITE_SYMBOLS[idx];
            }
        }

        for (idx, &loc) in self.black_pieces.iter().enumerate() {
            if on_board(loc) {
                let rank = (loc & 0xF0) >> 4;
                let file = loc & 0x0F;
                repr[((7-rank)*9 + file) as usize] = BLACK_SYMBOLS[idx];
            }
        }

        write!(f, "{}", String::from_utf8(repr).unwrap())
    }
}

fn on_board(loc: u8) -> bool {
    (loc & 0x88) == 0
}
