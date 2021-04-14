use crate::color::Color;
use crate::error::Error;
use crate::file::File;
use crate::rank::Rank;
use std::fmt;
use std::str::FromStr;

macro_rules! gen_squares_data {
    ($macro:ident) => {
        $macro! {
            A1 = First  , A;
            B1 = First  , B;
            C1 = First  , C;
            D1 = First  , D;
            E1 = First  , E;
            F1 = First  , F;
            G1 = First  , G;
            H1 = First  , H;
            A2 = Second , A;
            B2 = Second , B;
            C2 = Second , C;
            D2 = Second , D;
            E2 = Second , E;
            F2 = Second , F;
            G2 = Second , G;
            H2 = Second , H;
            A3 = Third  , A;
            B3 = Third  , B;
            C3 = Third  , C;
            D3 = Third  , D;
            E3 = Third  , E;
            F3 = Third  , F;
            G3 = Third  , G;
            H3 = Third  , H;
            A4 = Fourth , A;
            B4 = Fourth , B;
            C4 = Fourth , C;
            D4 = Fourth , D;
            E4 = Fourth , E;
            F4 = Fourth , F;
            G4 = Fourth , G;
            H4 = Fourth , H;
            A5 = Fifth  , A;
            B5 = Fifth  , B;
            C5 = Fifth  , C;
            D5 = Fifth  , D;
            E5 = Fifth  , E;
            F5 = Fifth  , F;
            G5 = Fifth  , G;
            H5 = Fifth  , H;
            A6 = Sixth  , A;
            B6 = Sixth  , B;
            C6 = Sixth  , C;
            D6 = Sixth  , D;
            E6 = Sixth  , E;
            F6 = Sixth  , F;
            G6 = Sixth  , G;
            H6 = Sixth  , H;
            A7 = Seventh, A;
            B7 = Seventh, B;
            C7 = Seventh, C;
            D7 = Seventh, D;
            E7 = Seventh, E;
            F7 = Seventh, F;
            G7 = Seventh, G;
            H7 = Seventh, H;
            A8 = Eighth , A;
            B8 = Eighth , B;
            C8 = Eighth , C;
            D8 = Eighth , D;
            E8 = Eighth , E;
            F8 = Eighth , F;
            G8 = Eighth , G;
            H8 = Eighth , H;
        }
    };
}

const fn square_index(rank: Rank, file: File) -> isize {
    (rank as isize) << 3 ^ (file as isize)
}

macro_rules! gen_enum {
    ($($square:ident = $rank:ident, $file:ident;)*) => {
        /// Represent a square on the chess board
        #[derive(PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
        pub enum Square {
            $($square = square_index(Rank::$rank, File::$file)),*
        }
    }
}
gen_squares_data!(gen_enum);

macro_rules! gen_all_squares {
    ($($square:ident = $rank:ident, $file:ident;)*) => {
        /// A list of every square on the chessboard.
        ///
        /// ```
        /// use chess::{ALL_SQUARES, BitBoard, EMPTY};
        ///
        /// let universe = !EMPTY;
        ///
        /// let mut new_universe = EMPTY;
        ///
        /// for sq in ALL_SQUARES.iter() {
        ///     new_universe ^= BitBoard::from_square(*sq);
        /// }
        ///
        /// assert_eq!(new_universe, universe);
        /// ```
        pub const ALL_SQUARES: [Square; 64] = [$(Square::$square),*];
    }
}
gen_squares_data!(gen_all_squares);


/// How many squares are there?
pub const NUM_SQUARES: usize = ALL_SQUARES.len();

impl Default for Square {
    /// Create a square on A1.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let explicit_sq = Square::make_square(Rank::First, File::A);
    /// let implicit_sq = Square::default();
    ///
    /// assert_eq!(explicit_sq, implicit_sq);
    /// ```
    fn default() -> Square {
        Square::new(0)
    }
}

