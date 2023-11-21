struct Wrapper<'a, T>(&'a T);

trait Trait<'a> {
    type Assoc;
    fn new() -> Self::Assoc;
}
impl Trait<'_> for () {
    type Assoc = ();
    fn new() {}
}

// Opaque return types that mention an associated type:
// 관련된 유형을 언급하는 불투명 반환 유형:
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
