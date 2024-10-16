# Result

- `cargo clippy`

```bash
$ cargo clippy

warning: `--x` could be misinterpreted as pre-decrement by C programmers, is usually a no-op
 --> src/main.rs:6:13
  |
6 |     let y = --x;
  |             ^^^
  |
  = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#double_neg
note: the lint level is defined here
 --> src/main.rs:3:8
  |
3 | #[warn(clippy::double_neg)]
  |        ^^^^^^^^^^^^^^^^^^

warning: unused variable: `y`
 --> src/main.rs:6:9
  |
6 |     let y = --x;
  |         ^ help: if this is intentional, prefix it with an underscore: `_y`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `a01_style_clipppy` (bin "a01_style_clipppy") generated 2 warnings

```
