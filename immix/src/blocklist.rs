use std::mem::replace;

use crate::bump::BumpBlock;
use crate::enums::*;


pub struct BlockList {
    head: Option<BumpBlock>,
    overflow: Option<BumpBlock>,
    rest: Vec<BumpBlock>,
}

impl BlockList {
    pub fn new() -> Self {
        Self {
            head: None,
            overflow: None,
            rest: Vec::new(),
        }
    }

    /// Called when we need to allocate to the overflow portion.
    pub fn overflow_alloc(&mut self, alloc_size: usize) -> Result<*const u8, AllocError> {
        // Depends on what the overflow field looks like
        match self.overflow {
            Some(ref mut overflow) => {
                match overflow.inner_alloc(alloc_size) {
                    Some(space) => Ok(space),

                    None => {
                        let previous = replace(overflow, BumpBlock::new()?);
                        self.rest.push(previous);
                        Ok(overflow.inner_alloc(alloc_size).expect("Unexpected error!"))
                    }
                }
            },

            None => {
                let mut overflow = BumpBlock::new()?;

                let space = overflow.inner_alloc(alloc_size).expect("Expected enough space here!");
                self.overflow = Some(overflow);
                Ok(space)
            }
        }
    }

    pub fn push_to_rest(&mut self, block: BumpBlock) {
        self.rest.push(block);
    }

    pub fn get_head_block(&self) -> &Option<BumpBlock> {
        &self.head
    }

    pub fn get_head_mut(&mut self) -> &mut Option<BumpBlock> {
        &mut self.head
    }

    pub fn get_rest_mut(&mut self) -> &mut Vec<BumpBlock> {
        &mut self.rest
    }
}