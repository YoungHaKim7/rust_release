// use std::fmt::Debug;

pub struct MyValue {
    x: u32,
}

#[no_mangle]
pub extern "C" fn myvalue_create() -> *mut MyValue {
    let v = Box::new(MyValue { x: 4u32 });
    Box::into_raw(v)
}

#[no_mangle]
pub extern "C" fn myvalue_free(v_ptr: *mut MyValue) {
    let v = unsafe { Box::from_raw(v_ptr) };
    drop(v); // be explicit to show intent
}

#[no_mangle]
pub extern "C" fn myvalue_print(v_ptr: *mut MyValue) {
    // Turn it into a &MyValue
    let v = unsafe { &*v_ptr };
    println!("MyValue content: {:?}", v.x);
}
