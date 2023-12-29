// https://rust-lang.github.io/rust-clippy/master/index.html#/unnecessary

fn main() {
    // let _: Result<i64, _> = 1i32.try_into();
    // let _: Result<i64, _> = <_>::try_from(1i32);

    let _: i64 = 1i32.into();
    let _: i64 = <_>::from(1i32);
}
