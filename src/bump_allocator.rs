use core::alloc::{GlobalAlloc, Layout};
use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

pub const MEM_SIZE: usize = 6400 * 1024;

pub struct BumpAllocator {
    pub(crate) mem: UnsafeCell<[u8; MEM_SIZE]>,
    pub(crate) next: AtomicUsize,
    pub(crate) allocations: AtomicUsize,
}

unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocations.fetch_add(1, Ordering::SeqCst);
        self.next
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |next| {
                let align = layout.align();
                let size = layout.size();

                let size = calc_size(size, align);
                Some(next + size)
            })
            .unwrap();

        let next = self.next.load(Ordering::SeqCst);
        (self.mem.get() as *mut u8).add(next)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // do not something
    }
}

fn calc_size(size: usize, align: usize) -> usize {
    let size = if size % align == 0 {
        size
    } else {
        ((size / align) + 1) * align
    };

    size
}

#[test]
fn calc_size_test() {
    let test_cases = vec![(1, 8, 8), (2, 8, 8), (8, 8, 8), (9, 8, 16), (24, 8, 24)];

    for test_case in test_cases {
        let actual = calc_size(test_case.0, test_case.1);
        assert_eq!(actual, test_case.2)
    }
}
