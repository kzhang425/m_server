#![cfg(test)] // Configure this entire file for tests

use super::*;

#[test]
fn verifybits() {
    let size = 2;
    let mask = size - 1;
    let block = Block::new(size).unwrap();
    assert_eq!((block.ptr.as_ptr() as usize & mask) ^ mask, mask);
}