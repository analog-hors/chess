use crate::castle_rights::CastleRights;
use crate::color::{Color, NUM_COLORS};
use crate::file::{File, NUM_FILES};
use crate::piece::{Piece, NUM_PIECES};
use crate::square::{Square, NUM_SQUARES};

/// Create a completely blank type.  This allows all the functions to be part of this type, which I
/// think is a bit cleaner than bare functions everywhere.
pub struct Zobrist;

// Include the generated lookup tables
include!(concat!(env!("OUT_DIR"), "/zobrist_gen.rs"));

impl Zobrist {
    /// Get the value for a particular piece
    #[inline]
    pub fn piece(piece: Piece, square: Square, color: Color) -> u64 {
        ZOBRIST_PIECES[color.to_index()][piece.to_index()][square.to_index()]
    }

    #[inline]
    pub fn castles(castle_rights: CastleRights, color: Color) -> u64 {
        ZOBRIST_CASTLES[color.to_index()][castle_rights.to_index()]
    }

    #[inline]
    pub fn en_passant(file: File, color: Color) -> u64 {
        ZOBRIST_EP[color.to_index()][file.to_index()]
    }

    #[inline]
    pub fn color() -> u64 {
        SIDE_TO_MOVE
    }
}
