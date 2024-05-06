struct Vec3(i32, i32, i32);

struct Mat3 {
    a: Vec3,
    b: Vec3,
    c: Vec3,
}

fn main() {
    let u8_ref: &u8 = &0u8;
    // let u64_ref: &u64 = unsafe { &*(u8_ref as *const u8 as *const u64) };
    //~^ ERROR casting references to a bigger memory layout is undefined behavior
    dbg!(u8_ref);

    let mat3 = Mat3 {
        a: Vec3(0i32, 0, 0),
        b: Vec3(0, 0, 0),
        c: Vec3(0, 0, 0),
    };
    // let mat3 = unsafe { &*(&mat3 as *const _ as *const [[i64; 3]; 3]) };
    //~^ ERROR casting references to a bigger memory layout is undefined behavior
    let mat3 = unsafe { &*(&mat3 as *const _ as *const [[i32; 3]; 3]) };
    dbg!(mat3);
}
