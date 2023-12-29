use std::num::Wrapping;

trait Trait<'a> {
    type Assoc;
    fn new() -> Self::Assoc;
}

impl Trait<'_> for () {
    type Assoc = ();
    fn new() {}
}

// Opaque return types that mention an associated type:
impl<'a, T: Trait<'a>> Wrapping<'a, T> {
    asyn fn mk_assoc() -> T::Assoc { /* ... */}
    fn a_few_assocs() -> impl Iterator<Item = T::Assoc> { /* ... */} 
}

fn main() {
    println!("Hello, world!");
}
