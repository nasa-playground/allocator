use core::alloc::{GlobalAlloc, Layout};
use std::{
    cell::UnsafeCell,
    sync::atomic::{AtomicUsize, Ordering},
};

pub const MEM_SIZE: usize = 64 * 1024;

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
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |mut next| {
                let align = layout.align();
                let size = layout.size();

                let size = if size % align != 0 {
                    ((size / align) + 1) * align
                } else {
                    size
                };
                next += size + 1;
                Some(next)
            })
            .unwrap();

        let next = self.next.load(Ordering::SeqCst);
        (self.mem.get() as *mut u8).add(next)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // do not something
    }
}
