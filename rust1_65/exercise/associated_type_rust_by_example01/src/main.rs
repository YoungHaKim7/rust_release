// The use of "Associated types" improves the overall readability of code by moving inner types locally into a trait as output types.
// https://doc.rust-lang.org/rust-by-example/generics/assoc_items/types.html

// `A` and `B` are defined in the trait via the `type` keyword.
// (Note: `type` in this context is different from `type` when used for aliases.)
trait Contains {
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}

fn main() {
    println!("Hello, world!");
}
