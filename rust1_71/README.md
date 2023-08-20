# Rust 1.71.0 | Let's Get Rusty

https://youtu.be/H8aHX-JkiZs

<hr>

# Rust Releases! Rust 1.71.0

https://youtu.be/8DG1V9nNYeg

# Announcing Rust 1.71.0

https://blog.rust-lang.org/2023/07/13/Rust-1.71.0.html

# Named levels of debug information

The ```-Cdebuginfo``` compiler option has previously only supported numbers 0..=2 for increasing amounts of debugging information, where Cargo defaults to 2 in dev and test profiles and 0 in release and bench profiles. These debug levels can now be set by name: "none" (0), "limited" (1), and "full" (2), as well as two new levels, "line-directives-only" and "line-tables-only".

https://doc.rust-lang.org/cargo/reference/profiles.html

# Debuginfo levels in Cargo

<table border="1">
    <tr>
    <td colspan="3" align="center">Rust tutorial</td>
    </tr>
    <tr align="center">
        <td>Number</td>
        <td>Profile</td>
        <td>Name</td>
    </tr>
    <tr align="center">
        <td>0</td>
        <td>release</td>
        <td>none</td>
    </tr>
    <tr align="center">
        <td></td>
        <td></td>
        <td>line-tables-only</td>
    </tr>
    <tr align="center">
        <td>1</td>
        <td></td>
        <td>limited</td>
    </tr>
    <tr align="center">
        <td>2</td>
        <td>dev</td>
        <td>full</td>
    </tr>
    <tr align="center">
        <td></td>
        <td></td>
        <td>line-directives-only</td>
    </tr>
</table>

# 배열을 튜플로 튜플을 배열로 ㅎ 

```rust
- From<T; N> for (T...) // (array to N-tuple for N in 1..=12)
- From(T...) for [T; N] // (N-tuple to array for N in 1..=12)

```

https://doc.rust-lang.org/stable/std/primitive.array.html#impl-From%3C%28T,%29%3E-for-%5BT;+1%5D
