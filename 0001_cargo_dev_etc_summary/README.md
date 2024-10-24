# link

- [`rustfmt.toml`세팅예시_러스트fmt세팅](https://github.com/taiki-e/pin-project/blob/main/.rustfmt.toml)

<hr />


# This Development-cycle in Cargo: 1.82
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

# cargo-tree
- https://doc.rust-lang.org/cargo/commands/cargo-tree.html

<hr />

# This Development-cycle in Cargo: 1.80
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
