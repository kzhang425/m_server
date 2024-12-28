use super::allocator_api;


#[derive(Debug, PartialEq)]
pub enum AllocError {
    BadRequest,
    OOM,
}

#[derive(Debug, PartialEq)]
pub enum SizeClass {
    Large,
    Medium,
    Small,
}

/// Purpose of this is to have a good way to interpret and work with the marking of lines in each block. Cannot be bigger than 128 entries.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MarkedState {
    Unmarked,
    TrueMarked,
    ConservMarked,
}

impl MarkedState {
    pub fn map_to_u8(&self) -> u8 {
        *self as u8
    }
}

/// Enum defining what it means to have a line in a block marked or not.
pub enum Mark {
    Allocated,
    Unmarked,
    Marked,
}