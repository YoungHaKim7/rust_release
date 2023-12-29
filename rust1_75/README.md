# Rust-1.75.0

https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html

# Rust 1.75.0: 54 highlights in 20 minutes | Nathan Stocks

https://youtu.be/Z8xig7wEV68?si=BUMlC4v5tvMQxaj0

- 1. "async fn" and "return-position impl Trait" in Traits

- 2. Pointer byte offset APIs

- 3. Code layout optimizations for ructc

- 4. Allow function pointer signatures to contain ```&mut T``` in const contexts

- 5. Match usize/isize exhautively with half-open ranges.

- 6. Guarantee char has same size and alignment as u32.

- 7. Null pointer has the 0 address.

- 8. Allow parially-moved values in match

- 9. Document non-compliant floating-point behavior on 32-bit x86 targets

- 10. Stabilize ratified RISC-V target features.



0:00 Intro
0:11 1. async fn and return-position impl Trait in Traits
2:17 2. Pointer byte offset APIs
3:01 3. Code layout optimizations for rustc
3:17 4. Allow function pointer signatures to contain &mut T in const contexts
3:40 5. Match usize/isize exhaustively with half-open ranges
4:57 6. Guarantee char has same size and alignment as u32
5:11 7. Null pointer has the 0 address
5:24 8. Allow partially-moved values in match
6:07 9. Document non-compliant floating-point behavior on 32-bit x86 targets
6:25 10. Stabilize ratified RISC-V target features
6:42 11. Negative coherence properly consider impls that only partly overlap
7:02 12. Deny-by-default coinductive_overlap_in_coherence
7:22 13. Consider alias bounds when computing liveness in NLL
7:36 14. Add V (vector) extension to the riscv64-linux-android target spec.
8:00 15. Automatically enable cross-crate inlining for small functions
8:10 16. Three New Tier 3 Targets
8:56 17. Waker::clone_from avoids unnecessary clones
9:14 18. Implement BufRead for VecDeque{u8}
9:21 19. Implement FusedIterator for DecodeUtf16 when the inner iterator does
9:40 20. impl Not, Bit{And,Or}{,Assign} for IP addresses
10:08 21. Implement Default for ExitCode
10:17 22. Guarantee representation of None in NPO
10:38 23. Document when atomic loads are guaranteed read-only
10:59 24. Better document effects of recursive thread local storage initialization
11:14 25. Support sub-millisecond sleep on Windows 10+
11:25 26. Fix reverse searching after calling str::split_inclusive
11:41 27. Fix exit status on non-Unix cfg(unix) platforms
11:58 Stabilized APIs
12:02 28. Atomic*::from_ptr
12:19 29. File time struct, trait, and methods.
13:00 30. IpAddr::to_canonical & Ipv6Addr::to_canonical
13:20 31. Option::as_slice & Option::as_mut_slice
13:32 Const Stabilized APIs
13:39 32. Ipv6Addr::to_ipv4_mapped
13:54 33. MaybeUninit::assume_init_read & MaybeUninit::zeroed
14:17 34. mem::discriminant
14:27 35. mem::zeroed
14:36 Cargo Changes
14:40 36. Add new packages to [workspace.members] automatically
14:52 37. Allow version-less manifests
15:13 38. Print the generated docs links
15:22 39. Make browser links out of HTML file paths
15:37 40. Print environment variables for build script executions with -vv
15:48 41. Rustdoc accepts less invalid Rust
16:03 42. Show enum discriminant if it is a C-like variant
16:36 Compatibility Notes
16:40 43. FreeBSD targets now require at least version 12
16:50 44. All MIPS targets dropped to Tier 3 support
17:13 45. invalid_alignment lint promoted to an error
17:21 46. Fix detecting references to packed unsized fields
17:38 47. Compiler plugin support removed
17:47 New Clippy Lints
17:55 48. unused_enumerate_index
18:09 49. unnecessary_fallible_conversions
18:30 50. waker_clone_wake
18:42 51. struct_field_names
19:03 52. into_iter_without_iter
19:20 53. iter_without_into_iter
19:37 54. manual_hash_one
19:55 Outro

<hr>

# Words od Advice

- Users cannot add additional bounds to the return type.
  - 사용자는 반환 유형에 추가 경계를 추가할 수 없습니다.

- Don't use ```-> impl Trait``` or ```async fn``` in public traits.
  - 공공의 특성에서 ```-> impl Trait```이나 ```asyncfn```을 사용하지 마십시오.

- Dynamic dispatch is not yet supported(use #[async_trait])
  - 동적 전송이 아직 지원되지 않습니다(#[async_trait] 사용)

- Use #[trait_variant::make] for multithreaded runtimes
  - 다중 스레드 실행 시간에 #[trait_variant::make] 사용


<hr>

# opaque 에 대해 

- https://users.rust-lang.org/t/generate-c-api-for-struct-methods-with-opaque-pointers/83163

- https://github.com/rust-lang/rust/issues/74298

- https://stackoverflow.com/questions/75510856/expected-type-parameter-found-opaque-type

# tokio

- https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs

- https://github.com/tokio-rs/tokio/blob/master/examples/hello_world.rs

