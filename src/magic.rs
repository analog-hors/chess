use crate::bitboard::{BitBoard, EMPTY};
use crate::color::Color;
use crate::file::File;
use crate::rank::Rank;
use crate::square::{Square, NUM_SQUARES};
#[cfg(target_feature = "bmi2")]
use std::arch::x86_64::{_pdep_u64, _pext_u64};

use static_assertions::const_assert;

//TODO remove remaining unsafe blocks in here.
//the compiler can't elide them, do we take the bounds check anyway?
//if some of the tables were matches it probably could elide them.

// Include the generated lookup tables
include!(concat!(env!("OUT_DIR"), "/magic_gen.rs"));

/// Get the rays for a bishop on a particular square.
#[inline]
pub fn get_bishop_rays(sq: Square) -> BitBoard {
    RAYS[BISHOP][sq.to_index()]
}

/// Get the rays for a rook on a particular square.
#[inline]
pub fn get_rook_rays(sq: Square) -> BitBoard {
    RAYS[ROOK][sq.to_index()]
}

#[allow(unused)]
const fn table_access_is_sound(index: usize) -> bool {
    let mut sq = 0;
    while sq < NUM_SQUARES {
        let magic = MAGIC_NUMBERS[index][sq];
        let max_index = ((magic.magic_number.0 * magic.mask.0) >> magic.rightshift) as usize;
        if magic.offset as usize + max_index >= MOVES.len() {
            return false;
        }
        sq += 1;
    }
    true
}

#[allow(unused)]
#[cfg(target_feature = "bmi2")]
const fn bmi_table_access_is_sound(masks: &[BmiMagic; NUM_SQUARES]) -> bool {
    let mut sq = 0;
    while sq < NUM_SQUARES {
        let bmi2_magic = masks[sq];
        //Simulate pext on a full bitboard
        let max_index = ((1u64 << bmi2_magic.blockers_mask.0.count_ones()) - 1) as usize;
        if bmi2_magic.offset as usize + max_index >= BMI_MOVES.len() {
            return false;
        }
        sq += 1;
    }
    true
}

/// Get the moves for a rook on a particular square, given blockers blocking my movement.
#[inline]
pub fn get_rook_moves(sq: Square, blockers: BitBoard) -> BitBoard {
    const_assert!(table_access_is_sound(ROOK));
    //SAFETY: Covered by the soundness check above.
    unsafe {
        let magic = MAGIC_NUMBERS[ROOK][sq.to_index()];
        *MOVES.get_unchecked(
            (magic.offset as usize)
                + (magic.magic_number * (blockers & magic.mask)).to_size(magic.rightshift),
        ) & get_rook_rays(sq)
    }
}

/// Get the moves for a rook on a particular square, given blockers blocking my movement.
#[cfg(target_feature = "bmi2")]
#[inline]
pub fn get_rook_moves_bmi(sq: Square, blockers: BitBoard) -> BitBoard {
    const_assert!(bmi_table_access_is_sound(&ROOK_BMI_MASK));
    //SAFETY: Covered by the soundness check above.
    unsafe {
        let bmi2_magic = ROOK_BMI_MASK[sq.to_index()];
        let index = (_pext_u64(blockers.0, bmi2_magic.blockers_mask.0) as usize)
            + (bmi2_magic.offset as usize);
        let result = _pdep_u64(
            *BMI_MOVES.get_unchecked(index as usize) as u64,
            get_rook_rays(sq).0,
        );
        BitBoard(result)
    }
}

/// Get the moves for a bishop on a particular square, given blockers blocking my movement.
#[inline]
pub fn get_bishop_moves(sq: Square, blockers: BitBoard) -> BitBoard {
    const_assert!(table_access_is_sound(BISHOP));
    //SAFETY: Covered by the soundness check above.
    unsafe {
        let magic: Magic = MAGIC_NUMBERS[BISHOP][sq.to_index()];
        *MOVES.get_unchecked(
            (magic.offset as usize)
                + (magic.magic_number * (blockers & magic.mask)).to_size(magic.rightshift),
        ) & get_bishop_rays(sq)
    }
}

