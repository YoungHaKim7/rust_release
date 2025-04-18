trait Supertrait {}

// Upcasting from &dyn Trait to &dyn Supertrait
fn upcast(x: &dyn Supertrait) -> &dyn Supertrait {
    x
}

#[derive(Debug)]
struct MyType;

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
    // println!("Super Trait obj = {:?}", supertrait_obj2);
}
