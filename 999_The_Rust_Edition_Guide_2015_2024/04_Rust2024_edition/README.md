# Edition Guide(Rust)

- https://doc.rust-lang.org/nightly/edition-guide/editions/index.html

<hr>

# Rust 프로젝트 목표: 이동 불가 타입과 소멸자 실행 보장 (github.com/rust-lang)
1P by GN⁺ 3달전 | 260817
- 모든 값이 이동되거나 `mem::forget` 으로 소멸자 실행 없이 잊힐 수 있다는 Rust의 전제를 완화해, 타입이 이동과 망각을 거부할 수 있도록 하는 2026~2027년 프로젝트가 승인됨
- 새로운 자동 트레이트 `Move`·`Destruct`·`Forget` 으로 이동, 암묵적 드롭, 망각 가능 여부를 명시하고 각 역량을 계층적으로 부여함
- `!Move`는 자기 참조 타입의 주소가 생존 기간 내내 고정되도록 하며, 이동 불가를 장소의 속성으로 표현하는 `Pin`의 복잡성을 줄이는 대안임
- `!Forget`은 소멸자 실행을 보장해 작업 핸들의 소멸자가 join을 수행하는 **안전한 비동기 범위 지정 `spawn`** 같은 패턴을 가능하게 함
- 컴파일러 MVP와 RFC, Linux 커널 검증, 소멸자 보장 설계에 집중하되 `Future` 변경과 `Pin` 마이그레이션은 별도 프로젝트로 남겨둠
- https://news.hada.io/topic?id=32132

<hr />

