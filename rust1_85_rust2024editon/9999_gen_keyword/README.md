# gen-blocks
- https://doc.rust-lang.org/edition-guide/rust-2024/gen-keyword.html

- https://rust-lang.github.io/rfcs/3513-gen-blocks.html

# gen사용법
- gen초기 아이디어
  - https://crates.io/crates/genawaiter

- https://github.com/rust-lang/rust/issues/123731#issuecomment-2053953399

- Throwing some options out there:

  - stick with `gen`, break lots of crates
  - use `gener` (personal opinion: it's not clear what it means, tho we'll learn)
  - `impl Iterator {}`, kinda long but very extensible to builtin syntax for other traits (`impl Generator`, `impl Try`), conflicts with inherent impls
  - `iter/iterator`, breaks too much code
  - `something!`, make it a builtin macro, so local definitions and imports override it
  - `generator`, wordy, but explicit. We don't need to stick with the "keywords must be short" rule and it likely has no problematic breakage
  - use an attribute: `#[gen] {}`, a bit odd, but very extensible
  - ask T-lang to revisit `gen` becoming a contextual keyword
  - `yields {}`, may be too confusing with yield
  - prefix with some existing keyword (`move gen {}` could signal the difference to `static gen {}` which would not create iterator impls, but only some so-far-non-existent `PinnedIterator` trait). Not really necessary (we can just infer which traits to implement for `gen` blocks, though this is another nail in the coffin of `gen fn`). Also it's very confusing as `move gen` is different from `gen move` (move all variables instead of letting the generator be movable) and implies the existence of `move gen move`.
