use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

trait PointerFamily {
    type Pointer<T>: Deref<Target = T>;

    fn new<T>(value: T) -> Self::Pointer<T>;
}

struct ArcFamily;
struct RcFamily;

impl PointerFamily for ArcFamily {
    type Pointer<T> = Arc<T>;
}

impl PointerFamily for RcFamily {
    type Pointer<T> = Rc<T>;
}

struct Mystruct<P: PointerFamily> {
    pointer: P::Pointer<String>,
}

fn main() {
    println!("Hello, world!");
}
