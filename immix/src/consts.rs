pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;

// For lines
pub const LINE_SIZE_BITS: usize = 7;
pub const LINE_SIZE: usize = 1 << LINE_SIZE_BITS; // Use this for the number of bytes in a line.

pub const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;
pub const BLOCK_CAPACITY: usize = BLOCK_SIZE - LINE_COUNT; // Need to allocate some space to store markings for lines.
pub const CURSOR_START_OFFSET: usize = (BLOCK_CAPACITY / LINE_SIZE) * LINE_SIZE;

pub const ALIGN_MASK: usize = !(size_of::<usize>() - 1);