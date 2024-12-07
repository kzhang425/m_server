use crate::blocklist::*;
use crate::bump::BumpBlock;
use crate::enums::*;
use core::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::replace;
pub struct StickyImmixHeap<H> {
    blocks: UnsafeCell<BlockList>,

    _header_type: PhantomData<H>, // For lifetime and defining the drop timings.
}

impl<H> StickyImmixHeap<H> {
    fn find_space(&self, alloc_size: usize, size_class: SizeClass) -> Result<*const u8, AllocError> {
        let blocks = unsafe { &mut *self.blocks.get()};

        // Look at the head field and determine how to handle it
        match blocks.get_head_mut() {
            None => {
                // Make a new one.
                replace(blocks.get_head_mut(), Some(BumpBlock::new()?));
            }

            _ => {

            }
        }

        unimplemented!();
    }
}