/// Get the moves for a bishop on a particular square, given blockers blocking my movement.
#[inline]
#[cfg(target_feature = "bmi2")]
pub fn get_bishop_moves_bmi(sq: Square, blockers: BitBoard) -> BitBoard {
    const_assert!(bmi_table_access_is_sound(&BISHOP_BMI_MASK));
    //SAFETY: Covered by the soundness check above.
    unsafe {
        let bmi2_magic = BISHOP_BMI_MASK[sq.to_index()];
        let index = (_pext_u64(blockers.0, bmi2_magic.blockers_mask.0) as usize)
            + (bmi2_magic.offset as usize);
        let result = _pdep_u64(
            *BMI_MOVES.get_unchecked(index as usize) as u64,
            get_bishop_rays(sq).0,
        );
        BitBoard(result)
    }
}

/// Get the king moves for a particular square.
#[inline]
pub fn get_king_moves(sq: Square) -> BitBoard {
    KING_MOVES[sq.to_index()]
}

/// Get the knight moves for a particular square.
#[inline]
pub fn get_knight_moves(sq: Square) -> BitBoard {
    KNIGHT_MOVES[sq.to_index()]
}

/// Get the pawn capture move for a particular square, given the pawn's color and the potential
/// victims
#[inline]
pub fn get_pawn_attacks(sq: Square, color: Color, blockers: BitBoard) -> BitBoard {
    PAWN_ATTACKS[color.to_index()][sq.to_index()] & blockers
}
/// Get the legal destination castle squares for both players
#[inline]
pub fn get_castle_moves() -> BitBoard {
    CASTLE_MOVES
}

/// Get the quiet pawn moves (non-captures) for a particular square, given the pawn's color and
/// the potential blocking pieces.
#[inline]
pub fn get_pawn_quiets(sq: Square, color: Color, blockers: BitBoard) -> BitBoard {
    if (BitBoard::from_square(sq.uforward(color)) & blockers) != EMPTY {
        EMPTY
    } else {
        PAWN_MOVES[color.to_index()][sq.to_index()] & !blockers
    }
}

/// Get all the pawn moves for a particular square, given the pawn's color and the potential
/// blocking pieces and victims.
#[inline]
pub fn get_pawn_moves(sq: Square, color: Color, blockers: BitBoard) -> BitBoard {
    get_pawn_attacks(sq, color, blockers) ^ get_pawn_quiets(sq, color, blockers)
}

/// Get a line (extending to infinity, which in chess is 8 squares), given two squares.
/// This line does extend past the squares.
#[inline]
pub fn line(sq1: Square, sq2: Square) -> BitBoard {
    LINE[sq1.to_index()][sq2.to_index()]
}

/// Get a line between these two squares, not including the squares themselves.
#[inline]
pub fn between(sq1: Square, sq2: Square) -> BitBoard {
    BETWEEN[sq1.to_index()][sq2.to_index()]
}

/// Get a `BitBoard` that represents all the squares on a particular rank.
#[inline]
pub fn get_rank(rank: Rank) -> BitBoard {
    RANKS[rank.to_index()]
}

/// Get a `BitBoard` that represents all the squares on a particular file.
#[inline]
pub fn get_file(file: File) -> BitBoard {
    FILES[file.to_index()]
}

/// Get a `BitBoard` that represents the squares on the 1 or 2 files next to this file.
#[inline]
pub fn get_adjacent_files(file: File) -> BitBoard {
    ADJACENT_FILES[file.to_index()]
}

#[inline]
pub fn get_pawn_source_double_moves() -> BitBoard {
    PAWN_SOURCE_DOUBLE_MOVES
}

#[inline]
pub fn get_pawn_dest_double_moves() -> BitBoard {
    PAWN_DEST_DOUBLE_MOVES
}
