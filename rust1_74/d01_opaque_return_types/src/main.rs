// RPIT = Return Position `impl Trait`

struct Wrapper<'a, T>(&'a T);

// Opaque return types that mention `Self`:
impl Wrapper<'_, ()> {
    async fn async_fn() -> Self { /* ... */
    }
    fn impl_trait() -> impl Iterator<Item = Self> { /* .. */
    }
}

fn main() {
    println!("Hello, world!");
}
