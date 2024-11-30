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

trait Tr<'a> {
    type Ty;
}
impl Tr<'_> for () {
    type Ty = ();
}

fn f_implicit() -> impl for<'a> Tr<'a, Ty = impl Copy> {}
// In Rust 2021 and earlier, the above is equivalent to:
fn f_2021() -> impl for<'a> Tr<'a, Ty = impl Copy + use<>> {}
// In Rust 2024 and later, it's equivalent to:
//fn f_2024() -> impl for<'a> Tr<'a, Ty = impl Copy + use<'a>> {}
//                                        ~~~~~~~~~~~~~~~~~~~~
// However, note that the capturing of higher-ranked lifetimes in
// nested opaque types is not yet supported.

// APIT
fn f_implicit_apit(_: impl Sized) -> impl Sized {}
//               ~~~~~~~~~~
//           This is called APIT.
//
// The above is *roughly* equivalent to:
fn f_explicit_apit<_0: Sized>(_: _0) -> impl Sized + use<_0> {}

fn main() {
    println!("Rust 2024 Hello, world!");
}