impl Square {
    /// Create a new square, given an index.
    /// Wrap on overflow (>= 64)
    ///
    /// ```
    ///
    /// use chess::{Square, Rank, File, EMPTY};
    ///
    /// assert_eq!(Square::new(0), Square::default());
    /// ```
    #[inline]
    pub fn new(sq: u8) -> Square {
        macro_rules! gen_match {
            ($($square:ident = $rank:ident, $file:ident;)*) => {
                {
                    $(const $square: u8 = Square::$square as u8;)*
                    match sq & 63 {
                        $($square => Square::$square,)*
                        _ => unreachable!()
                    }
                }
            }
        }
        gen_squares_data!(gen_match)
    }

    /// Make a square given a rank and a file
    ///
    /// ```
    /// use chess::{Square, Rank, File, BitBoard};
    ///
    /// // Make the A1 square
    /// let sq = Square::make_square(Rank::First, File::A);
    ///
    /// // Convert it to a bitboard
    /// let bb = BitBoard::from_square(sq);
    ///
    /// // loop over all squares in the bitboard (should be only one), and ensure that the square
    /// // is what we created
    /// for x in bb {
    ///     assert_eq!(sq, x);
    /// }
    /// ```
    #[inline]
    pub fn make_square(rank: Rank, file: File) -> Square {
        Self::new((rank as u8) << 3 ^ (file as u8))
    }

    /// Return the rank given this square.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.get_rank(), Rank::Seventh);
    /// ```
    #[inline]
    pub fn get_rank(self) -> Rank {
        Rank::from_index(self as usize >> 3)
    }

    /// Return the file given this square.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.get_file(), File::D);
    /// ```
    #[inline]
    pub fn get_file(self) -> File {
        File::from_index(self as usize & 7)
    }

    /// If there is a square above me, return that.  Otherwise, None.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.up().expect("Valid Square"), Square::make_square(Rank::Eighth, File::D));
    ///
    /// assert_eq!(sq.up().expect("Valid Square").up(), None);
    /// ```
    #[inline]
    pub fn up(&self) -> Option<Square> {
        if self.get_rank() == Rank::Eighth {
            None
        } else {
            Some(Square::make_square(self.get_rank().up(), self.get_file()))
        }
    }

    /// If there is a square below me, return that.  Otherwise, None.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Second, File::D);
    ///
    /// assert_eq!(sq.down().expect("Valid Square"), Square::make_square(Rank::First, File::D));
    ///
    /// assert_eq!(sq.down().expect("Valid Square").down(), None);
    /// ```
    #[inline]
    pub fn down(&self) -> Option<Square> {
        if self.get_rank() == Rank::First {
            None
        } else {
            Some(Square::make_square(self.get_rank().down(), self.get_file()))
        }
    }

    /// If there is a square to the left of me, return that.  Otherwise, None.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::B);
    ///
    /// assert_eq!(sq.left().expect("Valid Square"), Square::make_square(Rank::Seventh, File::A));
    ///
    /// assert_eq!(sq.left().expect("Valid Square").left(), None);
    /// ```
    #[inline]
    pub fn left(&self) -> Option<Square> {
        if self.get_file() == File::A {
            None
        } else {
            Some(Square::make_square(self.get_rank(), self.get_file().left()))
        }
    }

    /// If there is a square to the right of me, return that.  Otherwise, None.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::G);
    ///
    /// assert_eq!(sq.right().expect("Valid Square"), Square::make_square(Rank::Seventh, File::H));
    ///
    /// assert_eq!(sq.right().expect("Valid Square").right(), None);
    /// ```
    #[inline]
    pub fn right(&self) -> Option<Square> {
        if self.get_file() == File::H {
            None
        } else {
            Some(Square::make_square(
                self.get_rank(),
                self.get_file().right(),
            ))
        }
    }

