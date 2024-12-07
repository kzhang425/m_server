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

/// Purpose of this is to have a good way to interpret and work with the marking of lines in each block.
#[derive(Debug, PartialEq)]
pub enum MarkedState {
    Unmarked,
    TrueMarked,
    ConservMarked,
}

impl MarkedState {
    pub fn map_to_u8(&self) -> u8 {
        match self {
            Self::Unmarked => 0,
            Self::TrueMarked => 1,
            Self::ConservMarked => 2,
        }
    }
}