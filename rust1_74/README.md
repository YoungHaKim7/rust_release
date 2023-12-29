# Rust 1.74.0: All 45 changes in 19 minutes! | Nathan Stocks
- https://youtu.be/MOzuShpnUm8?si=djG0xOSm77XJHHGT

- 1. Add a [lints] table to Cargo.toml #3389  
  - https://github.com/rust-lang/rfcs/pull/3389
    - https://rust-lang.github.io/rfcs/
  - https://rust-lang.github.io/rfcs/3389-manifest-lint.html



- Intro
- 0. Correction from 1.73.0 -- fancy colors are now available!
- 1. Lint configuration through Cargo
- 2. Self and associated types allowed in Opaque Return types
- 3. Minimum Apple Versions Raised
- 4. private_in_public lint replaced with two new lints
- 5. (Docs) Lifetime parameters do not affect enum discrimanants
- 6. Allow explicit #[repr(Rust)]
- 7. Fix closure field captures depending on packed field alignment
- 8. Enable MIR-based drop-tracking for async blocks
- 9. Allow combining +bundle and +whole-archive link modifiers
- 10. Stabilize PATH for --print KIND=PATH
- 11. Enable ASAN, LSAN, & TSAN for *-apple-ios-macabi
- 12. New Platform Support
- 13. core::num::Saturating
- 14. Stdio stuff
- 15. Convert OS strings to/from encoded bytes
- 16. std::io::Error::other
- 17. impl TryFrom(char) for u16
- 18. Convert array reference to Vec
- 19. Convert array to reference-counted array
- 20. Const Stabilized APIs
- 21. MSRV for Cargo the library
- 22. Cargo changes how arrays in config are merged
- 23. cargo update --aggressive renamed to --recursive
- 24. -p optional in cargo update
- 25. Allow incomplete versions
- 26. Stabilize registry-auth with credential-process
- 27. Add -n alias for --dry-run
- 28. target.(cfg).linker
- 29. --keep-going
- 30. Rustdoc "warning" syntax
- 31. Rustdoc custom code CSS
- 32. Rustdoc search by generic type signature
- 33. Enum alias doc improvements
- 34. Cell::swap panics if cells overlap
- 35. rustc --extern requires valid ascii
- 36. redundant_as_str
- 37. needless_borrow_for_generic_args
- 38. path_ends_with_ext
- 39. unnecessary_map_on_constructor
- 40. missing_asserts_for_indexing
- 41. iter_out_of_bounds
- 42. implied_bounds_in_impls
- 43. reserve_after_initialization
- 44. should_panic_without_expect
- Buy my courses to be awesome at Rust



0:00 Intro
0:17 0. Correction from 1.73.0 -- fancy colors are now available!
0:32 1. Lint configuration through Cargo
2:18 2. Self and associated types allowed in Opaque Return types
2:54 3. Minimum Apple Versions Raised
3:24 4. private_in_public lint replaced with two new lints
3:59 5. (Docs) Lifetime parameters do not affect enum discrimanants
4:50 6. Allow explicit #[repr(Rust)]
5:35 7. Fix closure field captures depending on packed field alignment
6:00 8. Enable MIR-based drop-tracking for async blocks
6:34 9. Allow combining +bundle and +whole-archive link modifiers
7:04 10. Stabilize PATH for --print KIND=PATH
7:27 11. Enable ASAN, LSAN, & TSAN for *-apple-ios-macabi
7:50 12. New Platform Support
8:18 13. core::num::Saturating
8:46 14. Stdio stuff
9:15 15. Convert OS strings to/from encoded bytes
9:36 16. std::io::Error::other
9:51 17. impl TryFrom(char) for u16
10:04 18. Convert array reference to Vec
10:29 19. Convert array to reference-counted array
10:49 20. Const Stabilized APIs
11:29 21. MSRV for Cargo the library
11:46 22. Cargo changes how arrays in config are merged
12:15 23. cargo update --aggressive renamed to --recursive
12:34 24. -p optional in cargo update
12:51 25. Allow incomplete versions
13:15 26. Stabilize registry-auth with credential-process
13:39 27. Add -n alias for --dry-run
13:51 28. target.(cfg).linker
14:07 29. --keep-going
14:28 30. Rustdoc "warning" syntax
14:52 31. Rustdoc custom code CSS
15:12 32. Rustdoc search by generic type signature
15:31 33. Enum alias doc improvements
15:45 34. Cell::swap panics if cells overlap
15:58 35. rustc --extern requires valid ascii
16:08 36. redundant_as_str
16:24 37. needless_borrow_for_generic_args
16:36 38. path_ends_with_ext
17:03 39. unnecessary_map_on_constructor
17:20 40. missing_asserts_for_indexing
18:08 41. iter_out_of_bounds
18:26 42. implied_bounds_in_impls
18:55 43. reserve_after_initialization
19:08 44. should_panic_without_expect
19:25 Buy my courses to be awesome at Rust


<hr>

# Rust 1.74.0 | Let's Get Rusty
https://youtu.be/E0rPugR8s30?si=PqvJ4Lf-F0lcs-nw

- Stablize ```impl_trait_projections``` #115659
  - https://github.com/rust-lang/rust/pull/115659
- https://rustc-dev-guide.rust-lang.org/return-position-impl-trait-in-trait.html#aside-opaque-lifetime-duplication

<hr>

# Rust 컴파일 속도 올리기 How to use it(Nightly 231109 기준)

https://doc.rust-lang.org/beta/cargo/reference/registry-authentication.html

https://blog.rust-lang.org/2023/11/09/parallel-rustc.html

The nightly compiler is now shipping with the parallel front-end enabled. However, by default it runs in single-threaded mode and won't reduce compile times.

Keen users can opt into multi-threaded mode with the -Z threads option. For example:

```bash
$ RUSTFLAGS="-Z threads=8" cargo build --release
```

Alternatively, to opt in from a config.toml file (for one or more projects), add these lines:

- 경로 폴더 잘 만들어주고 toml만들고 안에 넣어주면됨.
- ```~/.cargo/config.toml```

```toml
[build]
rustflags = ["-Z", "threads=8"]

```

<hr>

# Rust 1.74.0 공식 문서

- https://github.com/rust-lang/rust/blob/1.74.0/RELEASES.md

- https://blog.rust-lang.org/2023/11/16/Rust-1.74.0.html

# Rust’s most wanted feature is coming!

https://youtu.be/EA19neOCeCo


# Stabilizing async fn in traits in 2023

https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html
