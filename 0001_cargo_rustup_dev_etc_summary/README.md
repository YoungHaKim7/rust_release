# link

- [`rustfmt.toml`세팅예시_러스트fmt세팅](https://github.com/taiki-e/pin-project/blob/main/.rustfmt.toml)
- [gccrs: Rust를 위한 대체 컴파일러_LLVM없이도 된다 신기..](#gccrs-rust를-위한-대체-컴파일러)

- [cargo-tree](#cargo-tree)

<hr />

- This Development-cycle in Cargo
  - [This Development-cycle in Cargo: 1.83](#this-development-cycle-in-cargo-183)
  - [This Development-cycle in Cargo: 1.82](#this-development-cycle-in-cargo-182)
  - [This Development-cycle in Cargo: 1.80](#this-development-cycle-in-cargo-180-1)

<hr />

- rustup
  - [Announcing rustup 1.28.1 | Mar. 4, 2025 · The Rustup Team](https://blog.rust-lang.org/2025/03/04/Rustup-1.28.1.html)

<hr />

# Shell completions
- Update from 1.82
  - With clap's new completion system integrated into nightly Cargo in [#14493](https://github.com/rust-lang/cargo/pull/14493), shannmu added custom completers in an effort to get parity with the hand-maintained completion scripts, including:
    - cargo 명령어 https://github.com/rust-lang/cargo/tree/master/tests/testsuite

<ul>
<li><code>--example &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14531">#14531</a>)</li>
<li><code>--bench &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14532">#14532</a>)</li>
<li><code>--bin &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14533">#14533</a>)</li>
<li><code>--test &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14548">#14548</a>)</li>
<li><code>--target &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14535">#14535</a>)</li>
<li><code>--registry &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14656">#14656</a>)</li>
<li><code>cargo update &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14552">#14552</a>)</li>
<li><code>cargo uninstall &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14534">#14534</a>)</li>
<li><code>-Z &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14536">#14536</a>)</li>
<li><code>cargo help &lt;TAB&gt;</code> (<a href="https://github.com/rust-lang/cargo/pull/14557">#14557</a>)</li>
</ul>

<hr />

# This Development-cycle in Cargo: 1.83[|🔝|](#link)
- Oct. 31, 2024 · Ed Page on behalf of The Cargo Team
- https://blog.rust-lang.org/inside-rust/2024/10/31/this-development-cycle-in-cargo-1.83.html

<hr />

# This Development-cycle in Cargo: 1.82[|🔝|](#link)
- Oct. 1, 2024 · Ed Page on behalf of The Cargo Team
- https://blog.rust-lang.org/inside-rust/2024/10/01/this-development-cycle-in-cargo-1.82.html
- This Development-cycle in Cargo: 1.82
  - This is a summary of what has been happening around Cargo development for the merge window for Rust 1.82.(다음은 러스트 1.82의 병합 창을 위한 화물 개발과 관련하여 어떤 일이 일어나고 있는지 요약한 것입니다)
    - Plugin of the cycle(주기의 플러그인)
      - Implementation
      - cargo info
      - Shell completions(쉘 completions)
      - MSRV-aware Cargo(MSRV 인식 cargo)
      - cargo publish --workspace
      - cargo::error build script directive
      - cargo update --precise <prerelease>
      - Snapshot testing(스냅샷 테스트)
    - Design discussions(디자인 토론)
      - time
      - Build probes(프로브 구축)
      - Detecting unused dependencies(사용하지 않는 종속성 탐지)
      - --all-targets and doctests
      - target-dir and artifact-dir
      - cargo update --save
    - Misc(miscellaneous 기타등등.. 이런저런것 을 의미)
    - Focus areas without progress(진전이 없는 영역에 집중)

<hr />

# cargo-tree[|🔝|](#link)
- https://doc.rust-lang.org/cargo/commands/cargo-tree.html

<hr />

# This Development-cycle in Cargo: 1.80[|🔝|](#link)
- June 19, 2024 · Ed Page on behalf of The Cargo Team
  - https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html


<hr>

<h1><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#this-development-cycle-in-cargo-180" aria-hidden="true" class="anchor" id="this-development-cycle-in-cargo-180"></a>This Development-cycle in Cargo: 1.80</h1>
<p>This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.80.</p>
<!-- time period: 2024-05-03 through 2024-06-13 -->
<ul>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#plugin-of-the-cycle">Plugin of the cycle</a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#implementation">Implementation</a>
<ul>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#-zcheck-cfg"><code>-Zcheck-cfg</code></a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#user-controlled-cargo-diagnostics">User-controlled cargo diagnostics</a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#-ztrim-paths"><code>-Ztrim-paths</code></a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#msrv-aware-cargo">MSRV-aware Cargo</a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#removing-implicit-features">Removing implicit features</a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#normalizing-published-manifest-files">Normalizing published manifest files</a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#merging-cargo-upgrade-into-cargo-update">Merging <code>cargo upgrade</code> into <code>cargo update</code></a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#crate-provenance"><code>.crate</code> provenance</a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#cargo-publish---workspace"><code>cargo publish --workspace</code></a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#snapshot-testing">Snapshot testing</a></li>
</ul>
</li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#design-discussions">Design discussions</a>
<ul>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#rfc-triage">RFC triage</a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#custom-test-harnesses-and-panic--abort">Custom test harnesses and <code>panic = &quot;abort&quot;</code></a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#short-hand-manifest-syntaxes">Short-hand manifest syntaxes</a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#leaky-abstractions-of-rustc">Leaky abstractions of rustc</a></li>
</ul>
</li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#misc">Misc</a></li>
<li><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#focus-areas-without-progress">Focus areas without progress</a></li>
</ul>
<h2><a href="https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html#plugin-of-the-cycle" aria-hidden="true" class="anchor" id="plugin-of-the-cycle"></a>Plugin of the cycle</h2>

<hr>

<hr>



<hr />

# **[gccrs: Rust를 위한 대체 컴파일러](<https://news.hada.io/topic?id=17681&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**[|🔝|](#link)
- `gccrs`는 GCC 프로젝트의 일환으로 개발 중인 대체 Rust 컴파일러입니다.  
- 이 프로젝트는 GNU 컴파일러 컬렉션 내에서 Rust를 지원하는 것을 목표로 하며, `rustc`와 동일한 동작을 목표로 합니다.  
- 주요 목표는 특히 LLVM이 지원하지 않는 플랫폼에서 Rust를 컴파일할 수 있는 대안을 제공하는 것입니다.  
...
