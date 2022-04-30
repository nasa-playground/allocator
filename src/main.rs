mod bump_allocator;

use std::{cell::UnsafeCell, sync::atomic::AtomicUsize};

#[global_allocator]
static ALLOCATOR: bump_allocator::BumpAllocator = bump_allocator::BumpAllocator {
    mem: UnsafeCell::new([0u8; bump_allocator::MEM_SIZE]),
    next: AtomicUsize::new(0),
    allocations: AtomicUsize::new(0),
};

fn main() {
    let s1 = format!("allocating a string1");
    let s2 = format!("allocating a string2");
    let _v: Vec<usize> = vec![];

    println!("{s1}");
    println!("{s2}");

    // 要素があるとき`bus error`になる
    // なんでだろう
    // let _v: Vec<usize> = vec![1, 2, 3];
}
