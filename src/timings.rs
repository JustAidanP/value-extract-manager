use std::fmt::Formatter;

/// A value if `i` is equivalent to saying `i` o'clock
pub type OClock = u8;

/// Defines an operational block, each block is exactly an hour long, and so the block can be
/// uniquely defined by its start time
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Block {
    starts_at: OClock,
}

impl Block {
    pub fn new(starts_at: OClock) -> Block {
        Block { starts_at }
    }
    /// The 'o'clock' of the starting time of this block
    pub fn starts_at(&self) -> OClock {
        self.starts_at
    }
    /// The 'o'clock' of the end of this block
    pub fn ends_at(&self) -> OClock {
        self.starts_at + 1
    }
}
