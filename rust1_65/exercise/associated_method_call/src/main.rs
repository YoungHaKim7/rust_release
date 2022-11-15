// https://doc.rust-lang.org/reference/items/associated-items.html
// Examples of methods implemented on struct `Example`.
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

struct Example;
type Alias = Example;
trait Trait {
    type Output;
}
impl Trait for Example {
    type Output = Example;
}

impl Example {
    fn by_value(self: Self) {}
    fn by_ref(self: &Self) {}
    fn by_ref_mut(self: &mut Self) {}
    fn by_box(self: Box<Self>) {}
    fn by_rc(self: Rc<Self>) {}
    fn by_arc(self: Arc<Self>) {}
    fn by_pin(self: Pin<&Self>) {}
    fn explicit_type(self: Arc<Example>) {}
    fn with_lifetime<'a>(self: &'a Self) {}
    fn nested<'a>(self: &mut &'a Arc<Rc<Box<Alias>>>) {}
    fn via_projection(self: <Example as Trait>::Output) {}
}

fn main() {
    println!("Hello, world!");
}
