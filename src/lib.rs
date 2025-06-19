#![feature(allocator_api)]

use std::{
    alloc::{AllocError, Allocator, Global, Layout},
    ptr::NonNull,
};

use hashbrown::{DefaultHashBuilder, HashSet};

#[inline(never)]
#[expect(non_snake_case)]
fn DOES_NOT_INLINE_ALLOCATE_FUNCTION() {
    unreachable!()
}

pub struct TestAllocator;

unsafe impl Allocator for TestAllocator {
    #[inline(always)]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        // The alignment is never 1024 in our cases, so if this function call shows up
        // in the assembly. That means that the layout's alignment is not statically
        // known.
        if layout.align() == 1024 {
            DOES_NOT_INLINE_ALLOCATE_FUNCTION();
        }

        Global.allocate(layout)
    }

    #[inline(always)]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe { Global.deallocate(ptr, layout) }
    }
}

pub fn vec_with_capacity(allocator: TestAllocator, capacity: usize) -> Vec<i32, TestAllocator> {
    Vec::with_capacity_in(capacity, allocator)
}

pub fn vec_push(vec: &mut Vec<i32, TestAllocator>, value: i32) {
    vec.push(value);
}

pub fn hash_set_with_capacity(
    allocator: TestAllocator,
    capacity: usize,
) -> HashSet<i32, DefaultHashBuilder, TestAllocator> {
    HashSet::with_capacity_in(capacity, allocator)
}

pub fn hash_set_insert(set: &mut HashSet<i32, DefaultHashBuilder, TestAllocator>, value: i32) {
    set.insert(value);
}
