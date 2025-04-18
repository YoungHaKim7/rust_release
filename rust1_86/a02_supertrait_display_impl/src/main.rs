use std::fmt::{self, Display};

// Supertrait now includes Display
trait Supertrait: Display {}

// Upcasting from &dyn Supertrait to &dyn Supertrait (trivial here)
fn upcast(x: &dyn Supertrait) -> &dyn Supertrait {
    x
}

#[derive(Debug)]
struct MyType;

// Implement Display for MyType
impl Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I'm a MyType instance!")
    }
}

// Implement Supertrait for MyType
impl Supertrait for MyType {}

fn main() {
    let my_value = MyType;

    // First create a trait object of the more specific type
    let trait_obj: &dyn Supertrait = &my_value;

    // Then upcast it to the supertrait
    let supertrait_obj: &dyn Supertrait = upcast(trait_obj);

    // Or more directly:
    let supertrait_obj2: &dyn Supertrait = trait_obj;

    println!("Upcasting works!");
    println!("Super Trait obj = {}", supertrait_obj); // uses Display!
    println!("Super Trait obj2 = {}", supertrait_obj2); // uses Display!
}
