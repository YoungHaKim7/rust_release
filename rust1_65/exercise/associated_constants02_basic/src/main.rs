trait ConstantID {
    const ID: i32;
}
struct Struct;

impl ConstantID for Struct {
    const ID: i32 = 1;
}

fn main() {
    assert_eq!(1, Struct::ID);
}
