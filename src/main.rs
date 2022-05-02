mod bump_allocator;
mod simple_allocator;

use std::{cell::UnsafeCell, sync::atomic::AtomicUsize};

// bump allocator
// #[global_allocator]
// static ALLOCATOR: bump_allocator::BumpAllocator = bump_allocator::BumpAllocator {
//     mem: UnsafeCell::new([0u8; bump_allocator::MEM_SIZE]),
//     next: AtomicUsize::new(0),
//     allocations: AtomicUsize::new(0),
// };

// simple allocator
#[global_allocator]
static ALLOCATOR: simple_allocator::SimpleAllocator = simple_allocator::SimpleAllocator {
    arena: UnsafeCell::new([0x55; simple_allocator::ARENA_SIZE]),
    remaining: AtomicUsize::new(simple_allocator::ARENA_SIZE),
};

fn main() {
    let s1 = format!("allocating a string1");
    let s2 = format!("allocating a string2");
    let _b = Box::new(1);
    let _v: Vec<usize> = vec![1, 2, 3];

    println!("{s1}");
    println!("{s2}");

    // unsafe {
    //     let mem = ALLOCATOR.mem.get();
    //     println!("{:?}", mem.as_ref());
    // }
}
