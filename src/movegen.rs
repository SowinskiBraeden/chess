use crate::Color;

// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// . . x x x . . .
// . . . P . . . .
// . . . . . . . .
// . . . . . . . .
// . . . . . . . .
// Pawn (P) on square 27 can move to any (x)
// sitting at square +7, +8, +9,
// if P is in rank 2, then it can also do square +16

pub fn pawn_attacks(square: u8, color: Color) -> u64 {
    assert!(square < 64, "invalid square index");

    let MAX_DIFF: i16 = 1;
    let diffs: [i16; 2] = match color {
        Color::White => [7, 9],
        Color::Black => [-7, -9],
    };

    let square = i16::from(square);
    let file = square % 8;
    let mut mask = 0u64;

    for diff in diffs {
        let dest = square + diff;
        if !(0..64).contains(&dest) || (dest % 8 - file).abs() != MAX_DIFF {
            continue;
        }
        mask |= 1u64 << dest;
    }

    mask
}

// . . . . . . . .
// . . . . . . . .
// . . x . x . . .
// . x . . . x . .
// . . . N . . . .
// . x . . . x . .
// . . x . x . . .
// . . . . . . . .
// Knight (N) on square 27 can jump to any (x)
// sitting at square +6, +10, +15, +17,
//                   -6, -10, -15, -17

pub fn knight_attacks(square: u8) -> u64 {
    assert!(square < 64, "invalid square index");

    const MAX_DIFF: i16 = 2;
    const DIFFS: [i16; 8] = [6, 10, 15, 17, -6, -10, -15, -17];

    let square = i16::from(square);
    let file = square % 8;
    let mut mask = 0u64;

    for diff in DIFFS {
        let dest = square + diff;
        let new_file = dest % 8;

        // checks going beyond rank or wrapping files
        if !(0..64).contains(&dest) || (new_file - file).abs() > MAX_DIFF {
            continue;
        }

        mask |= 1u64 << dest;
    }

    mask
}

// . . . . . . . x
// x . . . . . x .
// . x . . . x . .
// . . x . x . . .
// . . . B . . . .
// . . x . x . . .
// . x . . . x . .
// x . . . . . x .
// Bishop (B) on square 27 can jump to any (x)
// sitting at square +7, +9, -7, -9, repeating

pub fn bishop_attacks(square: u8) -> u64 {
    assert!(square < 64, "invalid square index");

    const MAX_DIFF: i16 = 1;
    const DIFFS: [i16; 4] = [7, 9, -7, -9];

    let square = i16::from(square);
    let mut mask = 0u64;

    for diff in DIFFS {
        let mut last = square;
        let mut dest = last + diff;

        while (0..64).contains(&dest) && (dest % 8 - last % 8).abs() == MAX_DIFF {
            mask |= 1u64 << dest;
            last = dest;
            dest += diff;
        }
    }

    mask
}

pub fn rook_attacks(square: u8) -> u64 {
    assert!(square < 64, "invalid square index");

    const MAX_DIFF: i16 = 1;
    const DIFFS: [i16; 4] = [8, 1, -8, -1];

    let square = i16::from(square);
    let mut mask = 0u64;

    for diff in DIFFS {
        let mut last = square;
        let mut dest = last + diff;

        while (0..64).contains(&dest) && (dest % 8 - last % 8).abs() <= MAX_DIFF {
            mask |= 1u64 << dest;
            last = dest;
            dest += diff;
        }
    }

    mask
}