# Rust Version
- 🚀 Cutting-edge Features with 'Editions'
- Rust introduces significant changes or new features through ‘Editions’, which are released every 3 years.
  - Rust Edition 2015
    - https://doc.rust-lang.org/edition-guide/rust-2015/index.html
  - Rust Edition 2018
    - (181206) https://blog.rust-lang.org/2018/12/06/Rust-1.31-and-rust-2018.html
      - https://doc.rust-lang.org/edition-guide/rust-2018/index.html
  - Rust Edition 2021
    - https://doc.rust-lang.org/edition-guide/rust-2021/index.html
    - (211021) https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html
      - https://news.hada.io/topic?id=5240
  - Rust2024 Edition (2025년 beta버젼에 출시 예정250109 beta version에서 사용가능)
      - [241127뉴스_rust2024 beta version에 250109 적용예정.](https://blog.rust-lang.org/2024/11/27/Rust-2024-public-testing.html)
        - Stable버젼은  Rust 1.85 on 2025-02-20
          - Rust 2024 will enter the beta channel on 2025-01-09, and will be released to stable Rust with Rust 1.85 on 2025-02-20.
      - (231215) https://blog.rust-lang.org/2023/12/15/2024-Edition-CFP.html
      - (240812) [(러스트 Blog)Rust Project goals for 2024](https://blog.rust-lang.org/2024/08/12/Project-goals.html)
        - Rust 2024 Edition 
          - https://rust-lang.github.io/rust-project-goals/2024h2/Rust-2024-Edition.html
          - [(251028) Project goals for 2025H2 · Niko Matsakis on behalf of the Goals team](https://blog.rust-lang.org/2025/10/28/project-goals-2025h2/)
        - Async
          - https://rust-lang.github.io/rust-project-goals/2024h2/async.html
        - Rust for Linux
          - https://rust-lang.github.io/rust-project-goals/2024h2/rfl_stable.html
            - Rust(2024) 23가지 목표
              - https://rust-lang.github.io/rust-project-goals/2024h2/index.html#project-goals
              - github
                - https://github.com/rust-lang/rust-project-goals

<hr>

# Rust 2024 최신뉴스
- Rust 2024 call for testing Nov. 27, 2024 · Eric Huss & TC on behalf of the Edition 2024 Project Group
  - [Rust 2024 call for testing](https://blog.rust-lang.org/2024/11/27/Rust-2024-public-testing.html)

- Changes to `impl Trait` in Rust 2024( Sept. 5, 2024) | Niko Matsakis 
  - https://blog.rust-lang.org/2024/09/05/impl-trait-capture-rules.html


# Rust 2024 edition | LGR
- https://youtu.be/7ZU8kQNjAcc?si=9a2_F9aWdtSgi5ED

<hr>

# Rust 2024 목표(중요한 3가지 목표)

||Goal 러스트 에디션 2024 핵심 3가지 목표|
|-|-|
|1|Bring the Async Rust experience closer to parity with sync Rust|
|2|Resolve the biggest blockers to Linux building on stable Rust|
|3|Rust 2024 Edition|

- 출처: https://rust-lang.github.io/rust-project-goals/2024h2/flagship.html

<hr>

- Rust 2024 목표(그외에 23가지 목표)

||Goal 러스트 에디션 2024 핵심 목표외 23가지 목표|
|-|-|
|1|"Stabilizable" prototype for expanded const generics|
|2|Administrator-provided reasons for yanked crates|
|3|Assemble project goal slate|
|4|Associated type position impl trait|
|5|Begin resolving cargo-semver-checks blockers for merging into cargo|
|6|Const traits|
|7|Ergonomic ref-counting|
|8|Explore sandboxed build scripts|
|9|Expose experimental LLVM features for automatic differentiation and GPU offloading|
|10|Extend pubgrub to match cargo's dependency resolution|
|11|Implement "merged doctests" to save doctest time|
|12|Make Rustdoc Search easier to learn|
|13|Next-generation trait solver|
|14|Optimizing Clippy & linting|
|15|Patterns of empty types|
|16|Scalable Polonius support on nightly|
|17|Stabilize cargo-script|
|18|Stabilize doc_cfg|
|19|Stabilize parallel front end|
|20|Survey tools suitability for Std safety verification|
|21|Testing infra + contributors for a-mir-formality|
|22|Use annotate-snippets for rustc diagnostic output|
|23|User-wide build cache|



- 출처 : https://rust-lang.github.io/rust-project-goals/2024h2/accepted.html

<hr>

# Async Rust roadmap

|Year|Language|
|-|-|
|2019| Async fns|
|2019-2022| Ecosystem development|
|2023| Async fn in traits|
|2024| Async closures, generators....|

- Standard way to write async Rust that...비동기 Rust를 작성하는 표준 방법은...
  - lets you gracefully handle cancellation and streams
    - 취소 및 스트림을 우아하게 처리할 수 있습니다
  - supports a rich, interopable ecosystem of middleware, logging,etc
    - 미들웨어, 로깅 등의 풍부하고 상호 운용 가능한 에코시스템을 지원합니다
  - works everywhere, from embedded to servers
    - 내장된 서버에서 서버까지 모든 곳에서 작동합니다
  - is easy to learn, well documented, and free of footguns
    - 배우기 쉽고, 잘 문서화되어 있으며, 발총이 없습니다 
      - C++은 코드를 잘못 만들면 내 발에 총을 쏜다는걸 이야기하는듯 그래서 코드가 터져버리는 ㅋㅋㅋ

- Stabilizing async fn in traits in 2023(async fn 안정화했다는 러스트 블로그 글)
  - May 3, 2023 · Niko Matsakis and Tyler Mandry on behalf of The Rust Async Working Group
  - https://blog.rust-lang.org/inside-rust/2023/05/03/stabilizing-async-fn-in-trait.html
  - 관련 Reddit글
    - https://www.reddit.com/r/rust/comments/136o73k/stabilizing_async_fn_in_traits_in_2023_inside/

- Rust Async GAT관련
  - https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html
    - Try using GAT to improve Future's failed: How to declare the life cycle?
      -  https://stackoverflow.com/questions/74441311/try-using-gat-to-improve-futures-failed-how-to-declare-the-life-cycle

<hr>

# Rust Project goals for 2024
Aug. 12, 2024 · Niko Matsakis on behalf of Leadership Council 
- https://blog.rust-lang.org/2024/08/12/Project-goals.html

<hr>

# 2024 Edition Update
Mar. 22, 2024 · Eric Huss on behalf of Edition 2024 Project Group
- https://blog.rust-lang.org/inside-rust/2024/03/22/2024-edition-update.html

# Rust 2024맛보기

- https://nikomatsakis.github.io/rustnation-24/

  - https://www.youtube.com/live/RQSZ3wLsjNM?si=GZ9UoAfQ7b3s-JOg

# 올해 하반기에 나올 Rust 2024에 대해
- Rust 2024 style edition

- This style guide describes the Rust 2024 style edition. The Rust 2024 style edition is currently nightly-only and may change before the release of Rust 2024.

- For a full history of changes in the Rust 2024 style edition, see the git history of the style guide. Notable changes in the Rust 2024 style edition include:

- 114764 As the last member of a delimited expression, delimited expressions are generally combinable, regardless of the number of members. Previously only applied with exactly one member (except for closures with explicit blocks).
Miscellaneous rustfmt bugfixes.
Use version-sort (sort x8, x16, x32, x64, x128 in that order).
Change "ASCIIbetical" sort to Unicode-aware "non-lowercase before lowercase".

- https://github.com/rust-lang/rust/pull/114764

# Hack without fear

- We had a lot of ideas to improve performance, but we were hesitant to introduce them into critical systems given the risk of subtle bugs.
  - 성능 향상을 위한 아이디어는 많았지만, 미묘한 버그의 위험을 고려하여 중요한 시스템에 도입하는 것을 주저했습니다.
- With Rust, we achieved up to double-digit percentage performance improvements. Rust’s type system provides a structure which we used to safely optimize our code.
  - Rust를 통해 최대 두 자릿수의 성능 향상을 달성했습니다. Rust의 유형 시스템은 코드를 안전하게 최적화하는 데 사용했던 구조를 제공합니다.

# Image(Rust crates.io)

https://crates.io/crates/image

# Blazingly 🔥 fast 🚀 memory vulnerabilities, written in 100% safe Rust. 🦀 

https://github.com/Speykious/cve-rs


# Hack without fear (Seth Markle, Senior Principal Engineer, S3)

- We had a lot of ideas to improve performance, but we were hesitant to introduce them into critical systems given the risk of subtle bugs.
  - 성능 향상을 위한 아이디어는 많았지만, 미묘한 버그의 위험을 고려하여 중요한 시스템에 도입하는 것을 주저했습니다.
- With Rust, we achieved up to double-digit percentage performance improvements. Rust’s type system provides a structure which we used to safely optimize our code.
  - Rust를 통해 최대 두 자릿수의 성능 향상을 달성했습니다. Rust의 유형 시스템은 코드를 안전하게 최적화하는 데 사용했던 구조를 제공합니다.

# Changes to Rust's WASI targets

Apr. 9, 2024 · Yosh Wuyts

- https://blog.rust-lang.org/2024/04/09/updates-to-rusts-wasi-targets.html

<table>
<thead>
<tr>
<th>date</th>
<th>Rust Stable</th>
<th>Rust Beta</th>
<th>Rust Nightly</th>
<th>Notes</th>
</tr>
</thead>
<tbody>
<tr>
<td>2024-02-08</td>
<td>1.76</td>
<td>1.77</td>
<td>1.78</td>
<td><code>wasm32-wasip1</code> available on nightly</td>
</tr>
<tr>
<td>2024-03-21</td>
<td>1.77</td>
<td>1.78</td>
<td>1.79</td>
<td><code>wasm32-wasip1</code> available on beta</td>
</tr>
<tr>
<td>2024-05-02</td>
<td>1.78</td>
<td>1.79</td>
<td>1.80</td>
<td><code>wasm32-wasip1</code> available on stable</td>
</tr>
<tr>
<td>2024-06-13</td>
<td>1.79</td>
<td>1.80</td>
<td>1.81</td>
<td>warn if <code>wasm32-wasi</code> is used on nightly</td>
</tr>
<tr>
<td>2024-07-25</td>
<td>1.80</td>
<td>1.81</td>
<td>1.82</td>
<td>warn if <code>wasm32-wasi</code> is used on beta</td>
</tr>
<tr>
<td>2024-09-05</td>
<td>1.81</td>
<td>1.82</td>
<td>1.83</td>
<td>warn if <code>wasm32-wasi</code> is used on stable</td>
</tr>
<tr>
<td>2024-10-17</td>
<td>1.82</td>
<td>1.83</td>
<td>1.84</td>
<td><code>wasm32-wasi</code> unavailable on nightly</td>
</tr>
<tr>
<td>2024-11-28</td>
<td>1.83</td>
<td>1.84</td>
<td>1.85</td>
<td><code>wasm32-wasi</code> unavailable on beta</td>
</tr>
<tr>
<td>2025-01-09</td>
<td>1.84</td>
<td>1.85</td>
<td>1.86</td>
<td><code>wasm32-wasi</code> unavailable on stable</td>
</tr>
</tbody>
</table>

- Conclusion
  - In this post we've discussed the upcoming updates to Rust's WASI targets. Come Rust 1.78 the wasm32-wasip1 (tier 2) and wasm32-wasip2 (tier 3) targets will be added. In Rust 1.81 we will begin warning if wasm32-wasi is being used. And in Rust 1.84, the existing wasm32-wasi target will be removed. This will free up wasm32-wasi to eventually be used for a WASI 1.0 target. Users will have 8 months to switch to the new target name when they update their Rust toolchains.
  - The wasm32-wasip2 target marks the start of native support for WASI 0.2. In order to target it today from Rust, people are encouraged to use cargo-component tool instead. The plan is to eventually graduate wasm32-wasip2 to a tier-2 target, at which point cargo-component will be upgraded to support it natively instead.
  - With WASI 0.2 finally stable, it's an exciting time for WebAssembly development. We're happy for Rust to begin implementing native support for WASI 0.2, and we're excited about what this will enable people to build.

- 결론
  - 이 게시물에서 우리는 러스트의 WASI 대상에 대한 다가오는 업데이트에 대해 논의했습니다. 러스트 1.78로 와스프1(2단계) 및 와스프2(3단계) 대상이 추가됩니다. 러스트 1.81에서는 wasm32-와스프2(3단계) 대상이 사용되고 있는지 경고하기 시작합니다. 그리고 러스트 1.84에서는 기존의 wasm32-와스프 대상이 제거됩니다. 이를 통해 wasm32-와스프는 최종적으로 WASI 1.0 대상에 사용될 수 있습니다. 사용자는 러스트 툴체인을 업데이트할 때 새로운 대상 이름으로 전환할 수 있는 8개월의 시간을 갖게 됩니다.
  - wasm32-wasip2 타겟은 WASI 0.2에 대한 네이티브 지원의 시작을 나타냅니다. 오늘날 러스트에서 타겟으로 삼기 위해서, 사람들은 대신 카고 컴포넌트 툴을 사용하는 것이 좋습니다. 이 계획은 최종적으로 wasm32-wasip2를 Tier-2 타겟으로 졸업시키는 것이며, 이 시점에서 카고 컴포넌트는 대신 네이티브 지원을 위해 업그레이드 될 것입니다.
  - WASI 0.2가 마침내 안정화됨에 따라, WebAssembly 개발을 위한 흥미진진한 시기가 되었습니다. Rust가 WASI 0.2에 대한 기본 지원을 구현하기 시작한 것을 기쁘게 생각하며, 이를 통해 사람들이 무엇을 구축할 수 있을지 기대됩니다.
