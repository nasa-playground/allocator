use core::alloc::{GlobalAlloc, Layout};
use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

const MEM_SIZE: usize = 64 * 1024;

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator {
    mem: UnsafeCell::new([0u8; MEM_SIZE]),
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

struct BumpAllocator {
    mem: UnsafeCell<[u8; MEM_SIZE]>,
    next: AtomicUsize,
    allocations: AtomicUsize,
}

unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let next = self.next.load(Ordering::SeqCst);

        self.allocations.fetch_add(1, Ordering::SeqCst);
        self.next.fetch_add(layout.size(), Ordering::SeqCst);

        (self.mem.get() as *mut u8).add(next)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // do not something
    }
}
