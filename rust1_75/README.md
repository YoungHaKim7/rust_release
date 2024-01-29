# Rust-1.75.0

- https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html

  - https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html
  - https://blog.rust-lang.org/2023/12/11/cargo-cache-cleaning.html
  - https://rust-lang.github.io/rfcs/3425-return-position-impl-trait-in-traits.html



# Rust Clippy

https://rust-lang.github.io/rust-clippy/rust-1.75.0/index.html

# Rust 1.75.0: 54 highlights in 20 minutes | Nathan Stocks

https://youtu.be/Z8xig7wEV68?si=BUMlC4v5tvMQxaj0

- 1. "async fn" and "return-position impl Trait" in Traits

- 2. Pointer byte offset APIs
  - byte_add  https://doc.rust-lang.org/std/primitive.pointer.html

```rs
byte_add  
byte_offset
byte_offset_from
byte_sub
wrapping_byte_add
wrapping_byte_offset
wrapping_byte_sub
```

- 3. Code layout optimizations for rusctc
  - Rustc에 대한 코드 레이아웃 최적화

- 4. Allow function pointer signatures to contain ```&mut T``` in const contexts
  - 함수 포인터 서명에 const 컨텍스트에서 &mutT 포함 허용

- 5. Match usize/isize exhautively with half-open ranges.
  - 반열린 범위로 "usize/isize"을 전체적으로 일치시킵니다.
    - exhaustively(부사 1.철저하게 2.남김 없이 3.속속들이)Adverb adv.부사

- 6. Guarantee char has same size and alignment as u32.
  - 보증 char는 u32와 같은 크기와 정렬을 가지고 있습니다.

- 7. Null pointer has the 0 address.
  - Null 포인터의 주소는 0입니다.

- 8. Allow parially-moved values in match
  - 일치하는 부분에서 부분적으로 이동된 값 허용

- 9. Document non-compliant floating-point behavior on 32-bit x86 targets
  - 32비트 x86 대상의 비준수 부동 소수점 동작 문서화

- 10. Stabilize ratified RISC-V target features.

- 11. Negative coherence properly consider impls that only partly overlap
  - 음의 일관성은 부분적으로만 겹치는 임펄스를 적절하게 고려합니다

- 12. Deny-by-default coinductive_overlap_in_coherence

- 13. Consider alias bounds when computing liveness in NLL

- 14. Add V (vector) extension to the riscv64-linux-android target spec.

- 15. Automatically enable cross-crate inlining for small functions

- 16. Three New Tier 3 Targets

- 17. Waker::clone_from avoids unnecessary clones

- 18. Implement BufRead for VecDeque{u8}

- 19. Implement FusedIterator for DecodeUtf16 when the inner iterator does

- 20. impl Not, Bit{And,Or}{,Assign} for IP addresses

- 21. Implement Default for ExitCode

- 22. Guarantee representation of None in NPO

- 23. Document when atomic loads are guaranteed read-only

- 24. Better document effects of recursive thread local storage initialization

- 25. Support sub-millisecond sleep on Windows 10+

- 26. Fix reverse searching after calling str::split_inclusive

- 27. Fix exit status on non-Unix cfg(unix) platforms

- Stabilized APIs

- 28. Atomic*::from_ptr

- 29. File time struct, trait, and methods.
```rs
FileTimes
FileTimesExt
File::set_modified
File::set_times
  
```

- 30. IpAddr::to_canonical & Ipv6Addr::to_canonical

- 31. Option::as_slice & Option::as_mut_slice
  - easy rust Rust1.75 as_slice영상
    - https://youtube.com/shorts/yyS8MH7sI1E?si=6xYcI2qvK6FwTTTc

<hr>

- Const Stabilized APIs

- 32. Ipv6Addr::to_ipv4_mapped

- 33. MaybeUninit::assume_init_read & MaybeUninit::zeroed

- 34. mem::discriminant

- 35. mem::zeroed

<hr>

- Cargo Changes

- 36. Add new packages to [workspace.members] automatically

- 37. Allow version-less manifests

- 38. Print the generated docs links

- 39. Make browser links out of HTML file paths

- 40. Print environment variables for build script executions with -vv

- 41. Rustdoc accepts less invalid Rust

- 42. Show enum discriminant if it is a C-like variant

<hr>

- Compatibility Notes

