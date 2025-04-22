# gen-blocks
- https://doc.rust-lang.org/edition-guide/rust-2024/gen-keyword.html

- https://rust-lang.github.io/rfcs/3513-gen-blocks.html

# gen사용법
- gen초기 아이디어
  - https://crates.io/crates/genawaiter
    - https://docs.rs/genawaiter/0.99.1/genawaiter/

- https://github.com/rust-lang/rust/issues/123731#issuecomment-2053953399

- Throwing some options out there:

  - stick with `gen`, break lots of crates
  - use `gener` (personal opinion: it's not clear what it means, tho we'll learn)
  - `impl Iterator {}`, kinda long but very extensible to builtin syntax for other traits (`impl Generator`, `impl Try`), conflicts with inherent impls
  - `iter/iterator`, breaks too much code
  - `something!`, make it a builtin macro, so local definitions and imports override it
  - `generator`, wordy, but explicit. We don't need to stick with the "keywords must be short" rule and it likely has no problematic breakage
  - use an attribute: `#[gen] {}`, a bit odd, but very extensible
  - ask T-lang to revisit `gen` becoming a contextual keyword
  - `yields {}`, may be too confusing with yield
  - prefix with some existing keyword (`move gen {}` could signal the difference to `static gen {}` which would not create iterator impls, but only some so-far-non-existent `PinnedIterator` trait). Not really necessary (we can just infer which traits to implement for `gen` blocks, though this is another nail in the coffin of `gen fn`). Also it's very confusing as `move gen` is different from `gen move` (move all variables instead of letting


# Gen keyword

- With generator blocks and closures there is a distinction between the logical return type and the logical yield type. While `Iterator` cannot express any return type other than `()`, we have to anticipate that at least syntactically generator blocks and closures will want to support distinguishing between the type yielded and the type returned. Using the bare `gen {}` syntax currently being proposed this would result in the following notations for generator blocks and closures.
  - 생성기 블록과 클로저에는 논리적 반환 유형과 논리적 반환 유형 사이에 차이가 있습니다. 'Iterator'는 '()' 이외의 반환 유형을 표현할 수 없지만, 최소한 구문적으로 생성기 블록과 클로저가 생성된 유형과 반환된 유형을 구별하는 데 도움이 될 것으로 예상해야 합니다. 현재 제안되고 있는 기본 'gen {}' 구문을 사용하면 생성기 블록과 클로저에 대해 다음과 같은 표기법을 얻을 수 있습니다.

```rs
// block, inferred
let iter = gen { yield 12; }

// block, yield type annotated
let iter: impl Iterator<Item = u32> gen { yield 12; }

// closure, inferred
let iter = gen || { yield 12; }

// closure, yield type annotated
let iter: impl Iterator<Item = u32> = gen || { yield 12; }

// closure, return type annotated
let iter = gen || -> () { yield 12; }the generator be movable) and implies the existence of `move gen move`.
```

- We're not yet making a decision on generator functions, though they are available on unstable. But it's worth pointing out that much of the same rationale I've listed here applies to those too, but unlike generator blocks and closures, generator functions must always spell out their yield type - they cannot just rely on inference the way blocks and closures do.

If we were to use `-> T` to mean "yields type T", that would make it impossible for generators notation to express a different return type, which as an implication means that generalized coroutines would need to use a different keyword with different semantics. This directly means that we'd need to be comfortable having e.g. both a `gen || -> T {}` and` coro || -> T {}` where `-> T` means different things. I believe that would be bad.

  - 아직 발전기 함수에 대한 결정을 내리지는 않았지만 불안정한 함수에 대해서는 사용할 수 있습니다. 하지만 여기서 나열한 것과 같은 이유의 대부분이 이러한 이유에도 적용된다는 점을 지적할 필요가 있습니다. 하지만 발전기 블록과 폐쇄와 달리 발전기 함수는 항상 항복 유형을 명시해야 하므로 블록과 폐쇄와 같은 방식으로 추론에만 의존할 수는 없습니다.

