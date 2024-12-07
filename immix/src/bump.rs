use crate::{enums::MarkedState, misc::*};
use allocator::*;
use crate::enums::AllocError;

use super::consts;
pub struct BumpBlock {
    cursor: *const u8,
    limit: *const u8,
    block: Block,
    meta: BlockMeta,
}

impl BumpBlock {
    pub fn new() -> Result<Self, AllocError> {
        let block = match Block::new(consts::BLOCK_SIZE) {
            Ok(something) => something,
            Err(e) => return Err(map_block_to_alloc_err(e)),
        };
        let cursor = unsafe { block.as_ptr().add(consts::CURSOR_START_OFFSET) };
        let limit = cursor; // Point to the same place.
        let meta = BlockMeta {
            lines: unsafe { block.as_ptr().add(consts::BLOCK_CAPACITY) as *mut u8 },
        };

        // If we define it differently, this is where we would handle a fresh unmarked block. We set the last byte to mark it not live.
        
        let result = Self {
            cursor,
            limit,
            block,
            meta,
        };

        if !result.meta.mark_block_status(MarkedState::Unmarked) {
            return Err(AllocError::BadRequest);
        }
        Ok(result)
    }

    /// Inner alloc. The variable alloc_size is how many bytes to allocate.
    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let limit = self.limit as usize;
        let cursor_ptr = self.cursor as usize;

        // Here, we align to word boundary
        let align_mask = consts::ALIGN_MASK; // Basically makes a field of 1s and 0s. For 64 bit, usize is 8 bytes. The memory address must be a multiple of this.
        let next_ptr = cursor_ptr.checked_sub(alloc_size)? & align_mask; // basically, we slide back to the next best boundary.

        if next_ptr < limit {
            let block_relative_limit = unsafe {
                self.limit.sub(self.block.as_ptr() as usize)
            } as usize; // We should not get a negative value. Limit needs to lie within the block.

            if block_relative_limit > 0 {
                if let Some((cursor, limit)) = self.meta.find_next_available_hole(block_relative_limit, alloc_size) {
                    self.cursor = unsafe { self.block.as_ptr().add(cursor) };
                    self.limit = unsafe { self.block.as_ptr().add(limit) };
                    return self.inner_alloc(alloc_size);
                }
            }
            
            // At this point, we're most likely out of memory.
            None
        } else {
            self.cursor = next_ptr as *const u8;
            Some(self.cursor)
        }
    }

    /// Standalone function that can be used to check if a block is live or not.
    #[inline(always)]
    pub fn block_is_live(&self) -> bool {
        // Pretty much can always reference the last byte of the block.
        unsafe { *self.block.as_ptr().add(consts::BLOCK_SIZE - 1) != 0 }
    }


}

/// The BlockMeta struct is designed specifically to be a line mark section at the end of the block.
pub struct BlockMeta {
    lines: *mut u8,
}

impl BlockMeta {
    /// Core functionality function that finds the next hole. This is used recursively in inner_alloc.
    pub fn find_next_available_hole(&self, starting_at: usize, alloc_size: usize) -> Option<(usize, usize)> {
        let mut count = 0;
        let starting_line = starting_at / consts::LINE_SIZE;
        let lines_required = (alloc_size + consts::LINE_SIZE - 1) / consts::LINE_SIZE;
        
        let mut end = starting_line;

        for i in (0..starting_line).rev() {
            let marked = unsafe { self.get_marked_index(i) };

            if marked == 0 {
                count += 1;

                if i == 0 && count >= lines_required {
                    return Some((end * consts::LINE_SIZE, i * consts::LINE_SIZE));
                }
            } else {
                // Line is marked
                if count > lines_required {
                    return Some((end * consts::LINE_SIZE, (i + 2) * consts::LINE_SIZE));
                }

                // Otherwise, reset counts
                count = 0;
                end = i;
            }
        }
        None
    }

    /// Marks a line given the line number is 0-indexed. Outputs true if successful and false otherwise.
    #[inline]
    pub fn mark_line(&self, line_number: usize, value: MarkedState) -> bool {
        // Bounds checking
        if line_number > consts::LINE_COUNT - 1 {
            false
        } else {
            unsafe {
                let ptr = self.lines.add(line_number);
                *ptr = value.map_to_u8();
            }
            true
        }
    }

    /// Convenience function to mark the status of a block.
    #[inline(always)]
    pub fn mark_block_status(&self, value: MarkedState) -> bool {
        self.mark_line(consts::LINE_COUNT - 1, value)
    }

    /// Gets the marked byte according to the line number in the block. This is unchecked, so use with caution.
    #[inline(always)]
    pub unsafe fn get_marked_index(&self, line_number: usize) -> u8 {
        *self.lines.add(line_number)
    }


}