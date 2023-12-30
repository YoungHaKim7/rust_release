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
  - https://rust-lang.github.io/rfcs/2145-type-privacy.html
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

```rs
impl From<io::Stdout> for std::process::Stdio
impl From<io::Stderr> for std::process::Stdio

impl From<OwnedFd> for std::process::ChildStdin
impl From<OwnedFd> for std::process::ChildStdout
impl From<OwnedFd> for std::process::ChildStderr

impl From<OwnedHandle> for std::process::ChildStdin
impl From<OwnedHandle> for std::process::ChildStdout
impl From<OwnedHandle> for std::process::ChildStderr
    
```

- 15. Convert OS strings to/from encoded bytes
```rs
std::ffi::OsString::from_encoded_bytes_unchecked
std::ffi::OsString::into_encoded_bytes
std::ffi::OsStr::from_encoded_bytes_unchecked
std::ffi::OsStr::as_encoded_bytes
  
```
- 16. std::io::Error::other
```rs
std::io::Error::other(err)

std::io::Error::new(io::ErrorKind::Other, err)
  
```
- 17. impl TryFrom(char) for u16
- 18. Convert array reference to Vec
```rs
impl<T: Clone, const N: usize> From<&[T; N]> for Vec<T>
impl<T: Clone, const N: usize> From<&mut [T; N]> for Vec<T>
```

- 19. Convert array to reference-counted array

```rs
impl<T, const N: usize> From<[T; N]> for Arc<T>
impl<T, const N: usize> From<[T; N]> for Rc<T>
```
- 20. Const Stabilized APIs
```rs
core::mem::transmut_copy
std::is_ascii
[u8]::is_ascii
```

- 21. MSRV for Cargo the library
- 22. Cargo changes how arrays in config are merged
  - Cargo changes how arrays in config are merged
    - https://blog.rust-lang.org/inside-rust/2023/08/24/cargo-config-merging.html

- 23. cargo update --aggressive renamed to --recursive
- 24. -p optional in cargo update
```bash
cargo update -p mypackage
```
- 25. Allow incomplete versions
```bash
cargo update mypackage@5
```
- 26. Stabilize registry-auth with credential-process
- 27. Add -n alias for --dry-run
- 28. target.(cfg).linker
```rs
[target.'cfg(not(target_arch= "avr"))']
linker = "some-linker"
```
- 29. --keep-going
  - 컴파일 할때 에러나도 무시하고 계속 컴파일하라는 거 같음
```bash
cargo run --keep-going
  
```
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



0:00 Intro<br>
0:17 0. Correction from 1.73.0 -- fancy colors are now available!<br>
0:32 1. Lint configuration through Cargo<br>
2:18 2. Self and associated types allowed in Opaque Return types<br>
2:54 3. Minimum Apple Versions Raised<br>
3:24 4. private_in_public lint replaced with two new lints<br>
3:59 5. (Docs) Lifetime parameters do not affect enum discrimanants<br>
4:50 6. Allow explicit #[repr(Rust)]<br>
5:35 7. Fix closure field captures depending on packed field alignment<br>
6:00 8. Enable MIR-based drop-tracking for async blocks<br>
6:34 9. Allow combining +bundle and +whole-archive link modifiers<br>
7:04 10. Stabilize PATH for --print KIND=PATH<br>
7:27 11. Enable ASAN, LSAN, & TSAN for *-apple-ios-macabi<br>
7:50 12. New Platform Support<br>
8:18 13. core::num::Saturating<br>
8:46 14. Stdio stuff<br>
9:15 15. Convert OS strings to/from encoded bytes<br>
9:36 16. std::io::Error::other<br>
9:51 17. impl TryFrom(char) for u16<br>
10:04 18. Convert array reference to Vec<br>
10:29 19. Convert array to reference-counted array<br>
10:49 20. Const Stabilized APIs<br>
11:29 21. MSRV for Cargo the library<br>
11:46 22. Cargo changes how arrays in config are merged<br>
12:15 23. cargo update --aggressive renamed to --recursive<br>
12:34 24. -p optional in cargo update<br>
12:51 25. Allow incomplete versions<br>
13:15 26. Stabilize registry-auth with credential-process<br>
13:39 27. Add -n alias for --dry-run<br>
13:51 28. target.(cfg).linker<br>
14:07 29. --keep-going<br>
14:28 30. Rustdoc "warning" syntax<br>
14:52 31. Rustdoc custom code CSS<br>
15:12 32. Rustdoc search by generic type signature<br>
15:31 33. Enum alias doc improvements<br>
15:45 34. Cell::swap panics if cells overlap<br>
15:58 35. rustc --extern requires valid ascii<br>
16:08 36. redundant_as_str<br>
16:24 37. needless_borrow_for_generic_args<br>
16:36 38. path_ends_with_ext<br>
17:03 39. unnecessary_map_on_constructor<br>
17:20 40. missing_asserts_for_indexing<br>
18:08 41. iter_out_of_bounds<br>
18:26 42. implied_bounds_in_impls<br>
18:55 43. reserve_after_initialization<br>
19:08 44. should_panic_without_expect<br>
19:25 Buy my courses to be awesome at Rust<br>


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
