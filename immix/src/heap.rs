use crate::blocklist::*;
use crate::bump::BumpBlock;
use crate::enums::*;
use core::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::replace;

pub struct StickyImmixHeap<H> {
    blocks: UnsafeCell<BlockList>,

    _header_type: PhantomData<*const H>, // For lifetime and defining the drop timings.
}

impl<H> StickyImmixHeap<H> {
    /// Creates a new StickyImmixHeap instance.
    pub fn new() -> StickyImmixHeap<H> {
        Self {
            blocks: UnsafeCell::new(BlockList::new()),
            _header_type: PhantomData,
        }
    }

    fn find_space(&self, alloc_size: usize, size_class: SizeClass) -> Result<*const u8, AllocError> {
        let blocks = unsafe { &mut *self.blocks.get()};

        // We will need to handle large objects later.
        if size_class == SizeClass::Large {
            return Err(AllocError::BadRequest);
        }

        // Look at the head field and determine how to handle it
        let space = match blocks.get_head_block() {
            None => {
                // Make a new one.
                let mut head = BumpBlock::new()?;

                let space = head
                    .inner_alloc(alloc_size)
                    .expect("We expected this object to fit!");
                
                *blocks.get_head_mut() = Some(head);
                space
            }

            Some(_) => {
                // Handle the medium size case, if it doesn't fit in the hole.
                if size_class == SizeClass::Medium && blocks.get_head_block().as_ref().unwrap().current_hole_size() < alloc_size {
                    return blocks.overflow_alloc(alloc_size);
                }

                // Otherwise, this is a small object that fits in the block.
                match blocks.get_head_mut().as_mut().unwrap().inner_alloc(alloc_size) {
                    Some(space) => space,

                    None => {
                        let previous = replace(blocks.get_head_mut().as_mut().unwrap(), BumpBlock::new()?);
                        blocks.push_to_rest(previous);
                        blocks.get_head_mut().as_mut().unwrap().inner_alloc(alloc_size).expect("At this point, we've failed twice.")
                    }
                }
            }
        } as *const u8;

        Ok(space)
    }
}