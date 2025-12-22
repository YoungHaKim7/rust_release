# Result

```bash

```

# Infallible

```rs
/// # Future compatibility
///
/// This enum has the same role as [the `!` “never” type][never],
/// which is unstable in this version of Rust.
/// When `!` is stabilized, we plan to make `Infallible` a type alias to it:
///
/// ```ignore (illustrates future std change)
/// pub type Infallible = !;
/// ```
///
/// … and eventually deprecate `Infallible`.
///
/// However there is one case where `!` syntax can be used
/// before `!` is stabilized as a full-fledged type: in the position of a function’s return type.
/// Specifically, it is possible to have implementations for two different function pointer types:
///
/// ```
/// trait MyTrait {}
/// impl MyTrait for fn() -> ! {}
/// impl MyTrait for fn() -> std::convert::Infallible {}
/// ```
///
/// With `Infallible` being an enum, this code is valid.
/// However when `Infallible` becomes an alias for the never type,
/// the two `impl`s will start to overlap
/// and therefore will be disallowed by the language’s trait coherence rules.
#[stable(feature = "convert_infallible", since = "1.34.0")]
#[derive(Copy)]
pub enum Infallible {}

#[stable(feature = "convert_infallible", since = "1.34.0")]
#[rustc_const_unstable(feature = "const_clone", issue = "142757")]
impl const Clone for Infallible {
    fn clone(&self) -> Infallible {
        match *self {}
    }
}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl fmt::Debug for Infallible {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

#[stable(feature = "convert_infallible", since = "1.34.0")]
impl fmt::Display for Infallible {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

#[stable(feature = "str_parse_error2", since = "1.8.0")]
impl Error for Infallible {}

#[stable(feature = "convert_infallible", since = "1.34.0")]
#[rustc_const_unstable(feature = "const_cmp", issue = "143800")]
impl const PartialEq for Infallible {
    fn eq(&self, _: &Infallible) -> bool {
        match *self {}
    }
}


```

