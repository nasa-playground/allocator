#![feature(const_mut_refs)]

mod bump_allocator;
mod linked_list_allocator;

#[global_allocator]
static ALLOCATOR: bump_allocator::BumpAllocator = bump_allocator::BumpAllocator::new();

fn main() {
    let s1 = format!("allocating a string1");
    let s2 = format!("allocating a string2");
    let b = Box::new(1);
    let v: Vec<usize> = vec![1, 2, 3];

    assert_eq!(s1, String::from("allocating a string1"));
    assert_eq!(s2, String::from("allocating a string2"));
    assert_eq!(*b, 1);
    assert_eq!(&v, &[1, 2, 3]);
}
