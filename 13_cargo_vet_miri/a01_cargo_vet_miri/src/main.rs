fn main() {
    let a = vec![1, 2, 3];

    let ptr_1 = a.as_ptr() as *mut i32;
    let mut ptr_1 = ptr_1 as usize;
    ptr_1 += std::mem::size_of::<i32>();

    let b = unsafe { Vec::from_raw_parts(ptr_1 as *mut i32, a.len() - 1, a.capacity() - 1) };

    println!("{b:?}");
}
