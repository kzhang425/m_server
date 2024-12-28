use crate::enums::*;
use allocator::BlockError;

pub fn map_block_to_alloc_err(block_err: BlockError) -> AllocError {
    match block_err {
        BlockError::BadRequest => AllocError::BadRequest,
        BlockError::OOM => AllocError::OOM,
    }
}