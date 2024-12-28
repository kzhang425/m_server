use std::ptr::NonNull;

use super::enums::{AllocError, SizeClass, Mark};
use super::rawptr::RawPtr;
use super::typedefs::*;
pub trait AllocRaw {
    type Header: AllocHeader;
    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>
    where 
        T: AllocObject<<Self::Header as AllocHeader>::TypeId>;

    fn alloc_array<T>(&self, size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError>;

    // Point is that the GC won't know types, so we'll have to do this.
    fn get_header(object: NonNull<()>) -> NonNull<Self::Header>;

    // Get object from header.
    fn get_object(header: NonNull<Self::Header>) -> NonNull<()>;
}

/// As long as it can be copied and cloned, we allow it as an identifier.
pub trait AllocTypeId: Copy + Clone {}

pub trait AllocObject<T: AllocTypeId> {
    const TYPE_ID: T;
}


/// The header trait that the interpreter shall use.
pub trait AllocHeader: Sized {
    type TypeId: AllocTypeId;

    fn new<O: AllocObject<Self::TypeId>>(
        size: u32,
        size_class: SizeClass,
        mark: Mark
    ) -> Self;

    fn new_array(size: ArraySize, size_class: SizeClass, mark: Mark) -> Self;

    // Purpose is to mark itself, must set itself to a status of "marked".
    fn mark(&mut self);

    fn is_marked(&self) -> bool;

    fn size_class(&self) -> SizeClass;

    fn size(&self) -> ObjectSize;

    fn type_id(&self) -> Self::TypeId;
}