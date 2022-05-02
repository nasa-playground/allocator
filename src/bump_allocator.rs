use core::alloc::{GlobalAlloc, Layout};
use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

pub const MEM_SIZE: usize = 128 * 1024;

pub struct BumpAllocator {
    pub(crate) mem: UnsafeCell<[u8; MEM_SIZE]>,
    pub(crate) next: AtomicUsize,
}

unsafe impl Sync for BumpAllocator {}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let next = self.next.load(Ordering::SeqCst);
        let res = (self.mem.get() as *mut u8).add(next);

        let align = layout.align();
        let size = layout.size();
        self.next
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |mut next| {
                next += size;
                Some(alignment_correction(next, align))
            })
            .unwrap();

        res
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // do not something
    }
}

fn alignment_correction(size: usize, align: usize) -> usize {
    (size + align - 1) & !(align - 1)
}

#[test]
fn alignment_correction_test() {
    let test_cases = vec![(1, 8, 8), (2, 8, 8), (8, 8, 8), (9, 8, 16), (24, 8, 24)];

    for test_case in test_cases {
        let actual = alignment_correction(test_case.0, test_case.1);
        assert_eq!(actual, test_case.2)
    }
}