만약 우리가 -> T를 "수익률 유형 T"를 의미하기 위해 사용한다면, 이는 생성자 표기법이 다른 반환 유형을 표현하는 것을 불가능하게 만들 것입니다. 이는 일반화된 코루틴이 다른 의미를 가진 다른 키워드를 사용해야 한다는 것을 의미합니다. 이는 직접적으로 우리가 gen || -> T {}와 coro || -> T {}를 모두 사용하는 것에 익숙해야 한다는 것을 의미합니다. 여기서 -> T는 다른 의미를 갖습니다. 저는 그것이 나쁘다고 생각합니다.

# Yields keyword

- On the RFC PR I floated the idea of using yields as the notation for bare generator blocks and closures. I wasn't entirely sure of that idea at the time, but I believe it has merit spelling out the example in full, so let's do that here:
  - RFC PR에서 저는 베어 제너레이터 블록과 클로저의 표기법으로 수율을 사용하는 아이디어를 떠올렸습니다. 당시에는 그 아이디어에 대해 완전히 확신하지는 못했지만, 예제를 완전히 철자할 가치가 있다고 생각합니다. 그래서 여기서 해봅시다:

```rs
// block, inferred
let iter = yields { yield 12; }

// block, yield type annotated
let iter = yields u32 { yield 12; }

// closure, inferred
let iter = yields || { yield 12; }

// closure, yield type annotated
// NOTE: must be unambiguous with `|| yields {}`
let iter = yields u32 || { yield 12; }

// closure, return type annotated
// NOTE: must be unambiguous with `|| yields {}`
let iter = yields || -> () { yield 12; }

// closure, yields + return type annotated
// NOTE: must be unambiguous with `|| yields {}`
let iter = yields u32 || -> () { yield 12; }
```

- As you can see here, the fact that we're supporting both generator blocks and generator functions ends up leading to syntactic ambiguity. If we were to adopt `|| yields {}` as the notation, it would be unclear whether this was a function returning an generator, or a generator function. These are distinct concepts, and I believe it's worth keeping these separate.

To resolve this ambiguity we'd need to put the `yields` keyword before the `||` argument list, which seems a little strange. In effect we're declaring a function output in a position where normally we'd describe function inputs. If we were to consider the syntax for generator functions, it's unlikely we'd want to write: `fn yields u32 () {}` or similar. We almost certainly would prefer any form of `yields` to come after the `()` argument list.

- 여기에서 볼 수 있듯이 생성기 블록과 생성기 함수를 모두 지원한다는 사실은 구문적 모호성으로 이어집니다. ||을 표기법으로 채택한다면 이것이 생성기를 반환하는 함수인지 생성기 함수인지 불분명할 것입니다. 이 개념들은 서로 다른 개념이며, 이를 분리하여 유지할 가치가 있다고 생각합니다.

이 모호성을 해결하려면 || 인수 목록 앞에 수율 키워드를 넣어야 하는데, 이는 조금 이상해 보입니다. 사실상 함수 출력을 일반적으로 함수 입력을 설명하는 위치에 선언하는 것과 같습니다. 생성 함수의 구문을 고려한다면 다음과 같이 쓰고 싶을 가능성은 낮습니다: fn 수율 u32 () {} 또는 이와 유사합니다. 거의 확실하게 () 인수 목록 뒤에 어떤 형태의 수율이 오는 것을 선호합니다.

# Gen + Yields

- One way to still reap the benefits of a yields notation without the parsing ambiguity would be for generator blocks and closures to always require gen to signal it's a generator, but permit yields as an explicit notation after the argument list to explicitly declare the yield type.
  - 구문 분석 모호성 없이도 수율 표기법의 이점을 계속 얻을 수 있는 한 가지 방법은 생성기 블록과 클로저가 항상 생성기임을 나타내는 gen을 요구하는 것이지만, 인수 목록 뒤에 명시적인 표기법으로 수율 유형을 명시적으로 선언할 수 있도록 허용하는 것입니다.

```rs
// block, inferred
let iter = gen { yield 12; }

// block, yield type annotated
let iter = gen yields u32 { yield 12; }

// closure, inferred
let iter = gen || { yield 12; }

// closure, yield type annotated
let iter = gen || yields u32 { yield 12; }

// closure, return type annotated
let iter = gen || -> () { yield 12; }

// closure, yields + return type annotated
let iter = gen || yields u32 -> () { yield 12; } // either…
let iter = gen || -> () yields u32 { yield 12; } // …or
```

