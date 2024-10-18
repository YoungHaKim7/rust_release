#[repr(packed)]
struct Packed {
    not_aligned_field: i32,
}

fn main() {
    let p = Packed {
        not_aligned_field: 1_82,
    };

    // This would be undefined behavior!
    // It is rejected by the compiler.
    //let ptr = &p.not_aligned_field as *const i32;

    // This is the old way of creating a pointer.
    let ptr = std::ptr::addr_of!(p.not_aligned_field);

    // This is the new way.
    let ptr = &raw const p.not_aligned_field;

    // Accessing the pointer has not changed.
    // Note that `val = *ptr` would be undefined behavior because
    // the pointer is not aligned!
    let val = unsafe { ptr.read_unaligned() };
}
