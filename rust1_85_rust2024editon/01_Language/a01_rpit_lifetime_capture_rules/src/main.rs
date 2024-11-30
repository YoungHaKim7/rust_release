struct S<T, const C: usize>((T, [(); C]));
impl<T, const C: usize> S<T, C> {
//   ~~~~~~~~~~~~~~~~~
// These generic parameters are in scope.
    fn f_implicit<U>() -> impl Sized {}
    //            ~       ~~~~~~~~~~
    //            ^ This generic is in scope too.
    //                    ^
    //                    |
    //     No `use<..>` bound is present here.
    //
    // In all editions, it's equivalent to:
    fn f_explicit<U>() -> impl Sized + use<T, U, C> {}
}

fn main() {
    println!("Rust 2024 Hello, world!");
}
