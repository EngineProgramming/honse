#[derive(PartialEq, Debug)]
pub enum SearchOptions {
    Depth(u8),
    Movetime(u32),
    Nodes(u64),
    Time(u32, u32, Option<u32>, Option<u32>, Option<u32>),
    Infinite,
}