    /// If there is a square "forward", given my `Color`, go in that direction.  Otherwise, None.
    ///
    /// ```
    /// use chess::{Square, Rank, File, Color};
    ///
    /// let mut sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.forward(Color::White).expect("Valid Square"), Square::make_square(Rank::Eighth, File::D));
    /// assert_eq!(sq.forward(Color::White).expect("Valid Square").forward(Color::White), None);
    ///
    /// sq = Square::make_square(Rank::Second, File::D);
    ///
    /// assert_eq!(sq.forward(Color::Black).expect("Valid Square"), Square::make_square(Rank::First, File::D));
    /// assert_eq!(sq.forward(Color::Black).expect("Valid Square").forward(Color::Black), None);
    /// ```
    #[inline]
    pub fn forward(&self, color: Color) -> Option<Square> {
        match color {
            Color::White => self.up(),
            Color::Black => self.down(),
        }
    }

    /// If there is a square "backward" given my `Color`, go in that direction.  Otherwise, None.
    ///
    /// ```
    /// use chess::{Square, Rank, File, Color};
    ///
    /// let mut sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.backward(Color::Black).expect("Valid Square"), Square::make_square(Rank::Eighth, File::D));
    /// assert_eq!(sq.backward(Color::Black).expect("Valid Square").backward(Color::Black), None);
    ///
    /// sq = Square::make_square(Rank::Second, File::D);
    ///
    /// assert_eq!(sq.backward(Color::White).expect("Valid Square"), Square::make_square(Rank::First, File::D));
    /// assert_eq!(sq.backward(Color::White).expect("Valid Square").backward(Color::White), None);
    /// ```
    #[inline]
    pub fn backward(&self, color: Color) -> Option<Square> {
        match color {
            Color::White => self.down(),
            Color::Black => self.up(),
        }
    }

    /// If there is a square above me, return that.  If not, wrap around to the other side.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.uup(), Square::make_square(Rank::Eighth, File::D));
    ///
    /// assert_eq!(sq.uup().uup(), Square::make_square(Rank::First, File::D));
    /// ```
    #[inline]
    pub fn uup(&self) -> Square {
        Square::make_square(self.get_rank().up(), self.get_file())
    }

    /// If there is a square below me, return that.  If not, wrap around to the other side.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Second, File::D);
    ///
    /// assert_eq!(sq.udown(), Square::make_square(Rank::First, File::D));
    ///
    /// assert_eq!(sq.udown().udown(), Square::make_square(Rank::Eighth, File::D));
    /// ```
    #[inline]
    pub fn udown(&self) -> Square {
        Square::make_square(self.get_rank().down(), self.get_file())
    }

    /// If there is a square to the left of me, return that. If not, wrap around to the other side.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::B);
    ///
    /// assert_eq!(sq.uleft(), Square::make_square(Rank::Seventh, File::A));
    ///
    /// assert_eq!(sq.uleft().uleft(), Square::make_square(Rank::Seventh, File::H));
    /// ```
    #[inline]
    pub fn uleft(&self) -> Square {
        Square::make_square(self.get_rank(), self.get_file().left())
    }

    /// If there is a square to the right of me, return that.  If not, wrap around to the other
    /// side.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// let sq = Square::make_square(Rank::Seventh, File::G);
    ///
    /// assert_eq!(sq.uright(), Square::make_square(Rank::Seventh, File::H));
    ///
    /// assert_eq!(sq.uright().uright(), Square::make_square(Rank::Seventh, File::A));
    /// ```
    #[inline]
    pub fn uright(&self) -> Square {
        Square::make_square(self.get_rank(), self.get_file().right())
    }

