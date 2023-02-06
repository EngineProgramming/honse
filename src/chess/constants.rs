pub const MAX_PLY: i16 = 128;

pub const INF: i16 = 32_001;
pub const NEG_INF: i16 = -32_001;

pub const MATE: i16 = 32_000;
pub const MATED: i16 = -32_000;

pub fn mated_in(ply: i16) -> i16 {
    ply - MATE
}
