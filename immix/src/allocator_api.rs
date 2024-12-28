use super::enums::{AllocError, SizeClass, Mark};
use super::rawptr::RawPtr;
pub trait AllocRaw {
    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>;
}

/// As long as it can be copied and cloned, we allow it as an identifier.
pub trait AllocTypeId: Copy + Clone {}

pub trait AllocObject<T: AllocTypeId> {
    const TYPE_ID: T;
}

type ArraySize = u32;
type ObjectSize = u32;

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