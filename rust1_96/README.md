# Announcing Rust 1.96.0
- May 28, 2026 · The Rust Release Team
  - https://blog.rust-lang.org/2026/05/28/Rust-1.96.0/

# 1.96 러스트 릴리즈
- https://open.substack.com/pub/weeklyrust/p/whats-new-in-rust-1960?r=2alrl0&utm_medium=ios

```rs
use core::range::Range;

#[derive(Clone, Copy)]
pub struct Span(Range<usize>);

impl Span {
    pub fn of(self, s: &str) -> &str {
        &s[self.0]
    }
}
```

- https://github.com/rust-lang/rfcs/pull/3550
```rs
let range1 = 0..5;
//  range1 : Range<usize>

let range2 = 0..#rit 5;
//  range2 : Range<usize>

let range3 = 0..#rii 5;
//  range3 : IntoRange<usize>

let range4 : Range<usize> = 0..5;

let range5 : IntoRange<usize> = 0..5;
```
