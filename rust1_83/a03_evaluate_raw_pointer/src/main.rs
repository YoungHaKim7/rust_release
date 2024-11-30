static mut S: i32 = 64;
const C: *mut i32 = &raw mut S;

const fn inc(x: &mut i32) {
    *x += 1;
}

fn main() {
    const C: i32 = {
        let mut c = 41;
        inc(&mut c);
        c
    };

    println!("Value of c after inc: {}", C);
}
