# Edition Guide(Rust)

- https://doc.rust-lang.org/nightly/edition-guide/editions/index.html

<hr>

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
  - Rust2024 Edition (2024년 하반기 출시 예정)
      - (231215) https://blog.rust-lang.org/2023/12/15/2024-Edition-CFP.html

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
