trait OtherTrait {
    type Assoc<'a>;
}
impl OtherTrait for u32 {
    type Assoc<'a> = &'a u32;
}

trait Trait {}
impl<T: OtherTrait> Trait for (T, for<'a> fn(<T as OtherTrait>::Assoc<'a>)) {}

fn impls<T: Trait>() {}

fn main() {
    impls::<(u32, for<'a> fn(&'a u32))>();
}
