use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Square {
    Empty = 0,
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Square::Empty => ".",
            Square::WhiteKing => "K",
            Square::WhiteQueen => "Q",
            Square::WhiteRook => "R",
            Square::WhiteBishop => "B",
            Square::WhiteKnight => "N",
            Square::WhitePawn => "P",
            Square::BlackKing => "k",
            Square::BlackQueen => "q",
            Square::BlackRook => "r",
            Square::BlackBishop => "b",
            Square::BlackKnight => "n",
            Square::BlackPawn => "p",
        };
        write!(f, "{}", c)
    }
}

pub struct Board {
    // 0 is A1, 1 is A2, 8 is B1, etc.
    pub squares: [Square; 64],
}

impl Board {
    pub fn new() -> Board {
        let mut squares = [Square::Empty; 64];

        squares[0] = Square::WhiteRook;
        squares[1] = Square::WhiteKnight;
        squares[2] = Square::WhiteBishop;
        squares[3] = Square::WhiteQueen;
        squares[4] = Square::WhiteKing;
        squares[5] = Square::WhiteBishop;
        squares[6] = Square::WhiteKnight;
        squares[7] = Square::WhiteRook;

        for idx in 0..8 {
            squares[idx + 8] = Square::WhitePawn;
            squares[idx + 6 * 8] = Square::BlackPawn;
        }

        squares[7 * 8] = Square::BlackRook;
        squares[7 * 8 + 1] = Square::BlackKnight;
        squares[7 * 8 + 2] = Square::BlackBishop;
        squares[7 * 8 + 3] = Square::BlackQueen;
        squares[7 * 8 + 4] = Square::BlackKing;
        squares[7 * 8 + 5] = Square::BlackBishop;
        squares[7 * 8 + 6] = Square::BlackKnight;
        squares[7 * 8 + 7] = Square::BlackRook;

        Board { squares }
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                write!(f, "{}", self.squares[rank * 8 + file])?;
            }
            if rank > 0 {
                f.write_str("\n")?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn squares_require_1_byte() {
        assert_eq!(size_of::<Square>(), 1);
    }
}
