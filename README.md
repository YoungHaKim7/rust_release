<p align="center">
    <img width=70px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" />
</p>

# link

- [Version 변경하기 (nightly & 버젼 낮게 만들기)](#강제로-버젼-낮추기-rust-toolchaintoml에서-통제함)
  - [nightly 세팅](./01_Faster_Compilation_Rust)
- [LLVM-downloads다운로드(LLVM)](#llvm-downloads)
- [rust-relese-노트-미리-알아보기](#rust-relese-노트-미리-알아보기)
- [최근-러스트-업데이트-러스트Weekly & etc뉴스](#최근-러스트-업데이트-weeklyetc뉴스)
  - [cratesio관련-뉴스](#cratesio관련-뉴스)
- [러스트-라이브러리-찾기librs--cratesio](#러스트-라이브러리-찾기librs--cratesio)
- [cargo-add-활용법](#cargo-add-활용법)

- [WASM-빌드하기target-add](#wasm-빌드하기target-add)
- [cargo-ructc-mir-hir보는법](#cargo-ructc-mir-hir보는법)
- [cargo asm사용법_07폴더](/07_cargo_rustc_mir_hir_llvm#cargo-asm-기타-등등)

- 새로 생긴 기능들
  - [Debug information is not included in build scripts by default anymore(속도 올리려고 1.69에서 디버그 정보 빠짐 다시 넣는 방법](#debug-information-is-not-included-in-build-scripts-by-default-anymore속도-올리려고-169에서-디버그-정보-빠짐-다시-넣는-방법)
  - [Cargo automatic cache cleaning(rust1.88에 기능 생김)](#cargo-automatic-cache-cleaningrust188에-기능-생김)

<hr />

- Rust doc문서 빨리 찾기
  - [(docs.rs/std)std(Standard Library 문서)](https://doc.rust-lang.org/stable/std/)
  ```
  # local에서 문서 보기 (인터넷 안되는 환경에서 볼수 있음 굿)
  rustup doc
  ``` 
  - [docs.rs/찾고 싶은 crates(라이브러리 치면됨)https://docs.rs/ (crates.io: Rust Package Registry)](https://docs.rs/)

<hr />

- [toml파일 이쁘게 정렬시키자`taplo format Cargo.toml`](#toml-fmt)
  - [외부링크)toml fmt_taplo.tamasfe.dev](https://taplo.tamasfe.dev/cli/installation/binary.html)

<hr />

# 공부할게 많은 버젼별 Rust-clippy[|🔝|](#link)

https://rust-lang.github.io/rust-clippy/

# rustc --version --verbose[|🔝|](#link)

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

# Debug information is not included in build scripts by default anymore(속도 올리려고 1.69에서 디버그 정보 빠짐 다시 넣는 방법)[|🔝|](#link)

- If you want to debug a build script, you can add this snippet to your ```Cargo.toml``` to emit debug information again:
- Cargo.toml https://blog.rust-lang.org/2023/04/20/Rust-1.69.0.html

```toml
[profile.dev.build-override]
debug = true
[profile.release.build-override]
debug = true

# Link-Time Optimizations, or LTOs in short, is that while Rust compiles the code file by file,
[profile.release]
lto = true
```
<br>

<hr>

<hr>

# 강제로 버젼 낮추기 rust-toolchain.toml에서 통제함[|🔝|](#link)

- `rust-toolchain.toml`

```toml
[toolchain]
channel = "1.76"
components = ["rustfmt", "clippy"]
targets = ["wasm32-unknown-unknown"]
```

```toml
# nfo: latest update on 2024-04-04, rust version 1.79.0-nightly (4fd4797c2 2024-04-03)
channel ="nightly-2024-04-04"
```

- 버젼 체크(날짜체크해서 날짜 바꿔주면됨) https://github.com/rust-lang/rust/blob/master/RELEASES.md
  -  https://github.com/rust-lang/rust/blob/master/RELEASES.md


# llvm downloads[|🔝|](#link)

- https://releases.llvm.org/

<hr>

# Rust version Setting[|🔝|](#link)

```bash
rustup update stable
```

- Version control

```bash
rustup default stable
rustup default beta
rustup default nightly
```

<br>

<hr>

# 최근 러스트 업데이트 weekly&etc뉴스[|🔝|](#link)

- 러스트 Weekly
  - https://this-week-in-rust.org/



# crates.io관련 뉴스[|🔝|](#link)
- crates.io: development update(Feb. 5, 2025)
  - https://blog.rust-lang.org/2025/02/05/crates-io-development-update.html

- (Official)Improved API tokens for crates.io
  - https://blog.rust-lang.org/2023/06/23/improved-api-tokens-for-crates-io.html

- crates.io: API status code changes(240206)
  - https://blog.rust-lang.org/2024/02/06/crates-io-status-codes.html

- crates.io: Download changes
  - Mar. 11, 2024 · Tobias Bieniek on behalf of the crates.io team
    - https://blog.rust-lang.org/2024/03/11/crates-io-download-changes.html


<hr>

<br>

# 러스트 라이브러리 찾기(lib.rs & crates.io)[|🔝|](#link)

- 라이브러리 다운로드 수 같은거 볼 수 있음 러스트 라이브러리 정리
  - https://lib.rs/

-  The Rust community’s crate registry
  - https://crates.io/

<hr>

# Rust Relese 노트 미리 알아보기[|🔝|](#link)

- https://releases.rs/docs/
- https://doc.rust-lang.org/nightly/releases.html


# rust_release[|🔝|](#link)

rust release 제일 빨리 알려주는 트위터

[Mara Bos Twitter](https://twitter.com/m_ou_se)

https://twitter.com/m_ou_se

- 이 분의 rust vs cpp concurrency

https://blog.m-ou.se/rust-cpp-concurrency/

<br>

# What Rust is it[|🔝|](#link)

https://www.whatrustisit.com/

<br>

<br>

- Releases 노트 지금까지 나온거 쭉 볼 수 있다.

https://github.com/rust-lang/rust/blob/master/RELEASES.md

<br>

https://github.com/rust-lang/rust/blob/1.64.0/RELEASES.md

<br>

# Rust 러스트 업데이트 내용 미리 보기[|🔝|](#link)

- spoiler book ㅋㅋ

- Spoiler: there are loads.↩︎

Luca Palmieri. Zero To Production In Rust (Kindle Locations 595-596). Kindle Edition.

https://doc.rust-lang.org/nightly/unstable-book/the-unstable-book.html

<hr>

# Rust 공식 Blog[|🔝|](#link)

https://blog.rust-lang.org/

<hr>

# Rust Code 배울게 많다.[|🔝|](#link)
https://github.com/oli-obk

<hr>

<br>

# Asynchronous Programming in Rust \_1.63 부터 이해해야할 Async[|🔝|](#link)

https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html

# cargo add 활용법[|🔝|](#link)
- https://doc.rust-lang.org/cargo/commands/cargo-add.html

- 여러개 넣는거 다른 방법
```bash
cargo add tokio -F rt,rt-multi-thread,macros
```

- 여러개 && 활용해서 하기
```bash
$ cargo add serde -F serde/derive && cargo add tokio -F tokio/full


// short ver.
$ cargo add serde -F derive && cargo add tokio -F full
```

- 알아서 이쁘게 들어간다.

Cargo.toml

```toml
[dependencies]
serde = { version = "1.0.140", features = ["derive"] }
tokio = { version = "1.20.1", features = ["full"] }
```

- cargo add 활용법❤️귀찮은 features넣는 법 -F이게 좋네 ㅋ❤️(Cargo.toml 넣기 귀찮다. ㅋㅋ)

https://economiceco.tistory.com/m/14544

- 다른 cargo add

```bash
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

- `cargo add` EXAMPLES
  - Add regex as a dependency

```bash
cargo add regex
```

- Add trybuild as a dev-dependency
```bash
cargo add --dev trybuild
```

```toml
[dev-dependencies]
regex = "1.11.1"
```

- Add an older version of nom as a dependency
```bash
cargo add nom@5
```

- Add support for serializing data structures to json with derives
```bash
cargo add serde serde_json -F serde/derive
```
- Add windows as a platform specific dependency on cfg(windows)
```bash
cargo add windows --target 'cfg(windows)'
```

<hr>

# ```cargo clean``` 활용법(target을 그냥 지우기 보단 cargo clean을 활용하자)[|🔝|](#link)

```bash
$ cargo clean

     Removed 347 files, 102.8MiB total

```

- target폴더 같은거 찾아서 지워준다 굿 👍Cleans dependencies and build artifacts from your projects.
  - https://github.com/tbillington/kondo


<br>

<hr>

# 중국 사람이 정리한 Rust eBook 번역해서 볼만함[|🔝|](#link)

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

# Rustup show[|🔝|](#link)

```bash
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

# rustup toolchain remove nightly-2023-02-21 1.65.0(필요없는거 지우기)[|🔝|](#link)

```bash
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
```bash
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

```bash
rustup target remove x86_64-apple-ios
info: removing component 'rust-std' for 'x86_64-apple-ios'

```

# WASM 빌드하기(target add)[|🔝|](#link)

```bash
rustup update
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

- rust 1.88부터는 `wasm32-wasip1` 써야함.
  - `wasm32-wasip1`
- https://www.reddit.com/r/rust/comments/1frlvi1/nine_rules_for_running_rust_on_wasm_wasi/


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

```bash
cargo build --target wasm32-wasi --release
```

- We will use the wasmedge command to run the program.
```bash
$ wasmedge target/wasm32-wasi/release/hello.wasm
Hello WasmEdg
```

# cargo ructc mir, hir보는법[|🔝|](#link)

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

# Expand macros[|🔝|](#link)
`$ cargo rustc --profile=check -- -Zunpretty=expanded`
<br>`$ cargo expand`

- https://github.com/dtolnay/cargo-expand

# Emit asm[|🔝|](#link)
`$ cargo rustc -- --emit asm && cat target/debug/deps/project_name-hash.s`
<br>`$ cargo rustc -- --emit asm=asssembly.s`

# Emit llvm-ir[|🔝|](#link)
`$ cargo rustc -- --emit llvm-ir && cat target/debug/deps/project_name-hash.ll`
<br>`$ cargo rustc -- --emit llvm-ir=testrust.ll`

# Emit HIR[|🔝|](#link)
`$ cargo rustc -- -Zunpretty=hir`

# Emit MIR[|🔝|](#link)
`$ cargo rustc -- -Zunpretty=mir`
<br>`$ cargo rustc -- --emit mir=testrust.mir`

# cargo rustc -- --emit dep-info=testrust.depinfo[|🔝|](#link)

```bash
cargo rustc -- --emit dep-info=testrust.depinfo
```

# cargo rustc -- --emit help[|🔝|](#link)

```bash
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
# .pdb[|🔝|](#link)

- Microsoft released the source code of their PDB formats, so other compiler developers like the LLVM team can implement the PDB format easier.
  - https://github.com/Microsoft/microsoft-pdb/
    - To actually dump the output of a file, just use this:
       - https://github.com/Microsoft/microsoft-pdb/blob/master/cvdump/cvdump.exe


<hr>

# rust-analyzer LSP설치하기[|🔝|](#link)
  - https://rust-analyzer.github.io/
```bash
# rustup
# rust-analyzer is available in rustup:

$ rustup component add rust-analyzer
```
<hr>

# rust install후 PATH설정[|🔝|](#link)

```bash
Rust is installed now. Great!

To get started you may need to restart your current shell.
This would reload your PATH environment variable to include
Cargo's bin directory ($HOME/.cargo/bin).

To configure your current shell, you need to source
the corresponding env file under $HOME/.cargo.

This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
source "$HOME/.cargo/env.nu"    # For nushell
```



# toml Fmt[|🔝|](#link)

```bash
taplo format Cargo.toml
```

- `.taplo.toml`

```toml
# See https://taplo.tamasfe.dev/configuration/file.html
# and https://taplo.tamasfe.dev/configuration/formatter-options.html

[formatting]
# Aligning comments with the largest line creates
# diff noise when neighboring lines are changed.
align_comments = false

# Matches how rustfmt formats Rust code
column_width = 100
indent_string = "    "
```


# Cargo automatic cache cleaning(rust1.88에 기능 생김)[|🔝|](#link)
- 자동으로 cache지우는거 안하려면 밑에 처럼 세팅
  - https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/#cargo-automatic-cache-cleaning

- Cargo.toml에 추가
```toml
cache.auto-clean-frequency = "never"
```
