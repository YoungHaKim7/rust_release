struct Wrapper<'a, T>(&'a T);

// Opaque return types that mention `Self`:
impl Wrapper<'_, ()> {
    // `async fn` return type cannot contain a projectction or `Self`
    // that references lifetimes from a parent scope
    //'async fn' 반환 유형에는 상위 범위에서
    // 수명을 참조하는 투영 또는 '자체'를 포함할 수 없습니다
    async fn async_fn() -> Self {
        //...
    }
    // `impl Trait` return type cannot contain a projection or
    // `Self` that references lifetimes from a parent scope
    // impl Trait' 반환 유형은 상위 범위에서 수명을 참조하는
    // 투영 또는 'Self'를 포함할 수 없습니다
    fn impl_trait() -> impl Iterator<Item = Self> {
        // ..
    }
}

fn main() {
    println!("Hello, world!");
}