While a little more verbose than the bare yields example, I believe this makes up for it by being unambiguous. It does introduce a small wart in the sense that generator functions don't require being annotated, so there may be a loss of parity between the two notations.

```rs
// Having a `gen fn` notation here would be
// unlikely to carry its weight here since it
// does not resolve any parser ambiguity.
gen fn iter() yields u32 -> () {} // either…
gen fn iter() -> () yields u32 {} // …or

// The `yields` keyword on its own here is enough
// to indicate this is a generator function.
fn iter() yields u32 -> () {} // either…
fn iter() -> () yields u32 {} // …or
```

- Having gone through this exercise, I believe that of the three options this option carries the least worst tradeoffs.

# Conclusion
- https://github.com/rust-lang/rust/issues/123731#issuecomment-2053953399
- I think `gen {}` as a keyword here is fine, as long as we eventually pair it up with some form of `yields` notation to be able to annotate the yield type. I mostly wanted to make sure it was spelled out why `yields` as a keyword is necessary, and why introducing it without an additional prefix marker for closures and blocks would lead to ambiguity in the parser.

- Basically that's a long way to say that I find myself agreeing with what @tmandry expressed earlier.

<hr />

# 다른 설명
- I don't think `async fn` was a mistake. The consistency among keywords that affect a function's syntax is definitely useful, at least documentation-wise (`const`, `async`, `unsafe`, etc.). In my opinion, gen affecting the entire function's control flow should keep consistent with keywords like async.

- It feels natural to read `-> T` as, "this produces T", but it is different from the usual meaning. I think this syntax looks confusing when `return` ends the function without a value.
  It would also make it possible to confuse functions returning single types versus iterator types, just in terms of skimming function return types in documentation or source code. I would prefer anything else like:
  - `gen fn() -> yield i32` - Could retain -> T as "produces T". e.g. coroutine fn() -> yield i32, return i32.
  - gen fn() yield i32 - Uses an existing keyword. Slightly awkward to read.
  - gen fn() yields i32 - Requires a new keyword. Reads very nicely.
  - gen fn() ^ i32 - Upwards arrow (yields to the caller, may resume later).
  - gen fn() >> i32 - Two arrows (produces multiple values).
  - gen fn() => i32 - Is -> meaningfully distinct from =>? I suppose -> is usually for types and => is usually for values.

I can't wait for generator functions, I love generator functions.

- - 저는 비동기 fn이 실수였다고 생각하지 않습니다. 함수의 구문에 영향을 미치는 키워드 간의 일관성은 적어도 문서 측면에서는 유용합니다(const, 비동기, 안전하지 않음 등). 제 생각에는 함수의 전체 제어 흐름에 영향을 미치는 gen은 비동기와 같은 키워드와 일관성을 유지해야 한다고 생각합니다.

- -> T를 "이것이 T를 생성한다"고 읽는 것은 자연스럽게 느껴지지만, 일반적인 의미와는 다릅니다. 반환이 함수를 값 없이 끝낼 때 이 구문이 혼란스러워 보이는 것 같아요.
또한 문서나 소스 코드에서 함수 반환 유형을 스키밍하는 것만으로도 단일 유형을 반환하는 함수와 반복자 유형을 혼동하는 것이 가능할 것입니다. 저는 다음과 같은 다른 것을 선호합니다:
- gen fn() -> 수율 i32 - -> T를 "생산 T"로 유지할 수 있습니다. 예를 들어, coroutine fn() -> 수율 i32, 반환 i32.
- gen fn() 수율 i32 - 기존 키워드를 사용합니다. 읽기가 약간 어색합니다.
- gen fn()은 i32를 생성합니다 - 새로운 키워드가 필요합니다. 매우 잘 읽힙니다.
- gen fn() ^ i32 - 위쪽 화살표(호출자에게 양보하며, 나중에 재개될 수 있음).
- gen fn() >> i32 - 두 개의 화살표(여러 값 생성).
- gen fn () => i32 - ->는 =>와 의미 있게 구별되나요? ->는 보통 유형을 의미하고 =>는 보통 값을 의미합니다.

발전기 기능이 너무 기대돼요. 발전기 기능이 정말 마음에 들어요.