- 43. FreeBSD targets now require at least version 12

- 44. All MIPS targets dropped to Tier 3 support

- 45. invalid_alignment lint promoted to an error

- 46. Fix detecting references to packed unsized fields

- 47. Compiler plugin support removed

- New Clippy Lints

- 48. unused_enumerate_index

- 49. unnecessary_fallible_conversions
  - https://rust-lang.github.io/rust-clippy/master/index.html#/unnecessary

- 50. waker_clone_wake

- 51. struct_field_names

- 52. into_iter_without_iter
  - clippy::iter_without_into_iter

- 53. iter_without_into_iter

- 54. manual_hash_one

- Outro


0:00 Intro
0:11 1. async fn and return-position impl Trait in Traits<br>
2:17 2. Pointer byte offset APIs<br>
3:01 3. Code layout optimizations for rustc<br>
3:17 4. Allow function pointer signatures to contain &mut T in const contexts<br>
3:40 5. Match usize/isize exhaustively with half-open ranges<br>
4:57 6. Guarantee char has same size and alignment as u32<br>
5:11 7. Null pointer has the 0 address<br>
5:24 8. Allow partially-moved values in match<br>
6:07 9. Document non-compliant floating-point behavior on 32-bit x86 targets<br>
6:25 10. Stabilize ratified RISC-V target features<br>
6:42 11. Negative coherence properly consider impls that only partly overlap<br>
7:02 12. Deny-by-default coinductive_overlap_in_coherence<br>
7:22 13. Consider alias bounds when computing liveness in NLL<br>
7:36 14. Add V (vector) extension to the riscv64-linux-android target spec.<br>
8:00 15. Automatically enable cross-crate inlining for small functions<br>
8:10 16. Three New Tier 3 Targets<br>
8:56 17. Waker::clone_from avoids unnecessary clones<br>
9:14 18. Implement BufRead for VecDeque{u8}<br>
9:21 19. Implement FusedIterator for DecodeUtf16 when the inner iterator does<br>
9:40 20. impl Not, Bit{And,Or}{,Assign} for IP addresses<br>
10:08 21. Implement Default for ExitCode<br>
10:17 22. Guarantee representation of None in NPO<br>
10:38 23. Document when atomic loads are guaranteed read-only<br>
10:59 24. Better document effects of recursive thread local storage initialization<br>
11:14 25. Support sub-millisecond sleep on Windows 10+<br>
11:25 26. Fix reverse searching after calling str::split_inclusive<br>
11:41 27. Fix exit status on non-Unix cfg(unix) platforms<br>
11:58 Stabilized APIs<br>
12:02 28. Atomic*::from_ptr<br>
12:19 29. File time struct, trait, and methods.<br>
13:00 30. IpAddr::to_canonical & Ipv6Addr::to_canonical<br>
13:20 31. Option::as_slice & Option::as_mut_slice<br>
13:32 Const Stabilized APIs<br>
13:39 32. Ipv6Addr::to_ipv4_mapped<br>
13:54 33. MaybeUninit::assume_init_read & MaybeUninit::zeroed<br>
14:17 34. mem::discriminant<br>
14:27 35. mem::zeroed<br>
14:36 Cargo Changes<br>
14:40 36. Add new packages to [workspace.members] automatically<br>
14:52 37. Allow version-less manifests<br>
15:13 38. Print the generated docs links<br>
15:22 39. Make browser links out of HTML file paths<br>
15:37 40. Print environment variables for build script executions with -vv<br>
15:48 41. Rustdoc accepts less invalid Rust<br>
16:03 42. Show enum discriminant if it is a C-like variant<br>
16:36 Compatibility Notes<br>
16:40 43. FreeBSD targets now require at least version 12<br>
16:50 44. All MIPS targets dropped to Tier 3 support<br>
17:13 45. invalid_alignment lint promoted to an error<br>
17:21 46. Fix detecting references to packed unsized fields<br>
17:38 47. Compiler plugin support removed<br>
17:47 New Clippy Lints<br>
17:55 48. unused_enumerate_index<br>
18:09 49. unnecessary_fallible_conversions<br>
18:30 50. waker_clone_wake<br>
18:42 51. struct_field_names<br>
19:03 52. into_iter_without_iter<br>
19:20 53. iter_without_into_iter<br>
19:37 54. manual_hash_one<br>
19:55 Outro<br>

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

