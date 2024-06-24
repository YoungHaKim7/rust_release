# This Development-cycle in Cargo: 1.80
- June 19, 2024 · Ed Page on behalf of The Cargo Team
  - https://blog.rust-lang.org/inside-rust/2024/06/19/this-development-cycle-in-cargo-1.80.html


<hr>

<h1><a href="#this-development-cycle-in-cargo-180" aria-hidden="true" class="anchor" id="this-development-cycle-in-cargo-180"></a>This Development-cycle in Cargo: 1.80</h1>
<p>This is a summary of what has been happening around Cargo development for the last 6 weeks which is approximately the merge window for Rust 1.80.</p>
<!-- time period: 2024-05-03 through 2024-06-13 -->
<ul>
<li><a href="#plugin-of-the-cycle">Plugin of the cycle</a></li>
<li><a href="#implementation">Implementation</a>
<ul>
<li><a href="#-zcheck-cfg"><code>-Zcheck-cfg</code></a></li>
<li><a href="#user-controlled-cargo-diagnostics">User-controlled cargo diagnostics</a></li>
<li><a href="#-ztrim-paths"><code>-Ztrim-paths</code></a></li>
<li><a href="#msrv-aware-cargo">MSRV-aware Cargo</a></li>
<li><a href="#removing-implicit-features">Removing implicit features</a></li>
<li><a href="#normalizing-published-manifest-files">Normalizing published manifest files</a></li>
<li><a href="#merging-cargo-upgrade-into-cargo-update">Merging <code>cargo upgrade</code> into <code>cargo update</code></a></li>
<li><a href="#crate-provenance"><code>.crate</code> provenance</a></li>
<li><a href="#cargo-publish---workspace"><code>cargo publish --workspace</code></a></li>
<li><a href="#snapshot-testing">Snapshot testing</a></li>
</ul>
</li>
<li><a href="#design-discussions">Design discussions</a>
<ul>
<li><a href="#rfc-triage">RFC triage</a></li>
<li><a href="#custom-test-harnesses-and-panic--abort">Custom test harnesses and <code>panic = &quot;abort&quot;</code></a></li>
<li><a href="#short-hand-manifest-syntaxes">Short-hand manifest syntaxes</a></li>
<li><a href="#leaky-abstractions-of-rustc">Leaky abstractions of rustc</a></li>
</ul>
</li>
<li><a href="#misc">Misc</a></li>
<li><a href="#focus-areas-without-progress">Focus areas without progress</a></li>
</ul>
<h2><a href="#plugin-of-the-cycle" aria-hidden="true" class="anchor" id="plugin-of-the-cycle"></a>Plugin of the cycle</h2>

<hr>
