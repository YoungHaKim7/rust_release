<p align="center">
    <img width=70px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" />
</p>

# link

- [최근-러스트-업데이트-러스트Weekly & etc뉴스](#최근-러스트-업데이트-weeklyetc뉴스)
  - [cratesio관련-뉴스](#cratesio관련-뉴스)
- [러스트-라이브러리-찾기librs--cratesio](#러스트-라이브러리-찾기librs--cratesio)

- [WASM-빌드하기target-add](#wasm-빌드하기target-add)
- [cargo-ructc-mir-hir보는법](#cargo-ructc-mir-hir보는법)
- [cargo asm사용법](/07_cargo_rustc_mir_hir_llvm#cargo-asm-기타-등등)

<hr>

# 공부할게 많은 버젼별 Rust-clippy

https://rust-lang.github.io/rust-clippy/

# rustc --version --verbose

- test한 환경을 남에게 보여줄때 좋다.

```
$ rustc --version --verbose
rustc 1.70.0 (90c541806 2023-05-31)
binary: rustc
commit-hash: 90c541806f23a127002de5b4038be731ba1458ca
commit-date: 2023-05-31
host: aarch64-apple-darwin
release: 1.70.0
LLVM version: 16.0.2
```

<hr>

# Rust 러스트 업데이트 내용 미리 보기

- spoiler book ㅋㅋ 

- Spoiler: there are loads.↩︎

Luca Palmieri. Zero To Production In Rust (Kindle Locations 595-596). Kindle Edition. 

https://doc.rust-lang.org/nightly/unstable-book/the-unstable-book.html

# Debug information is not included in build scripts by default anymore(속도 올리려고 1.69에서 디버그 정보 빠짐 다시 넣는 방법)

- If you want to debug a build script, you can add this snippet to your ```Cargo.toml``` to emit debug information again:
- Cargo.toml https://blog.rust-lang.org/2023/04/20/Rust-1.69.0.html

```
[profile.dev.build-override]
debug = true
[profile.release.build-override]
debug = true
```
<br>

<hr>

# Rust version Setting

```
rustup update stable
```

- Version control

```
rustup default stable
rustup default beta
rustup default nightly
```

<br>

<hr>

# Rust Relese 노트 미리 알아보기

https://releases.rs/docs/

<hr>

# 최근 러스트 업데이트 weekly&etc뉴스

- 러스트 Weekly
  - https://this-week-in-rust.org/

- crates.io: Download changes
  - Mar. 11, 2024 · Tobias Bieniek on behalf of the crates.io team
    - https://blog.rust-lang.org/2024/03/11/crates-io-download-changes.html

<hr>

<br>

# 러스트 라이브러리 찾기(lib.rs & crates.io)

- 라이브러리 다운로드 수 같은거 볼 수 있음 러스트 라이브러리 정리
  - https://lib.rs/

-  The Rust community’s crate registry
  - https://crates.io/

# rust_release

rust release 제일 빨리 알려주는 트위터

[Mara Bos Twitter](https://twitter.com/m_ou_se)

https://twitter.com/m_ou_se

- 이 분의 rust vs cpp concurrency

https://blog.m-ou.se/rust-cpp-concurrency/

<br>

# What Rust is it

https://www.whatrustisit.com/

<br>

<br>
- Releases 노트 지금까지 나온거 쭉 볼 수 있다.

https://github.com/rust-lang/rust/blob/master/RELEASES.md

<br>

https://github.com/rust-lang/rust/blob/1.64.0/RELEASES.md

<br>

# Rust 공식 Blog

https://blog.rust-lang.org/

<hr>

# Rust Code 배울게 많다.
https://github.com/oli-obk

<hr>

<br>

# Asynchronous Programming in Rust \_1.63 부터 이해해야할 Async

https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html

# cargo add 활용법

```
$ cargo add serde -F serde/derive && cargo add tokio -F tokio/full


// short ver.
$ cargo add serde -F derive && cargo add tokio -F full
```

- 알아서 이쁘게 들어간다.

Cargo.toml

```
[dependencies]
serde = { version = "1.0.140", features = ["derive"] }
tokio = { version = "1.20.1", features = ["full"] }
```

cargo add 활용법❤️귀찮은 features넣는 법 -F이게 좋네 ㅋ❤️(Cargo.toml 넣기 귀찮다. ㅋㅋ)

https://economiceco.tistory.com/m/14544

- 다른 cargo add

```
$ cargo add serde tokio -F serde/derive -F tokio/full

    Updating crates.io index
      Adding serde v1.0.159 to dependencies.
             Features:
             + derive
             + serde_derive
             + std
             - alloc
             - rc
             - unstable
      Adding tokio v1.27.0 to dependencies.
             Features:
             + bytes
             + fs
             + full
             + io-std
             + io-util
             + libc
             + macros
             + net
             + num_cpus
             + parking_lot
             + process
```

# ```cargo clean``` 활용법(target을 그냥 지우기 보단 cargo clean을 활용하자)

```
$ cargo clean

     Removed 347 files, 102.8MiB total

```

- target폴더 같은거 찾아서 지워준다 굿 👍Cleans dependencies and build artifacts from your projects. 
  - https://github.com/tbillington/kondo


<br>

<hr>

# 중국 사람이 정리한 Rust eBook 번역해서 볼만함

https://github.com/sunface/rust-course

<br>

- 이거 eBook 장난아니게 버젼별 정리까지 최고

https://course.rs/about-book.html

<br>

- Rust by practice

영문 버전 중국어 보다 보기 편하다. ^^;

https://practice.rs/why-exercise.html

https://github.com/sunface/rust-by-practice

<br>

- Cook Book

https://rusty.rs/about.html

<br>

# Rustup show

```
PS D:\rust_toolchain_toml> rustup show
Default host: x86_64-pc-windows-msvc
rustup home:  C:\Users\user\.rustup

installed toolchains
--------------------

stable-x86_64-pc-windows-msvc (default)
nightly-2023-02-21-x86_64-pc-windows-msvc
1.65.0-x86_64-pc-windows-msvc
1.68.0-x86_64-pc-windows-msvc

active toolchain
----------------

1.68.0-x86_64-pc-windows-msvc (overridden by 'D:\rust-toolchain.toml')
rustc 1.68.0 (2c8cc3432 2023-03-06)
```

# rustup toolchain remove nightly-2023-02-21 1.65.0(필요없는거 지우기)

```
PS D:\rust_toolchain_toml> rustup toolchain remove nightly-2023-02-21 1.65.0

info: uninstalling toolchain 'nightly-2023-02-21-x86_64-pc-windows-msvc'
info: toolchain 'nightly-2023-02-21-x86_64-pc-windows-msvc' uninstalled
info: uninstalling toolchain '1.65.0-x86_64-pc-windows-msvc'
info: toolchain '1.65.0-x86_64-pc-windows-msvc' uninstalled

PS D:\rust_toolchain_toml> rustup show
Default host: x86_64-pc-windows-msvc
rustup home:  C:\Users\user\.rustup

installed toolchains
--------------------

stable-x86_64-pc-windows-msvc (default)
1.68.0-x86_64-pc-windows-msvc

active toolchain
----------------

1.68.0-x86_64-pc-windows-msvc (overridden by 'D:\rust-toolchain.toml')
rustc 1.68.0 (2c8cc3432 2023-03-06)


```

- rustup show
```
rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/gy/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
x86_64-apple-ios
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.75.0 (82e1608df 2023-12-21)


```

- rustup target remove

```
rustup target remove x86_64-apple-ios   
info: removing component 'rust-std' for 'x86_64-apple-ios'

```

# crates.io관련 뉴스
- crates.io: API status code changes(240206)
  - https://blog.rust-lang.org/2024/02/06/crates-io-status-codes.html


# WASM 빌드하기(target add)

```bash
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

- https://stackoverflow.com/questions/66180416/rust-nightly-not-installed-please-install-it

- A simple main app(러스트 코드를 WASM 빌드하기_확장자명 .wasm)
- [여기에 정리중](https://github.com/YoungHaKim7/Rust_Tutorial_Full_course/tree/main/17_Rust_WASM_Web_Dev_FullStack#a-simple-main-app)

  - The Hello World example is a standalone Rust application that can be executed by the WasmEdge CLI. The full source code for the Rust main.rs file is as follows. It echoes the command line arguments passed to this program at runtime.

```rs
fn main() {
  let s : &str = "Hello WasmEdge!";
  println!("{}", s);
}
```

- Build the WASM bytecode:

  - `cargo build --target wasm32-wasi --release`

```
cargo build --target wasm32-wasi --release
```

- We will use the wasmedge command to run the program.
```bash
$ wasmedge target/wasm32-wasi/release/hello.wasm
Hello WasmEdg
```

# cargo ructc mir, hir보는법

- 여기에 자세히 정리중
  - [여기에 자세히 정리중 mir, hir, llvm](./07_cargo_rustc_mir_hir_llvm)
  - https://github.com/YoungHaKim7/Rust_Tutorial_Full_course/tree/main/23_Rust_LLVM_IR


- cargo asm
  - https://github.com/gnzlbg/cargo-asm

- ```cargo rustc -- -Zunpretty=mir```

```bash
cargo rustc -- -Zunpretty=mir
   Compiling testrust01 v0.1.0 (D:\young_linux\11111\testrust01)
// WARNING: This output format is intended for human consumers only
// and is subject to change without notice. Knock yourself out.
fn main() -> () {
    let mut _0: ();
    let _1: std::result::Result<ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Di
m<[usize; 2]>>, ndarray::ShapeError>;
    let mut _2: (usize, usize);
    let mut _3: std::vec::Vec<f64>;
    let mut _4: &ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>;
    let _5: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>;

...
...
...

promoted[1] in main: &[&str; 2] = {
    let mut _0: &[&str; 2];
    let mut _1: [&str; 2];

    bb0: {
        _1 = [const "create array 01 bool : ", const "\n"];
        _0 = &_1;
        return;
    }
}
    Finished dev [unoptimized + debuginfo] target(s) in 0.67s
```

- ```cargo rustc -- --emit llvm-ir && cat .\target\debug\deps\testrust01.ll```

```bash

$ cargo rustc -- --emit llvm-ir && cat .\target\debug\deps\testrust01.ll


...
...
...

코드가 겁나게 많다. 

...
!12775 = distinct !DISubprogram(name: "new<ndarray::ArrayBase<ndar
ray::data_repr::OwnedRepr<f64>,ndarray::dimension::dim::Dim<array$
<usize,2> > > >", linkageName: "_ZN4core3fmt2rt8Argument3new17h0fb
bb2618fd00175E", scope: !3030, file: !3029, line: 83, type: !12776
, scopeLine: 83, flags: DIFlagPrototyped, spFlags: DISPFlagLocalTo
Unit | DISPFlagDefinition, unit: !330, templateParams: !3989, decl
aration: !12779, retainedNodes: !12780)
!12776 = !DISubroutineType(types: !12777)
!12777 = !{!3030, !8337, !12778}
!12778 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "enum2$<co
re::result::Result<tuple$<>,core::fmt::Error> > (*)(ref$<ndarray::
ArrayBase<ndarray::data_repr::OwnedRepr<f64>,ndarray::dimension::d
im::Dim<array$<usize,2> > > >,ref_mut$<core::fmt::Formatter>)", ba
seType: !8553, size: 64, align: 64, dwarfAddressSpace: 0)
!12779 = !DISubprogram(name: "new<ndarray::ArrayBase<ndarray::data
_repr::OwnedRepr<f64>,ndarray::dimension::dim::Dim<array$<usize,2>
 > > >", linkageName: "_ZN4core3fmt2rt8Argument3new17h0fbbb2618fd0
0175E", scope: !3030, file: !3029, line: 83, type: !12776, scopeLi
ne: 83, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit, tem
plateParams: !3989)
!12780 = !{!12773, !12781}
!12781 = !DILocalVariable(name: "f", arg: 2, scope: !12774, file:
!3029, line: 83, type: !12778)
!12782 = !DILocation(line: 83, scope: !12774, inlinedAt: !12783)
!12783 = distinct !DILocation(line: 101, scope: !12766, inlinedAt:
 !12772)
!12784 = !DILocation(line: 101, scope: !12766, inlinedAt: !12772)
!12785 = !DILocation(line: 92, scope: !12774, inlinedAt: !12783)
!12786 = !DILocation(line: 102, scope: !12766, inlinedAt: !12772)
!12787 = !DILocation(line: 7, scope: !12733)
!12788 = !DILocation(line: 3, scope: !12727)

```

- cargo hir
  - https://gist.github.com/niklasad1/b838695ef436a0a16d5cd80cf462905f

# Expand macros
`$ cargo rustc --profile=check -- -Zunpretty=expanded`
<br>`$ cargo expand`

- https://github.com/dtolnay/cargo-expand

# Emit asm
`$ cargo rustc -- --emit asm && cat target/debug/deps/project_name-hash.s`
<br>`$ cargo rustc -- --emit asm=asssembly.s`

# Emit llvm-ir
`$ cargo rustc -- --emit llvm-ir && cat target/debug/deps/project_name-hash.ll`
<br>`$ cargo rustc -- --emit llvm-ir=testrust.ll`

# Emit HIR
`$ cargo rustc -- -Zunpretty=hir`

# Emit MIR
`$ cargo rustc -- -Zunpretty=mir`
<br>`$ cargo rustc -- --emit mir=testrust.mir`

# cargo rustc -- --emit dep-info=testrust.depinfo

```
cargo rustc -- --emit dep-info=testrust.depinfo
```

# cargo rustc -- --emit help

```
cargo rustc -- --emit help
   Compiling testrust01 v0.1.0 (D:\young_linux\11111\testrust01)
error: unknown emission type: `help` - expected one of:

`llvm-bc`,
`asm`,
`llvm-ir`,
 `mir`,
`obj`,
`metadata`,
`link`,
`dep-info`
```
# .pdb

- Microsoft released the source code of their PDB formats, so other compiler developers like the LLVM team can implement the PDB format easier.
  - https://github.com/Microsoft/microsoft-pdb/
    - To actually dump the output of a file, just use this:
       - https://github.com/Microsoft/microsoft-pdb/blob/master/cvdump/cvdump.exe


<hr>
