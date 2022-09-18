#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct BytePos(pub u32);

const DUMMY_RESERVE: u32 = u32::MAX - 2_u32.pow(16);

impl BytePos {
    /// Dummy position. This is reserved for synthesized spans.
    pub const DUMMY: Self = BytePos(0);
    const MIN_RESERVED: Self = BytePos(DUMMY_RESERVE);

    pub const fn is_reserved_for_comments(self) -> bool {
        self.0 >= Self::MIN_RESERVED.0 && self.0 != u32::MAX
    }

    /// Returns true if this is synthesized and has no relevant input source
    /// code.
    pub const fn is_dummy(self) -> bool {
        self.0 == 0
    }
}
