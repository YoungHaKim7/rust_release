trait Num {
    fn from_i32(n: i32) -> Self;
}

impl Num for f64 {
    fn from_i32(n: i32) -> f64 {
        n as f64
    }
}

fn main() {
    let _: f64 = Num::from_i32(42);
    let _: f64 = <_ as Num>::from_i32(42);
    let _: f64 = <f64 as Num>::from_i32(42);
    let _: f64 = f64::from_i32(42);
}