    /// If there is a square "forward", given my color, return that.  If not, wrap around to the
    /// other side.
    ///
    /// ```
    /// use chess::{Square, Rank, File, Color};
    ///
    /// let mut sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.uforward(Color::White), Square::make_square(Rank::Eighth, File::D));
    /// assert_eq!(sq.uforward(Color::White).uforward(Color::White), Square::make_square(Rank::First, File::D));
    ///
    /// sq = Square::make_square(Rank::Second, File::D);
    ///
    /// assert_eq!(sq.uforward(Color::Black), Square::make_square(Rank::First, File::D));
    /// assert_eq!(sq.uforward(Color::Black).uforward(Color::Black), Square::make_square(Rank::Eighth, File::D));
    /// ```
    #[inline]
    pub fn uforward(&self, color: Color) -> Square {
        match color {
            Color::White => self.uup(),
            Color::Black => self.udown(),
        }
    }

    /// If there is a square "backward", given my color, return that.  If not, wrap around to the
    /// other side.
    ///
    /// ```
    /// use chess::{Square, Rank, File, Color};
    ///
    /// let mut sq = Square::make_square(Rank::Seventh, File::D);
    ///
    /// assert_eq!(sq.ubackward(Color::Black), Square::make_square(Rank::Eighth, File::D));
    /// assert_eq!(sq.ubackward(Color::Black).ubackward(Color::Black), Square::make_square(Rank::First, File::D));
    ///
    /// sq = Square::make_square(Rank::Second, File::D);
    ///
    /// assert_eq!(sq.ubackward(Color::White), Square::make_square(Rank::First, File::D));
    /// assert_eq!(sq.ubackward(Color::White).ubackward(Color::White), Square::make_square(Rank::Eighth, File::D));
    /// ```
    #[inline]
    pub fn ubackward(&self, color: Color) -> Square {
        match color {
            Color::White => self.udown(),
            Color::Black => self.uup(),
        }
    }

    /// Convert this square to an integer.
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// assert_eq!(Square::make_square(Rank::First, File::A).to_int(), 0);
    /// assert_eq!(Square::make_square(Rank::Second, File::A).to_int(), 8);
    /// assert_eq!(Square::make_square(Rank::First, File::B).to_int(), 1);
    /// assert_eq!(Square::make_square(Rank::Eighth, File::H).to_int(), 63);
    /// ```
    #[inline]
    pub fn to_int(self) -> u8 {
        self as u8
    }

    /// Convert this `Square` to a `usize` for table lookup purposes
    ///
    /// ```
    /// use chess::{Square, Rank, File};
    ///
    /// assert_eq!(Square::make_square(Rank::First, File::A).to_index(), 0);
    /// assert_eq!(Square::make_square(Rank::Second, File::A).to_index(), 8);
    /// assert_eq!(Square::make_square(Rank::First, File::B).to_index(), 1);
    /// assert_eq!(Square::make_square(Rank::Eighth, File::H).to_index(), 63);
    /// ```
    #[inline]
    pub fn to_index(self) -> usize {
        self as usize
    }

    /// Convert a UCI `String` to a square.  If invalid, return `None`
    ///
    /// ```
    /// use chess::Square;
    ///
    /// let sq = Square::default();
    ///
    /// assert_eq!(Square::from_string("a1".to_owned()).expect("Valid Square"), sq);
    /// ```
    #[deprecated(
        since = "3.1.0",
        note = "please use `Square::from_str(square)?` instead"
    )]
    pub fn from_string(s: String) -> Option<Square> {
        Square::from_str(&s).ok()
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{}",
            (('a' as u8) + (self.get_file() as u8)) as char,
            (('1' as u8) + (self.get_rank() as u8)) as char
        )
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(Error::InvalidSquare);
        }
        let ch: Vec<char> = s.chars().collect();
        match ch[0] {
            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' => {}
            _ => {
                return Err(Error::InvalidSquare);
            }
        }
        match ch[1] {
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {}
            _ => {
                return Err(Error::InvalidSquare);
            }
        }
        Ok(Square::make_square(
            Rank::from_index((ch[1] as usize) - ('1' as usize)),
            File::from_index((ch[0] as usize) - ('a' as usize)),
        ))
    }
}
