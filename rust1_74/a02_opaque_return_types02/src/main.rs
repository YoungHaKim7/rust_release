struct Wrapper<'a, T>(&'a T);

impl Wrapper<'_, ()> {
    async fn async_fn() -> Self {
        /* ... */
        fn impl_trait() -> impl Iterator<Item = Self> { /* ... */
        }
    }
}
trait Trait<'a> {
    type Assoc;
    fn new() -> Self::Assoc;
}

impl Trait<'_> for () {
    type Assoc = ();
    fn new() {}
}

impl<'a, T: Trait<'a>> Wrapper<'a, T> {
    async fn mk_assoc() -> T::Assoc {
        // ..
    }

    fn a_few_assocs() -> impl Iterator<Item = T::Assoc> {
        // ..
    }
}

fn main() {
    println!("Hello, world!");
}
