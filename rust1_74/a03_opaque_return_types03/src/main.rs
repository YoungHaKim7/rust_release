#![feature(return_position_impl_trait_in_trait, async_fn_in_trait)]
// (just needed for final tests below)

// ---------------------------------------- //

struct Wrapper<'a, T>(&'a T);

impl Wrapper<'_, ()> {
    async fn async_fn() -> Self {
        //^ Previously rejected because it returns `-> Self`, not `-> Wrapper<'_, ()>`.
        Wrapper(&())
    }

    fn impl_trait() -> impl Iterator<Item = Self> {
        //^ Previously rejected because it mentions `Self`, not `Wrapper<'_, ()>`.
        std::iter::once(Wrapper(&()))
    }
}

// ---------------------------------------- //

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
        //^ Previously rejected because `T::Assoc` doesn't mention `'a` in the HIR,
        //  but ends up resolving to `<T as Trait<'a>>::Assoc`, which does rely on `'a`.
        // That's the important part -- the elided trait.
        T::new()
    }

    fn a_few_assocs() -> impl Iterator<Item = T::Assoc> {
        //^ Previously rejected for the same reason
        [T::new(), T::new(), T::new()].into_iter()
    }
}

// ---------------------------------------- //

trait InTrait {
    async fn async_fn() -> Self;

    fn impl_trait() -> impl Iterator<Item = Self>;
}

impl InTrait for &() {
    async fn async_fn() -> Self {
        &()
    }
    //^ Previously rejected just like inherent impls

    fn impl_trait() -> impl Iterator<Item = Self> {
        //^ Previously rejected just like inherent impls
        [&()].into_iter()
    }
}

fn main() {
    println!("Hello, world!");
}
