// fn main() {
//     println!("Debug assertions that pointers are non-null when required for soundness");
//     unsafe {
//         let _x = *std::ptr::null::<u8>();
//         let _x = &*std::ptr::null::<u8>();
//     }
// }

fn main() {
    println!("Debug assertions that pointers are non-null when required for soundness");

    unsafe {
        let ptr = std::ptr::null::<u8>();

        if ptr.is_null() {
            println!("Cannot dereference: ptr is null");
        } else {
            // This block will never run, just for illustration
            let x = *ptr;
            println!("{}", x);
        }

        // Same with a reference: this is invalid and must not be done
        // let _x = &*std::ptr::null::<u8>(); // UB!
        // println!("{}", _x);
    }
}
