#[derive(PartialEq, Debug, Copy, Clone)]
pub enum SearchOption {
    Depth(u8),
    Movetime(u32),
    Nodes(u64),
    //    wt,  bt,       winc,        binc,    movestogo
    Time(u32, u32, Option<u32>, Option<u32>, Option<u32>),
    Infinite,
}
