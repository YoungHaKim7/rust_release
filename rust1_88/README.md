# Announcing Rust 1.88.0
- June 26, 2025 · The Rust Release Team
- https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/


<hr />

Version 1.88.0 (2025-06-26)
==========================

<a id="1.88.0-Language"></a>

Language
--------
- [Stabilize `#![feature(let_chains)]` in the 2024 edition.](https://github.com/rust-lang/rust/pull/132833)
  This feature allows `&&`-chaining `let` statements inside `if` and `while`, allowing intermixture with boolean expressions. The patterns inside the `let` sub-expressions can be irrefutable or refutable.
    - Stabilize let_chains in Rust 1.64 #94927 
      - https://github.com/rust-lang/rust/pull/94927

- [Stabilize `#![feature(naked_functions)]`.](https://github.com/rust-lang/rust/pull/134213)
  Naked functions allow writing functions with no compiler-generated epilogue and prologue, allowing full control over the generated assembly for a particular function.
- [Stabilize `#![feature(cfg_boolean_literals)]`.](https://github.com/rust-lang/rust/pull/138632)
  This allows using boolean literals as `cfg` predicates, e.g. `#[cfg(true)]` and `#[cfg(false)]`.
- [Fully de-stabilize the `#[bench]` attribute](https://github.com/rust-lang/rust/pull/134273). Usage of `#[bench]` without `#![feature(custom_test_frameworks)]` already triggered a deny-by-default future-incompatibility lint since Rust 1.77, but will now become a hard error.
- [Add warn-by-default `dangerous_implicit_autorefs` lint against implicit autoref of raw pointer dereference.](https://github.com/rust-lang/rust/pull/123239)
  The lint [will be bumped to deny-by-default](https://github.com/rust-lang/rust/pull/141661) in the next version of Rust.
- [Add `invalid_null_arguments` lint to prevent invalid usage of null pointers.](https://github.com/rust-lang/rust/pull/119220)
  This lint is uplifted from `clippy::invalid_null_ptr_usage`.
- [Change trait impl candidate preference for builtin impls and trivial where-clauses.](https://github.com/rust-lang/rust/pull/138176)
- [Check types of generic const parameter defaults](https://github.com/rust-lang/rust/pull/139646)

<a id="1.88.0-Compiler"></a>

Compiler
--------
- [Stabilize `-Cdwarf-version` for selecting the version of DWARF debug information to generate.](https://github.com/rust-lang/rust/pull/136926)


<a id="1.88.0-Platform-Support"></a>

Platform Support
----------------
- [Demote `i686-pc-windows-gnu` to Tier 2.](https://blog.rust-lang.org/2025/05/26/demoting-i686-pc-windows-gnu/)


Refer to Rust's [platform support page][platform-support-doc]
for more information on Rust's tiered platform support.

[platform-support-doc]: https://doc.rust-lang.org/rustc/platform-support.html

<a id="1.88.0-Libraries"></a>

Libraries
---------
- [Remove backticks from `#[should_panic]` test failure message.](https://github.com/rust-lang/rust/pull/136160)
- [Guarantee that `[T; N]::from_fn` is generated in order of increasing indices.](https://github.com/rust-lang/rust/pull/139099), for those passing it a stateful closure.
- [The libtest flag `--nocapture` is deprecated in favor of the more consistent `--no-capture` flag.](https://github.com/rust-lang/rust/pull/139224)
- [Guarantee that `{float}::NAN` is a quiet NaN.](https://github.com/rust-lang/rust/pull/139483)


<a id="1.88.0-Stabilized-APIs"></a>

Stabilized APIs
---------------

- [`Cell::update`](https://doc.rust-lang.org/stable/std/cell/struct.Cell.html#method.update)
- [`impl Default for *const T`](https://doc.rust-lang.org/nightly/std/primitive.pointer.html#impl-Default-for-*const+T)
- [`impl Default for *mut T`](https://doc.rust-lang.org/nightly/std/primitive.pointer.html#impl-Default-for-*mut+T)
- [`HashMap::extract_if`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.extract_if)
- [`HashSet::extract_if`](https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html#method.extract_if)
- [`proc_macro::Span::line`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.line)
- [`proc_macro::Span::column`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.column)
- [`proc_macro::Span::start`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.start)
- [`proc_macro::Span::end`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.end)
- [`proc_macro::Span::file`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.file)
- [`proc_macro::Span::local_file`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.local_file)

These previously stable APIs are now stable in const contexts:

- [`NonNull<T>::replace`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.replace)
- [`<*mut T>::replace`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.replace)
- [`std::ptr::swap_nonoverlapping`](https://github.com/rust-lang/rust/pull/137280)
- [`Cell::{replace, get, get_mut, from_mut, as_slice_of_cells}`](https://github.com/rust-lang/rust/pull/137928)


<a id="1.88.0-Cargo"></a>

Cargo
-----
- [Stabilize automatic garbage collection.](https://github.com/rust-lang/cargo/pull/14287/)
- [use `zlib-rs` for gzip compression in rust code](https://github.com/rust-lang/cargo/pull/15417/)

<a id="1.88.0-Rustdoc"></a>

Rustdoc
-----
- [Doctests can be ignored based on target names using `ignore-*` attributes.](https://github.com/rust-lang/rust/pull/137096)
- [Stabilize the `--test-runtool` and `--test-runtool-arg` CLI options to specify a program (like qemu) and its arguments to run a doctest.](https://github.com/rust-lang/rust/pull/137096)

<a id="1.88.0-Compatibility-Notes"></a>

Compatibility Notes
-------------------
- [Finish changing the internal representation of pasted tokens](https://github.com/rust-lang/rust/pull/124141). Certain invalid declarative macros that were previously accepted in obscure circumstances are now correctly rejected by the compiler. Use of a `tt` fragment specifier can often fix these macros.
- [Fully de-stabilize the `#[bench]` attribute](https://github.com/rust-lang/rust/pull/134273). Usage of `#[bench]` without `#![feature(custom_test_frameworks)]` already triggered a deny-by-default future-incompatibility lint since Rust 1.77, but will now become a hard error.
- [Fix borrow checking some always-true patterns.](https://github.com/rust-lang/rust/pull/139042)
  The borrow checker was overly permissive in some cases, allowing programs that shouldn't have compiled.
- [Update the minimum external LLVM to 19.](https://github.com/rust-lang/rust/pull/139275)
- [Make it a hard error to use a vector type with a non-Rust ABI without enabling the required target feature.](https://github.com/rust-lang/rust/pull/139309)


<hr />

# cfg attribute with arbitrary constant expression
- https://stackoverflow.com/questions/71368383/cfg-attribute-with-arbitrary-constant-expression

- The answer is not. You have to use something like build script to achieve that.

It cannot work because cfg-expansion occurs at an earlier pass in the compiler than constant evaluation.

cfg expansion works at the same time as macro expansion. Both can affect name resolution (macros can create new names, which other macros, or even the same macro, can later refer to) which forces us to use a fixed-point algorithm (resolve names, expand macros, resolve names, expand macros... until no more names can be resolved, i.e. a fixed point was reached). const evaluation takes a place after type checking, sometimes (with generic_const_exprs) even during codegen. If it could affect macro expansion, we would have a giant fixed-point loop resolve names - expand macros - resolve names - expand macros... until a fixed point is reached, then lower to HIR - type-check - evaluate constants (or even lower to MIR - monomorphize and evaluate constants) - and back to name resolution. Besides slowing the compiler down a lot, it'll also make it significantly more complex, not really something the rustc team wants.

- 답은 아니다. 이를 달성하기 위해서는 빌드 스크립트와 같은 것을 사용해야 합니다.

Cfg-expansion은 상수 평가보다 컴파일러의 초기 패스에서 발생하기 때문에 작동할 수 없습니다.

Cfg 확장은 매크로 확장과 동시에 작동합니다. 둘 다 이름 해결에 영향을 미칠 수 있습니다(매크로는 새로운 이름을 만들 수 있으며, 다른 매크로 또는 심지어 동일한 매크로가 나중에 참조할 수 있음) 고정점 알고리즘(이름 해결, 매크로 확장, 이름 해결, 매크로 확장... 더 이상 이름이 해결될 수 없을 때까지, 즉 고정점에 도달할 때까지)을 사용하도록 강요합니다. const 평가는 유형 검사 후 이루어지며, 때로는 코드 생성 중에도 (generic_const_exprs와 함께). 만약 그것이 매크로 확장에 영향을 미칠 수 있다면, 우리는 거대한 고정점 루프 해결 이름을 가질 것이다 - 매크로 확장 - 이름 해결 - 고정점에 도달할 때까지 - 매크로를 확장할 것이다... 고정점에 도달할 때까지, 그 다음 HIR - 유형 검사 - 상수 평가로 낮추고 (또는 MIR - 상수 모노모피화 및 평가)로 낮추고 - 그리고 이름 해결로 돌아갈 것이다. 컴파일러 속도를 많이 늦출 뿐만 아니라, rustc 팀이 원하는 것이 아니라 훨씬 더 복잡하게 만들 것입니다.
