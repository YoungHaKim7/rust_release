trait Contains {
    type A;
    type B;

    // Updated syntax to refer to these new types generically.
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}

// Without using associated types
fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
}

// Using associated types
fn difference<C: Contains>(container: &C) -> i32 {}

fn main() {
    println!("Hello, world!");
}
