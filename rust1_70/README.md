# Rust 1.70 들어간 sparse-protocol

https://doc.rust-lang.org/stable/cargo/reference/registry-index.html#sparse-protocol

Version 1.70.0 (2023-06-01)

# lld 가 러스트 안으로 들어가서 빨라짐

https://stackoverflow.com/questions/76398409/does-rust-use-lld-linker-as-standard

- lld명령어 이해하기 https://shaeod.tistory.com/858

==========================

<a id="1.70.0-Language"></a>

Language
--------
- [Relax ordering rules for `asm!` operands](https://github.com/rust-lang/rust/pull/105798/)
- [Properly allow macro expanded `format_args` invocations to uses captures](https://github.com/rust-lang/rust/pull/106505/)
- [Lint ambiguous glob re-exports](https://github.com/rust-lang/rust/pull/107880/)
- [Perform const and unsafe checking for expressions in `let _ = expr` position.](https://github.com/rust-lang/rust/pull/102256/)

<a id="1.70.0-Compiler"></a>

Compiler
--------
- [Extend -Cdebuginfo with new options and named aliases](https://github.com/rust-lang/rust/pull/109808/)
  This provides a smaller version of debuginfo for cases that only need line number information
  (`-Cdebuginfo=line-tables-only`), which may eventually become the default for `-Cdebuginfo=1`.
- [Make `unused_allocation` lint against `Box::new` too](https://github.com/rust-lang/rust/pull/104363/)
- [Detect uninhabited types early in const eval](https://github.com/rust-lang/rust/pull/109435/)
- [Switch to LLD as default linker for {arm,thumb}v4t-none-eabi](https://github.com/rust-lang/rust/pull/109721/)
- [Add tier 3 target `loongarch64-unknown-linux-gnu`](https://github.com/rust-lang/rust/pull/96971)
- [Add tier 3 target for `i586-pc-nto-qnx700` (QNX Neutrino RTOS, version 7.0)](https://github.com/rust-lang/rust/pull/109173/), 
- [Insert alignment checks for pointer dereferences as debug assertions](https://github.com/rust-lang/rust/pull/98112)
  This catches undefined behavior at runtime, and may cause existing code to fail.

Refer to Rust's [platform support page][platform-support-doc]
for more information on Rust's tiered platform support.

<a id="1.70.0-Libraries"></a>

Libraries
---------
- [Document NonZeroXxx layout guarantees](https://github.com/rust-lang/rust/pull/94786/)
- [Windows: make `Command` prefer non-verbatim paths](https://github.com/rust-lang/rust/pull/96391/)
- [Implement Default for some alloc/core iterators](https://github.com/rust-lang/rust/pull/99929/)
- [Fix handling of trailing bare CR in str::lines](https://github.com/rust-lang/rust/pull/100311/)
- [allow negative numeric literals in `concat!`](https://github.com/rust-lang/rust/pull/106844/)
- [Add documentation about the memory layout of `Cell`](https://github.com/rust-lang/rust/pull/106921/)
- [Use `partial_cmp` to implement tuple `lt`/`le`/`ge`/`gt`](https://github.com/rust-lang/rust/pull/108157/)
- [Stabilize `atomic_as_ptr`](https://github.com/rust-lang/rust/pull/108419/)
- [Stabilize `nonnull_slice_from_raw_parts`](https://github.com/rust-lang/rust/pull/97506/)
- [Partial stabilization of `once_cell`](https://github.com/rust-lang/rust/pull/105587/)
- [Stabilize `nonzero_min_max`](https://github.com/rust-lang/rust/pull/106633/)
- [Flatten/inline format_args!() and (string and int) literal arguments into format_args!()](https://github.com/rust-lang/rust/pull/106824/)
- [Stabilize movbe target feature](https://github.com/rust-lang/rust/pull/107711/)
- [don't splice from files into pipes in io::copy](https://github.com/rust-lang/rust/pull/108283/)
- [Add a builtin unstable `FnPtr` trait that is implemented for all function pointers](https://github.com/rust-lang/rust/pull/108080/)
  This extends `Debug`, `Pointer`, `Hash`, `PartialEq`, `Eq`, `PartialOrd`, and `Ord`
  implementations for function pointers with all ABIs.

<a id="1.70.0-Stabilized-APIs"></a>

Stabilized APIs
---------------

- [`NonZero*::MIN/MAX`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroI8.html#associatedconstant.MIN)
- [`BinaryHeap::retain`](https://doc.rust-lang.org/stable/std/collections/struct.BinaryHeap.html#method.retain)
- [`Default for std::collections::binary_heap::IntoIter`](https://doc.rust-lang.org/stable/std/collections/binary_heap/struct.IntoIter.html)
- [`Default for std::collections::btree_map::{IntoIter, Iter, IterMut}`](https://doc.rust-lang.org/stable/std/collections/btree_map/struct.IntoIter.html)
- [`Default for std::collections::btree_map::{IntoKeys, Keys}`](https://doc.rust-lang.org/stable/std/collections/btree_map/struct.IntoKeys.html)
- [`Default for std::collections::btree_map::{IntoValues, Values}`](https://doc.rust-lang.org/stable/std/collections/btree_map/struct.IntoKeys.html)
- [`Default for std::collections::btree_map::Range`](https://doc.rust-lang.org/stable/std/collections/btree_map/struct.Range.html)
- [`Default for std::collections::btree_set::{IntoIter, Iter}`](https://doc.rust-lang.org/stable/std/collections/btree_set/struct.IntoIter.html)
- [`Default for std::collections::btree_set::Range`](https://doc.rust-lang.org/stable/std/collections/btree_set/struct.Range.html)
- [`Default for std::collections::linked_list::{IntoIter, Iter, IterMut}`](https://doc.rust-lang.org/stable/alloc/collections/linked_list/struct.IntoIter.html)
- [`Default for std::vec::IntoIter`](https://doc.rust-lang.org/stable/alloc/vec/struct.IntoIter.html#impl-Default-for-IntoIter%3CT,+A%3E)
- [`Default for std::iter::Chain`](https://doc.rust-lang.org/stable/std/iter/struct.Chain.html)
- [`Default for std::iter::Cloned`](https://doc.rust-lang.org/stable/std/iter/struct.Cloned.html)
- [`Default for std::iter::Copied`](https://doc.rust-lang.org/stable/std/iter/struct.Copied.html)
- [`Default for std::iter::Enumerate`](https://doc.rust-lang.org/stable/std/iter/struct.Enumerate.html)
- [`Default for std::iter::Flatten`](https://doc.rust-lang.org/stable/std/iter/struct.Flatten.html)
- [`Default for std::iter::Fuse`](https://doc.rust-lang.org/stable/std/iter/struct.Fuse.html)
- [`Default for std::iter::Rev`](https://doc.rust-lang.org/stable/std/iter/struct.Rev.html)
- [`Default for std::slice::Iter`](https://doc.rust-lang.org/stable/std/slice/struct.Iter.html)
- [`Default for std::slice::IterMut`](https://doc.rust-lang.org/stable/std/slice/struct.IterMut.html)
- [`Rc::into_inner`](https://doc.rust-lang.org/stable/alloc/rc/struct.Rc.html#method.into_inner)
- [`Arc::into_inner`](https://doc.rust-lang.org/stable/alloc/sync/struct.Arc.html#method.into_inner)
- [`std::cell::OnceCell`](https://doc.rust-lang.org/stable/std/cell/struct.OnceCell.html)
- [`Option::is_some_and`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.is_some_and)
- [`NonNull::slice_from_raw_parts`](https://doc.rust-lang.org/stable/std/ptr/struct.NonNull.html#method.slice_from_raw_parts)
- [`Result::is_ok_and`](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.is_ok_and)
- [`Result::is_err_and`](https://doc.rust-lang.org/stable/std/result/enum.Result.html#method.is_err_and)
- [`std::sync::atomic::Atomic*::as_ptr`](https://doc.rust-lang.org/stable/std/sync/atomic/struct.AtomicU8.html#method.as_ptr)
- [`std::io::IsTerminal`](https://doc.rust-lang.org/stable/std/io/trait.IsTerminal.html)
- [`std::os::linux::net::SocketAddrExt`](https://doc.rust-lang.org/stable/std/os/linux/net/trait.SocketAddrExt.html)
- [`std::os::unix::net::UnixDatagram::bind_addr`](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixDatagram.html#method.bind_addr)
- [`std::os::unix::net::UnixDatagram::connect_addr`](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixDatagram.html#method.connect_addr)
- [`std::os::unix::net::UnixDatagram::send_to_addr`](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixDatagram.html#method.send_to_addr)
- [`std::os::unix::net::UnixListener::bind_addr`](https://doc.rust-lang.org/stable/std/os/unix/net/struct.UnixListener.html#method.bind_addr)
- [`std::path::Path::as_mut_os_str`](https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.as_mut_os_str)
- [`std::sync::OnceLock`](https://doc.rust-lang.org/stable/std/sync/struct.OnceLock.html)

<a id="1.70.0-Cargo"></a>

Cargo
-----

- [Add `CARGO_PKG_README`](https://github.com/rust-lang/cargo/pull/11645/)
- [Make `sparse` the default protocol for crates.io](https://github.com/rust-lang/cargo/pull/11791/)
- [Accurately show status when downgrading dependencies](https://github.com/rust-lang/cargo/pull/11839/)
- [Use registry.default for login/logout](https://github.com/rust-lang/cargo/pull/11949/)
- [Stabilize `cargo logout`](https://github.com/rust-lang/cargo/pull/11950/)

<a id="1.70.0-Misc"></a>

Misc
----

- [Stabilize rustdoc `--test-run-directory`](https://github.com/rust-lang/rust/pull/103682/)

<a id="1.70.0-Compatibility-Notes"></a>

Compatibility Notes
-------------------

- [Prevent stable `libtest` from supporting `-Zunstable-options`](https://github.com/rust-lang/rust/pull/109044/)
- [Perform const and unsafe checking for expressions in `let _ = expr` position.](https://github.com/rust-lang/rust/pull/102256/)
- [WebAssembly targets enable `sign-ext` and `mutable-globals` features in codegen](https://github.com/rust-lang/rust/issues/109807)
  This may cause incompatibility with older execution environments.
- [Insert alignment checks for pointer dereferences as debug assertions](https://github.com/rust-lang/rust/pull/98112)
  This catches undefined behavior at runtime, and may cause existing code to fail.

<a id="1.70.0-Internal-Changes"></a>

Internal Changes
----------------

These changes do not affect any public interfaces of Rust, but they represent
significant improvements to the performance or internals of rustc and related
tools.

- [Upgrade to LLVM 16](https://github.com/rust-lang/rust/pull/109474/)
- [Use SipHash-1-3 instead of SipHash-2-4 for StableHasher](https://github.com/rust-lang/rust/pull/107925/)

Version 1.69.0 (2023-04-20)
==========================

<a id="1.69.0-Language"></a>

Language
--------

- [Deriving built-in traits on packed structs works with `Copy` fields.](https://github.com/rust-lang/rust/pull/104429/)
- [Stabilize the `cmpxchg16b` target feature on x86 and x86_64.](https://github.com/rust-lang/rust/pull/106774/)
- [Improve analysis of trait bounds for associated types.](https://github.com/rust-lang/rust/pull/103695/)
- [Allow associated types to be used as union fields.](https://github.com/rust-lang/rust/pull/106938/)
- [Allow `Self: Autotrait` bounds on dyn-safe trait methods.](https://github.com/rust-lang/rust/pull/107082/)
- [Treat `str` as containing `[u8]` for auto trait purposes.](https://github.com/rust-lang/rust/pull/107941/)

<a id="1.69.0-Compiler"></a>

Compiler
--------

- [Upgrade `*-pc-windows-gnu` on CI to mingw-w64 v10 and GCC 12.2.](https://github.com/rust-lang/rust/pull/100178/)
- [Rework min_choice algorithm of member constraints.](https://github.com/rust-lang/rust/pull/105300/)
- [Support `true` and `false` as boolean flags in compiler arguments.](https://github.com/rust-lang/rust/pull/107043/)
- [Default `repr(C)` enums to `c_int` size.](https://github.com/rust-lang/rust/pull/107592/)

<a id="1.69.0-Libraries"></a>

Libraries
---------

- [Implement the unstable `DispatchFromDyn` for cell types, allowing downstream experimentation with custom method receivers.](https://github.com/rust-lang/rust/pull/97373/)
- [Document that `fmt::Arguments::as_str()` may return `Some(_)` in more cases after optimization, subject to change.](https://github.com/rust-lang/rust/pull/106823/)
- [Implement `AsFd` and `AsRawFd` for `Rc`.](https://github.com/rust-lang/rust/pull/107317/)

<a id="1.69.0-Stabilized-APIs"></a>

Stabilized APIs
---------------

- [`CStr::from_bytes_until_nul`](https://doc.rust-lang.org/stable/core/ffi/struct.CStr.html#method.from_bytes_until_nul)
- [`core::ffi::FromBytesUntilNulError`](https://doc.rust-lang.org/stable/core/ffi/struct.FromBytesUntilNulError.html)

These APIs are now stable in const contexts:

- [`SocketAddr::new`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.new)
- [`SocketAddr::ip`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.ip)
- [`SocketAddr::port`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.port)
- [`SocketAddr::is_ipv4`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.is_ipv4)
- [`SocketAddr::is_ipv6`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.is_ipv6)
- [`SocketAddrV4::new`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV4.html#method.new)
- [`SocketAddrV4::ip`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV4.html#method.ip)
- [`SocketAddrV4::port`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV4.html#method.port)
- [`SocketAddrV6::new`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.new)
- [`SocketAddrV6::ip`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.ip)
- [`SocketAddrV6::port`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.port)
- [`SocketAddrV6::flowinfo`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.flowinfo)
- [`SocketAddrV6::scope_id`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.scope_id)

<a id="1.69.0-Cargo"></a>

Cargo
-----

- [Cargo now suggests `cargo fix` or `cargo clippy --fix` when compilation warnings are auto-fixable.](https://github.com/rust-lang/cargo/pull/11558/)
- [Cargo now suggests `cargo add` if you try to install a library crate.](https://github.com/rust-lang/cargo/pull/11410/)
- [Cargo now sets the `CARGO_BIN_NAME` environment variable also for binary examples.](https://github.com/rust-lang/cargo/pull/11705/)

<a id="1.69.0-Rustdoc"></a>

Rustdoc
-----

- [Vertically compact trait bound formatting.](https://github.com/rust-lang/rust/pull/102842/)
- [Only include stable lints in `rustdoc::all` group.](https://github.com/rust-lang/rust/pull/106316/)
- [Compute maximum Levenshtein distance based on the query.](https://github.com/rust-lang/rust/pull/107141/)
- [Remove inconsistently-present sidebar tooltips.](https://github.com/rust-lang/rust/pull/107490/)
- [Search by macro when query ends with `!`.](https://github.com/rust-lang/rust/pull/108143/)

<a id="1.69.0-Compatibility-Notes"></a>

Compatibility Notes
-------------------

- [The `rust-analysis` component from `rustup` now only contains a warning placeholder.](https://github.com/rust-lang/rust/pull/101841/) This was primarily intended for RLS, and the corresponding `-Zsave-analysis` flag has been removed from the compiler as well.
- [Unaligned references to packed fields are now a hard error.](https://github.com/rust-lang/rust/pull/102513/) This has been a warning since 1.53, and denied by default with a future-compatibility warning since 1.62.
- [Update the minimum external LLVM to 14.](https://github.com/rust-lang/rust/pull/107573/)
- [Cargo now emits errors on invalid characters in a registry token.](https://github.com/rust-lang/cargo/pull/11600/)
- [When `default-features` is set to false of a workspace dependency, and an inherited dependency of a member has `default-features = true`, Cargo will enable default features of that dependency.](https://github.com/rust-lang/cargo/pull/11409/)
- [Cargo denies `CARGO_HOME` in the `[env]` configuration table. Cargo itself doesn't pick up this value, but recursive calls to cargo would, which was not intended.](https://github.com/rust-lang/cargo/pull/11644/)
- [Debuginfo for build dependencies is now off if not explicitly set. This is expected to improve the overall build time.](https://github.com/rust-lang/cargo/pull/11252/)
- [The Rust distribution no longer always includes rustdoc](https://github.com/rust-lang/rust/pull/106886)
  If `tools = [...]` is set in config.toml, we will respect a missing rustdoc in that list. By
  default rustdoc remains included. To retain the prior behavior explicitly add `"rustdoc"` to the
  list.
  
<a id="1.69.0-Internal-Changes"></a>

Internal Changes
----------------

These changes do not affect any public interfaces of Rust, but they represent
significant improvements to the performance or internals of rustc and related
tools.

- [Move `format_args!()` into AST (and expand it during AST lowering)](https://github.com/rust-lang/rust/pull/106745/)

Version 1.68.2 (2023-03-28)
===========================

- [Update the GitHub RSA host key bundled within Cargo](https://github.com/rust-lang/cargo/pull/11883).
  The key was [rotated by GitHub](https://github.blog/2023-03-23-we-updated-our-rsa-ssh-host-key/)
  on 2023-03-24 after the old one leaked.
- [Mark the old GitHub RSA host key as revoked](https://github.com/rust-lang/cargo/pull/11889).
  This will prevent Cargo from accepting the leaked key even when trusted by
  the system.
- [Add support for `@revoked` and a better error message for `@cert-authority` in Cargo's SSH host key verification](https://github.com/rust-lang/cargo/pull/11635)

Version 1.68.1 (2023-03-23)
===========================

- [Fix miscompilation in produced Windows MSVC artifacts](https://github.com/rust-lang/rust/pull/109094)
  This was introduced by enabling ThinLTO for the distributed rustc which led
  to miscompilations in the resulting binary. Currently this is believed to be
  limited to the -Zdylib-lto flag used for rustc compilation, rather than a
  general bug in ThinLTO, so only rustc artifacts should be affected.
- [Fix --enable-local-rust builds](https://github.com/rust-lang/rust/pull/109111/)
- [Treat `$prefix-clang` as `clang` in linker detection code](https://github.com/rust-lang/rust/pull/109156)
- [Fix panic in compiler code](https://github.com/rust-lang/rust/pull/108162)

Version 1.68.0 (2023-03-09)
==========================

<a id="1.68.0-Language"></a>

Language
--------

- [Stabilize default_alloc_error_handler](https://github.com/rust-lang/rust/pull/102318/)
  This allows usage of `alloc` on stable without requiring the 
  definition of a handler for allocation failure. Defining custom handlers is still unstable.
- [Stabilize `efiapi` calling convention.](https://github.com/rust-lang/rust/pull/105795/)
- [Remove implicit promotion for types with drop glue](https://github.com/rust-lang/rust/pull/105085/)

<a id="1.68.0-Compiler"></a>

Compiler
--------

- [Change `bindings_with_variant_name` to deny-by-default](https://github.com/rust-lang/rust/pull/104154/)
- [Allow .. to be parsed as let initializer](https://github.com/rust-lang/rust/pull/105701/)
- [Add `armv7-sony-vita-newlibeabihf` as a tier 3 target](https://github.com/rust-lang/rust/pull/105712/)
- [Always check alignment during compile-time const evaluation](https://github.com/rust-lang/rust/pull/104616/)
- [Disable "split dwarf inlining" by default.](https://github.com/rust-lang/rust/pull/106709/)
- [Add vendor to Fuchsia's target triple](https://github.com/rust-lang/rust/pull/106429/)
- [Enable sanitizers for s390x-linux](https://github.com/rust-lang/rust/pull/107127/)

<a id="1.68.0-Libraries"></a>

Libraries
---------

- [Loosen the bound on the Debug implementation of Weak.](https://github.com/rust-lang/rust/pull/90291/)
- [Make `std::task::Context` !Send and !Sync](https://github.com/rust-lang/rust/pull/95985/)
- [PhantomData layout guarantees](https://github.com/rust-lang/rust/pull/104081/)
- [Don't derive Debug for `OnceWith` & `RepeatWith`](https://github.com/rust-lang/rust/pull/104163/)
- [Implement DerefMut for PathBuf](https://github.com/rust-lang/rust/pull/105018/)
- [Add O(1) `Vec -> VecDeque` conversion guarantee](https://github.com/rust-lang/rust/pull/105128/)
- [Leak amplification for peek_mut() to ensure BinaryHeap's invariant is always met](https://github.com/rust-lang/rust/pull/105851/)

<a id="1.68.0-Stabilized-APIs"></a>

Stabilized APIs
---------------

- [`{core,std}::pin::pin!`](https://doc.rust-lang.org/stable/std/pin/macro.pin.html)
- [`impl From<bool> for {f32,f64}`](https://doc.rust-lang.org/stable/std/primitive.f32.html#impl-From%3Cbool%3E-for-f32)
- [`std::path::MAIN_SEPARATOR_STR`](https://doc.rust-lang.org/stable/std/path/constant.MAIN_SEPARATOR_STR.html)
- [`impl DerefMut for PathBuf`](https://doc.rust-lang.org/stable/std/path/struct.PathBuf.html#impl-DerefMut-for-PathBuf)

These APIs are now stable in const contexts:

- [`VecDeque::new`](https://doc.rust-lang.org/stable/std/collections/struct.VecDeque.html#method.new)

<a id="1.68.0-Cargo"></a>

Cargo
-----

- [Stabilize sparse registry support for crates.io](https://github.com/rust-lang/cargo/pull/11224/)
- [`cargo build --verbose` tells you more about why it recompiles.](https://github.com/rust-lang/cargo/pull/11407/)
- [Show progress of crates.io index update even `net.git-fetch-with-cli` option enabled](https://github.com/rust-lang/cargo/pull/11579/)

<a id="1.68.0-Misc"></a>

Misc
----

<a id="1.68.0-Compatibility-Notes"></a>

Compatibility Notes
-------------------

- [Only support Android NDK 25 or newer](https://blog.rust-lang.org/2023/01/09/android-ndk-update-r25.html)
- [Add `SEMICOLON_IN_EXPRESSIONS_FROM_MACROS` to future-incompat report](https://github.com/rust-lang/rust/pull/103418/)
- [Only specify `--target` by default for `-Zgcc-ld=lld` on wasm](https://github.com/rust-lang/rust/pull/101792/)
- [Bump `IMPLIED_BOUNDS_ENTAILMENT` to Deny + ReportNow](https://github.com/rust-lang/rust/pull/106465/)
- [`std::task::Context` no longer implements Send and Sync](https://github.com/rust-lang/rust/pull/95985)

<a id="1.68.0-Internal-Changes"></a>

Internal Changes
----------------

These changes do not affect any public interfaces of Rust, but they represent
significant improvements to the performance or internals of rustc and related
tools.

- [Encode spans relative to the enclosing item](https://github.com/rust-lang/rust/pull/84762/)
- [Don't normalize in AstConv](https://github.com/rust-lang/rust/pull/101947/)
- [Find the right lower bound region in the scenario of partial order relations](https://github.com/rust-lang/rust/pull/104765/)
- [Fix impl block in const expr](https://github.com/rust-lang/rust/pull/104889/)
- [Check ADT fields for copy implementations considering regions](https://github.com/rust-lang/rust/pull/105102/)
- [rustdoc: simplify JS search routine by not messing with lev distance](https://github.com/rust-lang/rust/pull/105796/)
- [Enable ThinLTO for rustc on `x86_64-pc-windows-msvc`](https://github.com/rust-lang/rust/pull/103591/)
- [Enable ThinLTO for rustc on `x86_64-apple-darwin`](https://github.com/rust-lang/rust/pull/103647/)

Version 1.67.1 (2023-02-09)
===========================

- [Fix interoperability with thin archives.](https://github.com/rust-lang/rust/pull/107360)
- [Fix an internal error in the compiler build process.](https://github.com/rust-lang/rust/pull/105624)
- [Downgrade `clippy::uninlined_format_args` to pedantic.](https://github.com/rust-lang/rust-clippy/pull/10265)

Version 1.67.0 (2023-01-26)
==========================

<a id="1.67.0-Language"></a>

Language
--------

- [Make `Sized` predicates coinductive, allowing cycles.](https://github.com/rust-lang/rust/pull/100386/)
- [`#[must_use]` annotations on `async fn` also affect the `Future::Output`.](https://github.com/rust-lang/rust/pull/100633/)
- [Elaborate supertrait obligations when deducing closure signatures.](https://github.com/rust-lang/rust/pull/101834/)
- [Invalid literals are no longer an error under `cfg(FALSE)`.](https://github.com/rust-lang/rust/pull/102944/)
- [Unreserve braced enum variants in value namespace.](https://github.com/rust-lang/rust/pull/103578/)

<a id="1.67.0-Compiler"></a>

Compiler
--------

- [Enable varargs support for calling conventions other than `C` or `cdecl`.](https://github.com/rust-lang/rust/pull/97971/)
- [Add new MIR constant propagation based on dataflow analysis.](https://github.com/rust-lang/rust/pull/101168/)
- [Optimize field ordering by grouping m\*2^n-sized fields with equivalently aligned ones.](https://github.com/rust-lang/rust/pull/102750/)
- [Stabilize native library modifier `verbatim`.](https://github.com/rust-lang/rust/pull/104360/)

Added, updated, and removed targets:

- [Add a tier 3 target for PowerPC on AIX](https://github.com/rust-lang/rust/pull/102293/), `powerpc64-ibm-aix`.
- [Add a tier 3 target for the Sony PlayStation 1](https://github.com/rust-lang/rust/pull/102689/), `mipsel-sony-psx`.
- [Add tier 3 `no_std` targets for the QNX Neutrino RTOS](https://github.com/rust-lang/rust/pull/102701/),
  `aarch64-unknown-nto-qnx710` and `x86_64-pc-nto-qnx710`.
- [Promote UEFI targets to tier 2](https://github.com/rust-lang/rust/pull/103933/), `aarch64-unknown-uefi`, `i686-unknown-uefi`, and `x86_64-unknown-uefi`.
- [Remove tier 3 `linuxkernel` targets](https://github.com/rust-lang/rust/pull/104015/) (not used by the actual kernel).

Refer to Rust's [platform support page][platform-support-doc]
for more information on Rust's tiered platform support.

<a id="1.67.0-Libraries"></a>

Libraries
---------

- [Merge `crossbeam-channel` into `std::sync::mpsc`.](https://github.com/rust-lang/rust/pull/93563/)
- [Fix inconsistent rounding of 0.5 when formatted to 0 decimal places.](https://github.com/rust-lang/rust/pull/102935/)
- [Derive `Eq` and `Hash` for `ControlFlow`.](https://github.com/rust-lang/rust/pull/103084/)
- [Don't build `compiler_builtins` with `-C panic=abort`.](https://github.com/rust-lang/rust/pull/103786/)

<a id="1.67.0-Stabilized-APIs"></a>

Stabilized APIs
---------------

- [`{integer}::checked_ilog`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.checked_ilog)
- [`{integer}::checked_ilog2`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.checked_ilog2)
- [`{integer}::checked_ilog10`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.checked_ilog10)
- [`{integer}::ilog`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.ilog)
- [`{integer}::ilog2`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.ilog2)
- [`{integer}::ilog10`](https://doc.rust-lang.org/stable/std/primitive.i32.html#method.ilog10)
- [`NonZeroU*::ilog2`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroU32.html#method.ilog2)
- [`NonZeroU*::ilog10`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroU32.html#method.ilog10)
- [`NonZero*::BITS`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroU32.html#associatedconstant.BITS)

These APIs are now stable in const contexts:

- [`char::from_u32`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.from_u32)
- [`char::from_digit`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.from_digit)
- [`char::to_digit`](https://doc.rust-lang.org/stable/std/primitive.char.html#method.to_digit)
- [`core::char::from_u32`](https://doc.rust-lang.org/stable/core/char/fn.from_u32.html)
- [`core::char::from_digit`](https://doc.rust-lang.org/stable/core/char/fn.from_digit.html)

<a id="1.67.0-Compatibility-Notes"></a>

Compatibility Notes
-------------------

- [The layout of `repr(Rust)` types now groups m\*2^n-sized fields with
  equivalently aligned ones.](https://github.com/rust-lang/rust/pull/102750/)
  This is intended to be an optimization, but it is also known to increase type
  sizes in a few cases for the placement of enum tags. As a reminder, the layout
  of `repr(Rust)` types is an implementation detail, subject to change.
- [0.5 now rounds to 0 when formatted to 0 decimal places.](https://github.com/rust-lang/rust/pull/102935/)
  This makes it consistent with the rest of floating point formatting that
  rounds ties toward even digits.
- [Chains of `&&` and `||` will now drop temporaries from their sub-expressions in
  evaluation order, left-to-right.](https://github.com/rust-lang/rust/pull/103293/)
  Previously, it was "twisted" such that the _first_ expression dropped its
  temporaries _last_, after all of the other expressions dropped in order.
- [Underscore suffixes on string literals are now a hard error.](https://github.com/rust-lang/rust/pull/103914/)
  This has been a future-compatibility warning since 1.20.0.
- [Stop passing `-export-dynamic` to `wasm-ld`.](https://github.com/rust-lang/rust/pull/105405/)
- [`main` is now mangled as `__main_void` on `wasm32-wasi`.](https://github.com/rust-lang/rust/pull/105468/)
- [Cargo now emits an error if there are multiple registries in the configuration
  with the same index URL.](https://github.com/rust-lang/cargo/pull/10592)

<a id="1.67.0-Internal-Changes"></a>

Internal Changes
----------------

These changes do not affect any public interfaces of Rust, but they represent
significant improvements to the performance or internals of rustc and related
tools.

- [Rewrite LLVM's archive writer in Rust.](https://github.com/rust-lang/rust/pull/97485/)

Version 1.66.1 (2023-01-10)
===========================

- Added validation of SSH host keys for git URLs in Cargo ([CVE-2022-46176](https://www.cve.org/CVERecord?id=CVE-2022-46176))

Version 1.66.0 (2022-12-15)
==========================

Language
--------
- [Permit specifying explicit discriminants on all `repr(Int)` enums](https://github.com/rust-lang/rust/pull/95710/)
  ```rust
  #[repr(u8)]
  enum Foo {
      A(u8) = 0,
      B(i8) = 1,
      C(bool) = 42,
  }
  ```
- [Allow transmutes between the same type differing only in lifetimes](https://github.com/rust-lang/rust/pull/101520/)
- [Change constant evaluation errors from a deny-by-default lint to a hard error](https://github.com/rust-lang/rust/pull/102091/)
- [Trigger `must_use` on `impl Trait` for supertraits](https://github.com/rust-lang/rust/pull/102287/)
  This makes `impl ExactSizeIterator` respect the existing `#[must_use]` annotation on `Iterator`.
- [Allow `..=X` in patterns](https://github.com/rust-lang/rust/pull/102275/)
- [Uplift `clippy::for_loops_over_fallibles` lint into rustc](https://github.com/rust-lang/rust/pull/99696/)
- [Stabilize `sym` operands in inline assembly](https://github.com/rust-lang/rust/pull/103168/)
- [Update to Unicode 15](https://github.com/rust-lang/rust/pull/101912/)
- [Opaque types no longer imply lifetime bounds](https://github.com/rust-lang/rust/pull/95474/)
  This is a soundness fix which may break code that was erroneously relying on this behavior.

Compiler
--------
- [Add armv5te-none-eabi and thumbv5te-none-eabi tier 3 targets](https://github.com/rust-lang/rust/pull/101329/)
  - Refer to Rust's [platform support page][platform-support-doc] for more
    information on Rust's tiered platform support.
- [Add support for linking against macOS universal libraries](https://github.com/rust-lang/rust/pull/98736)

Libraries
---------
- [Fix `#[derive(Default)]` on a generic `#[default]` enum adding unnecessary `Default` bounds](https://github.com/rust-lang/rust/pull/101040/)
- [Update to Unicode 15](https://github.com/rust-lang/rust/pull/101821/)

Stabilized APIs
---------------

- [`proc_macro::Span::source_text`](https://doc.rust-lang.org/stable/proc_macro/struct.Span.html#method.source_text)
- [`uX::{checked_add_signed, overflowing_add_signed, saturating_add_signed, wrapping_add_signed}`](https://doc.rust-lang.org/stable/std/primitive.u8.html#method.checked_add_signed)
- [`iX::{checked_add_unsigned, overflowing_add_unsigned, saturating_add_unsigned, wrapping_add_unsigned}`](https://doc.rust-lang.org/stable/std/primitive.i8.html#method.checked_add_unsigned)
- [`iX::{checked_sub_unsigned, overflowing_sub_unsigned, saturating_sub_unsigned, wrapping_sub_unsigned}`](https://doc.rust-lang.org/stable/std/primitive.i8.html#method.checked_sub_unsigned)
- [`BTreeSet::{first, last, pop_first, pop_last}`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html#method.first)
- [`BTreeMap::{first_key_value, last_key_value, first_entry, last_entry, pop_first, pop_last}`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeMap.html#method.first_key_value)
- [Add `AsFd` implementations for stdio lock types on WASI.](https://github.com/rust-lang/rust/pull/101768/)
- [`impl TryFrom<Vec<T>> for Box<[T; N]>`](https://doc.rust-lang.org/stable/std/boxed/struct.Box.html#impl-TryFrom%3CVec%3CT%2C%20Global%3E%3E-for-Box%3C%5BT%3B%20N%5D%2C%20Global%3E)
- [`core::hint::black_box`](https://doc.rust-lang.org/stable/std/hint/fn.black_box.html)
- [`Duration::try_from_secs_{f32,f64}`](https://doc.rust-lang.org/stable/std/time/struct.Duration.html#method.try_from_secs_f32)
- [`Option::unzip`](https://doc.rust-lang.org/stable/std/option/enum.Option.html#method.unzip)
- [`std::os::fd`](https://doc.rust-lang.org/stable/std/os/fd/index.html)


Rustdoc
-------

- [Add Rustdoc warning for invalid HTML tags in the documentation](https://github.com/rust-lang/rust/pull/101720/)

Cargo
-----

- [Added `cargo remove` to remove dependencies from Cargo.toml](https://doc.rust-lang.org/nightly/cargo/commands/cargo-remove.html)
- [`cargo publish` now waits for the new version to be downloadable before exiting](https://github.com/rust-lang/cargo/pull/11062)

See [detailed release notes](https://github.com/rust-lang/cargo/blob/master/CHANGELOG.md#cargo-166-2022-12-15) for more.

Compatibility Notes
-------------------

- [Only apply `ProceduralMasquerade` hack to older versions of `rental`](https://github.com/rust-lang/rust/pull/94063/)
- [Don't export `__heap_base` and `__data_end` on wasm32-wasi.](https://github.com/rust-lang/rust/pull/102385/)
- [Don't export `__wasm_init_memory` on WebAssembly.](https://github.com/rust-lang/rust/pull/102426/)
- [Only export `__tls_*` on wasm32-unknown-unknown.](https://github.com/rust-lang/rust/pull/102440/)
- [Don't link to `libresolv` in libstd on Darwin](https://github.com/rust-lang/rust/pull/102766/)
- [Update libstd's libc to 0.2.135 (to make `libstd` no longer pull in `libiconv.dylib` on Darwin)](https://github.com/rust-lang/rust/pull/103277/)
- [Opaque types no longer imply lifetime bounds](https://github.com/rust-lang/rust/pull/95474/)
  This is a soundness fix which may break code that was erroneously relying on this behavior.
- [Make `order_dependent_trait_objects` show up in future-breakage reports](https://github.com/rust-lang/rust/pull/102635/)
- [Change std::process::Command spawning to default to inheriting the parent's signal mask](https://github.com/rust-lang/rust/pull/101077/)

Internal Changes
----------------

These changes do not affect any public interfaces of Rust, but they represent
significant improvements to the performance or internals of rustc and related
tools.

- [Enable BOLT for LLVM compilation](https://github.com/rust-lang/rust/pull/94381/)
- [Enable LTO for rustc_driver.so](https://github.com/rust-lang/rust/pull/101403/)

Version 1.65.0 (2022-11-03)
==========================

Language
--------
- [Error on `as` casts of enums with `#[non_exhaustive]` variants](https://github.com/rust-lang/rust/pull/92744/)
- [Stabilize `let else`](https://github.com/rust-lang/rust/pull/93628/)
- [Stabilize generic associated types (GATs)](https://github.com/rust-lang/rust/pull/96709/)
- [Add lints `let_underscore_drop` and `let_underscore_lock` from Clippy](https://github.com/rust-lang/rust/pull/97739/)
- [Stabilize `break`ing from arbitrary labeled blocks ("label-break-value")](https://github.com/rust-lang/rust/pull/99332/)
- [Uninitialized integers, floats, and raw pointers are now considered immediate UB](https://github.com/rust-lang/rust/pull/98919/).
  Usage of `MaybeUninit` is the correct way to work with uninitialized memory.
- [Stabilize raw-dylib for Windows x86_64, aarch64, and thumbv7a](https://github.com/rust-lang/rust/pull/99916/)
- [Do not allow `Drop` impl on foreign ADTs](https://github.com/rust-lang/rust/pull/99576/)

Compiler
--------
- [Stabilize -Csplit-debuginfo on Linux](https://github.com/rust-lang/rust/pull/98051/)
- [Use niche-filling optimization even when multiple variants have data](https://github.com/rust-lang/rust/pull/94075/)
- [Associated type projections are now verified to be well-formed prior to resolving the underlying type](https://github.com/rust-lang/rust/pull/99217/#issuecomment-1209365630)
- [Stringify non-shorthand visibility correctly](https://github.com/rust-lang/rust/pull/100350/)
- [Normalize struct field types when unsizing](https://github.com/rust-lang/rust/pull/101831/)
- [Update to LLVM 15](https://github.com/rust-lang/rust/pull/99464/)
- [Fix aarch64 call abi to correctly zeroext when needed](https://github.com/rust-lang/rust/pull/97800/)
- [debuginfo: Generalize C++-like encoding for enums](https://github.com/rust-lang/rust/pull/98393/)
- [Add `special_module_name` lint](https://github.com/rust-lang/rust/pull/94467/)
- [Add support for generating unique profraw files by default when using `-C instrument-coverage`](https://github.com/rust-lang/rust/pull/100384/)
- [Allow dynamic linking for iOS/tvOS targets](https://github.com/rust-lang/rust/pull/100636/)

New targets:

- [Add armv4t-none-eabi as a tier 3 target](https://github.com/rust-lang/rust/pull/100244/)
- [Add powerpc64-unknown-openbsd and riscv64-unknown-openbsd as tier 3 targets](https://github.com/rust-lang/rust/pull/101025/)
  - Refer to Rust's [platform support page][platform-support-doc] for more
    information on Rust's tiered platform support.

Libraries
---------

- [Don't generate `PartialEq::ne` in derive(PartialEq)](https://github.com/rust-lang/rust/pull/98655/)
- [Windows RNG: Use `BCRYPT_RNG_ALG_HANDLE` by default](https://github.com/rust-lang/rust/pull/101325/)
- [Forbid mixing `System` with direct system allocator calls](https://github.com/rust-lang/rust/pull/101394/)
- [Document no support for writing to non-blocking stdio/stderr](https://github.com/rust-lang/rust/pull/101416/)
- [`std::layout::Layout` size must not overflow `isize::MAX` when rounded up to `align`](https://github.com/rust-lang/rust/pull/95295)
  This also changes the safety conditions on `Layout::from_size_align_unchecked`.

Stabilized APIs
---------------

- [`std::backtrace::Backtrace`](https://doc.rust-lang.org/stable/std/backtrace/struct.Backtrace.html)
- [`Bound::as_ref`](https://doc.rust-lang.org/stable/std/ops/enum.Bound.html#method.as_ref)
- [`std::io::read_to_string`](https://doc.rust-lang.org/stable/std/io/fn.read_to_string.html)
- [`<*const T>::cast_mut`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.cast_mut)
- [`<*mut T>::cast_const`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.cast_const)

These APIs are now stable in const contexts:

- [`<*const T>::offset_from`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset_from)
- [`<*mut T>::offset_from`](https://doc.rust-lang.org/stable/std/primitive.pointer.html#method.offset_from)

Cargo
-----

- [Apply GitHub fast path even for partial hashes](https://github.com/rust-lang/cargo/pull/10807/)
- [Do not add home bin path to PATH if it's already there](https://github.com/rust-lang/cargo/pull/11023/)
- [Take priority into account within the pending queue](https://github.com/rust-lang/cargo/pull/11032/).
  This slightly optimizes job scheduling by Cargo, with typically small improvements on larger crate graph builds.

Compatibility Notes
-------------------

- [`std::layout::Layout` size must not overflow `isize::MAX` when rounded up to `align`](https://github.com/rust-lang/rust/pull/95295).
  This also changes the safety conditions on `Layout::from_size_align_unchecked`.
- [`PollFn` now only implements `Unpin` if the closure is `Unpin`](https://github.com/rust-lang/rust/pull/102737).
  This is a possible breaking change if users were relying on the blanket unpin implementation.
  See discussion on the PR for details of why this change was made.
- [Drop ExactSizeIterator impl from std::char::EscapeAscii](https://github.com/rust-lang/rust/pull/99880)
  This is a backwards-incompatible change to the standard library's surface
  area, but is unlikely to affect real world usage.
- [Do not consider a single repeated lifetime eligible for elision in the return type](https://github.com/rust-lang/rust/pull/103450)
  This behavior was unintentionally changed in 1.64.0, and this release reverts that change by making this an error again.
- [Reenable disabled early syntax gates as future-incompatibility lints](https://github.com/rust-lang/rust/pull/99935/)
- [Update the minimum external LLVM to 13](https://github.com/rust-lang/rust/pull/100460/)
- [Don't duplicate file descriptors into stdio fds](https://github.com/rust-lang/rust/pull/101426/)
- [Sunset RLS](https://github.com/rust-lang/rust/pull/100863/)
- [Deny usage of `#![cfg_attr(..., crate_type = ...)]` to set the crate type](https://github.com/rust-lang/rust/pull/99784/)
  This strengthens the forward compatibility lint deprecated_cfg_attr_crate_type_name to deny.
- [`llvm-has-rust-patches` allows setting the build system to treat the LLVM as having Rust-specific patches](https://github.com/rust-lang/rust/pull/101072)
  This option may need to be set for distributions that are building Rust with a patched LLVM via `llvm-config`, not the built-in LLVM.
- Combining three or more languages (e.g. Objective C, C++ and Rust) into one binary may hit linker limitations when using `lld`. For more information, see [issue 102754][102754].

[102754]: https://github.com/rust-lang/rust/issues/102754

Internal Changes
----------------

These changes do not affect any public interfaces of Rust, but they represent
significant improvements to the performance or internals of rustc and related
tools.

- [Add `x.sh` and `x.ps1` shell scripts](https://github.com/rust-lang/rust/pull/99992/)
- [compiletest: use target cfg instead of hard-coded tables](https://github.com/rust-lang/rust/pull/100260/)
- [Use object instead of LLVM for reading bitcode from rlibs](https://github.com/rust-lang/rust/pull/98100/)
- [Enable MIR inlining for optimized compilations](https://github.com/rust-lang/rust/pull/91743)
  This provides a 3-10% improvement in compiletimes for real world crates. See [perf results](https://perf.rust-lang.org/compare.html?start=aedf78e56b2279cc869962feac5153b6ba7001ed&end=0075bb4fad68e64b6d1be06bf2db366c30bc75e1&stat=instructions:u).

Version 1.64.0 (2022-09-22)
===========================

Language
--------
- [Unions with mutable references or tuples of allowed types are now allowed](https://github.com/rust-lang/rust/pull/97995/)
- It is now considered valid to deallocate memory pointed to by a shared reference `&T` [if every byte in `T` is inside an `UnsafeCell`](https://github.com/rust-lang/rust/pull/98017/)
- Unused tuple struct fields are now warned against in an allow-by-default lint, [`unused_tuple_struct_fields`](https://github.com/rust-lang/rust/pull/95977/), similar to the existing warning for unused struct fields. This lint will become warn-by-default in the future.

Compiler
--------
- [Add Nintendo Switch as tier 3 target](https://github.com/rust-lang/rust/pull/88991/)
  - Refer to Rust's [platform support page][platform-support-doc] for more
    information on Rust's tiered platform support.
- [Only compile `#[used]` as llvm.compiler.used for ELF targets](https://github.com/rust-lang/rust/pull/93718/)
- [Add the `--diagnostic-width` compiler flag to define the terminal width.](https://github.com/rust-lang/rust/pull/95635/)
- [Add support for link-flavor `rust-lld` for iOS, tvOS and watchOS](https://github.com/rust-lang/rust/pull/98771/)

Libraries
---------
- [Remove restrictions on compare-exchange memory ordering.](https://github.com/rust-lang/rust/pull/98383/)
- You can now `write!` or `writeln!` into an `OsString`: [Implement `fmt::Write` for `OsString`](https://github.com/rust-lang/rust/pull/97915/)
- [Make RwLockReadGuard covariant](https://github.com/rust-lang/rust/pull/96820/)
- [Implement `FusedIterator` for `std::net::[Into]Incoming`](https://github.com/rust-lang/rust/pull/97300/)
- [`impl<T: AsRawFd> AsRawFd for {Arc,Box}<T>`](https://github.com/rust-lang/rust/pull/97437/)
- [`ptr::copy` and `ptr::swap` are doing untyped copies](https://github.com/rust-lang/rust/pull/97712/)
- [Add cgroupv1 support to `available_parallelism`](https://github.com/rust-lang/rust/pull/97925/)
- [Mitigate many incorrect uses of `mem::uninitialized`](https://github.com/rust-lang/rust/pull/99182/)

Stabilized APIs
---------------

- [`future::IntoFuture`](https://doc.rust-lang.org/stable/std/future/trait.IntoFuture.html)
- [`future::poll_fn`](https://doc.rust-lang.org/stable/std/future/fn.poll_fn.html)
- [`task::ready!`](https://doc.rust-lang.org/stable/std/task/macro.ready.html)
- [`num::NonZero*::checked_mul`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroUsize.html#method.checked_mul)
- [`num::NonZero*::checked_pow`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroUsize.html#method.checked_pow)
- [`num::NonZero*::saturating_mul`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroUsize.html#method.saturating_mul)
- [`num::NonZero*::saturating_pow`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroUsize.html#method.saturating_pow)
- [`num::NonZeroI*::abs`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroIsize.html#method.abs)
- [`num::NonZeroI*::checked_abs`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroIsize.html#method.checked_abs)
- [`num::NonZeroI*::overflowing_abs`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroIsize.html#method.overflowing_abs)
- [`num::NonZeroI*::saturating_abs`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroIsize.html#method.saturating_abs)
- [`num::NonZeroI*::unsigned_abs`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroIsize.html#method.unsigned_abs)
- [`num::NonZeroI*::wrapping_abs`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroIsize.html#method.wrapping_abs)
- [`num::NonZeroU*::checked_add`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroUsize.html#method.checked_add)
- [`num::NonZeroU*::checked_next_power_of_two`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroUsize.html#method.checked_next_power_of_two)
- [`num::NonZeroU*::saturating_add`](https://doc.rust-lang.org/stable/std/num/struct.NonZeroUsize.html#method.saturating_add)
- [`os::unix::process::CommandExt::process_group`](https://doc.rust-lang.org/stable/std/os/unix/process/trait.CommandExt.html#tymethod.process_group)
- [`os::windows::fs::FileTypeExt::is_symlink_dir`](https://doc.rust-lang.org/stable/std/os/windows/fs/trait.FileTypeExt.html#tymethod.is_symlink_dir)
- [`os::windows::fs::FileTypeExt::is_symlink_file`](https://doc.rust-lang.org/stable/std/os/windows/fs/trait.FileTypeExt.html#tymethod.is_symlink_file)

These types were previously stable in `std::ffi`, but are now also available in `core` and `alloc`:

- [`core::ffi::CStr`](https://doc.rust-lang.org/stable/core/ffi/struct.CStr.html)
- [`core::ffi::FromBytesWithNulError`](https://doc.rust-lang.org/stable/core/ffi/struct.FromBytesWithNulError.html)
- [`alloc::ffi::CString`](https://doc.rust-lang.org/stable/alloc/ffi/struct.CString.html)
- [`alloc::ffi::FromVecWithNulError`](https://doc.rust-lang.org/stable/alloc/ffi/struct.FromVecWithNulError.html)
- [`alloc::ffi::IntoStringError`](https://doc.rust-lang.org/stable/alloc/ffi/struct.IntoStringError.html)
- [`alloc::ffi::NulError`](https://doc.rust-lang.org/stable/alloc/ffi/struct.NulError.html)

These types were previously stable in `std::os::raw`, but are now also available in `core::ffi` and `std::ffi`:

- [`ffi::c_char`](https://doc.rust-lang.org/stable/std/ffi/type.c_char.html)
- [`ffi::c_double`](https://doc.rust-lang.org/stable/std/ffi/type.c_double.html)
- [`ffi::c_float`](https://doc.rust-lang.org/stable/std/ffi/type.c_float.html)
- [`ffi::c_int`](https://doc.rust-lang.org/stable/std/ffi/type.c_int.html)
- [`ffi::c_long`](https://doc.rust-lang.org/stable/std/ffi/type.c_long.html)
- [`ffi::c_longlong`](https://doc.rust-lang.org/stable/std/ffi/type.c_longlong.html)
- [`ffi::c_schar`](https://doc.rust-lang.org/stable/std/ffi/type.c_schar.html)
- [`ffi::c_short`](https://doc.rust-lang.org/stable/std/ffi/type.c_short.html)
- [`ffi::c_uchar`](https://doc.rust-lang.org/stable/std/ffi/type.c_uchar.html)
- [`ffi::c_uint`](https://doc.rust-lang.org/stable/std/ffi/type.c_uint.html)
- [`ffi::c_ulong`](https://doc.rust-lang.org/stable/std/ffi/type.c_ulong.html)
- [`ffi::c_ulonglong`](https://doc.rust-lang.org/stable/std/ffi/type.c_ulonglong.html)
- [`ffi::c_ushort`](https://doc.rust-lang.org/stable/std/ffi/type.c_ushort.html)

These APIs are now usable in const contexts:

- [`slice::from_raw_parts`](https://doc.rust-lang.org/stable/core/slice/fn.from_raw_parts.html)

Cargo
-----
- [Packages can now inherit settings from the workspace so that the settings
  can be centralized in one place.](https://github.com/rust-lang/cargo/pull/10859) See
  [`workspace.package`](https://doc.rust-lang.org/nightly/cargo/reference/workspaces.html#the-workspacepackage-table)
  and
  [`workspace.dependencies`](https://doc.rust-lang.org/nightly/cargo/reference/workspaces.html#the-workspacedependencies-table)
  for more details on how to define these common settings.
- [Cargo commands can now accept multiple `--target` flags to build for
  multiple targets at once](https://github.com/rust-lang/cargo/pull/10766), and the
  [`build.target`](https://doc.rust-lang.org/nightly/cargo/reference/config.html#buildtarget)
  config option may now take an array of multiple targets.
- [The `--jobs` argument can now take a negative number to count backwards from
  the max CPUs.](https://github.com/rust-lang/cargo/pull/10844)
- [`cargo add` will now update `Cargo.lock`.](https://github.com/rust-lang/cargo/pull/10902)
- [Added](https://github.com/rust-lang/cargo/pull/10838) the
  [`--crate-type`](https://doc.rust-lang.org/nightly/cargo/commands/cargo-rustc.html#option-cargo-rustc---crate-type)
  flag to `cargo rustc` to override the crate type.
- [Significantly improved the performance fetching git dependencies from GitHub
  when using a hash in the `rev` field.](https://github.com/rust-lang/cargo/pull/10079)

Misc
----
- [The `rust-analyzer` rustup component is now available on the stable channel.](https://github.com/rust-lang/rust/pull/98640/)

Compatibility Notes
-------------------
- The minimum required versions for all `-linux-gnu` targets are now at least kernel 3.2 and glibc 2.17, for targets that previously supported older versions: [Increase the minimum linux-gnu versions](https://github.com/rust-lang/rust/pull/95026/)
- [Network primitives are now implemented with the ideal Rust layout, not the C system layout](https://github.com/rust-lang/rust/pull/78802/). This can cause problems when transmuting the types.
- [Add assertion that `transmute_copy`'s `U` is not larger than `T`](https://github.com/rust-lang/rust/pull/98839/)
- [A soundness bug in `BTreeMap` was fixed](https://github.com/rust-lang/rust/pull/99413/) that allowed data it was borrowing to be dropped before the container.
- [The Drop behavior of C-like enums cast to ints has changed](https://github.com/rust-lang/rust/pull/96862/). These are already discouraged by a compiler warning.
- [Relate late-bound closure lifetimes to parent fn in NLL](https://github.com/rust-lang/rust/pull/98835/)
- [Errors at const-eval time are now in future incompatibility reports](https://github.com/rust-lang/rust/pull/97743/)
- On the `thumbv6m-none-eabi` target, some incorrect `asm!` statements were erroneously accepted if they used the high registers (r8 to r14) as an input/output operand. [This is no longer accepted](https://github.com/rust-lang/rust/pull/99155/).
- [`impl Trait` was accidentally accepted as the associated type value of return-position `impl Trait`](https://github.com/rust-lang/rust/pull/97346/), without fulfilling all the trait bounds of that associated type, as long as the hidden type satisfies said bounds. This has been fixed.

Internal Changes
----------------

These changes do not affect any public interfaces of Rust, but they represent
significant improvements to the performance or internals of rustc and related
tools.

- Windows builds now use profile-guided optimization, providing 10-20% improvements to compiler performance: [Utilize PGO for windows x64 rustc dist builds](https://github.com/rust-lang/rust/pull/96978/)
- [Stop keeping metadata in memory before writing it to disk](https://github.com/rust-lang/rust/pull/96544/)
- [compiletest: strip debuginfo by default for mode=ui](https://github.com/rust-lang/rust/pull/98140/)
- Many improvements to generated code for derives, including performance improvements:
  - [Don't use match-destructuring for derived ops on structs.](https://github.com/rust-lang/rust/pull/98446/)
  - [Many small deriving cleanups](https://github.com/rust-lang/rust/pull/98741/)
  - [More derive output improvements](https://github.com/rust-lang/rust/pull/98758/)
  - [Clarify deriving code](https://github.com/rust-lang/rust/pull/98915/)
  - [Final derive output improvements](https://github.com/rust-lang/rust/pull/99046/)
  - [Stop injecting `#[allow(unused_qualifications)]` in generated `derive` implementations](https://github.com/rust-lang/rust/pull/99485/)
  - [Improve `derive(Debug)`](https://github.com/rust-lang/rust/pull/98190/)
- [Bump to clap 3](https://github.com/rust-lang/rust/pull/98213/)
- [fully move dropck to mir](https://github.com/rust-lang/rust/pull/98641/)
- [Optimize `Vec::insert` for the case where `index == len`.](https://github.com/rust-lang/rust/pull/98755/)
- [Convert rust-analyzer to an in-tree tool](https://github.com/rust-lang/rust/pull/99603/)

