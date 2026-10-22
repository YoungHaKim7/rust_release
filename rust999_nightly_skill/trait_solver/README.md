# stabilize the next-generation trait solver
- https://github.com/rust-lang/goals/issues/113

# (260822)▲Rust nightly, 차세대 트레이트 솔버 기본 활성화 (blog.rust-lang.org)
1P by GN⁺ 2달전 | 
- https://news.hada.io/topic?id=32752
- 약 4년간 개발한 차세대 트레이트 솔버가 안정화를 앞두고 nightly에서 기본 활성화되며, where 절 증명과 연관 타입 정규화 등 Rust 컴파일러의 핵심 처리 방식을 전면 교체함
- 기존 구현을 제거하면 `Type Alias Impl Trait·Return Type Notation` 안정화, `Move`·`Forget` 같은 암시적 기본 트레이트 경계 추가, 남은 타입 시스템 건전성 문제 해결이 가능해짐
- 상위 20,000개 crates.io 크레이트는 거의 모두 기존 솔버와 비슷한 컴파일 성능을 보였으며, `datafusion`은 8배 넘게 빨라지고 기존 솔버에서 멈추던 타입 시스템 기반 체스 구현은 약 1분 만에 처리됨
- `rustup update nightly`로 직접 테스트해 문제를 신고할 수 있으며, 호환성 문제가 생기면 `-Znext-solver=coherence`로 새 솔버를 비활성화할 수 있음

### 기본 활성화와 안정화 계획
- 약 4년간 개발한 차세대 트레이트 솔버가 안정화에 가까워졌으며, 남은 문제를 찾기 위해 nightly에서 기본 활성화됨
- 향후 몇 달 안에 안정화할 계획으로, Rust 초기 출시 이후 컴파일러에 적용되는 가장 큰 단일 변경임
- 컴파일러 내부의 where 절 증명과 연관 타입 정규화 등을 담당하던 기존 방식을 완전히 교체함
- 새 구현은 이미 확인된 것만 200개 넘는 GitHub 이슈를 수정했으며, 컴파일 시간에도 큰 영향을 줌
- nightly로 개발한 코드는 개발자가 모르는 사이 새 솔버에서만 지원하는 동작에 의존할 수 있음
- 변경 규모가 커 적지 않은 호환성 문제가 발생하지만, 대부분 타입 추론 개선이나 바람직하지 않은 기존 동작 제거에서 비롯됨

### `impl Trait` 처리 방식의 변화
- 반환 위치 `impl Trait`(RPIT), 불안정 기능인 **Type Alias Impl Trait(TAIT)** 및 **Return Type Notation(RTN)** 등 불투명 타입의 처리 방식이 거의 완전히 바뀜

```rs
fn foo(b: bool) -> impl Sized {
    if b {
        foo(false) + 1
    } else {
        0
    }
}
```

### 고차 순위 타입의 연관 타입 처리
가장 영향이 큰 변화는 `for<'a>` 바인더의 수명처럼 바인딩된 변수를 참조하는 연관 타입의 처리 방식임
예: `for<'a> fn(<T as Trait>::Assoc<'a>)`
많은 사용자가 이런 타입을 직접 다루지는 않지만, 널리 쓰이는 크레이트 일부가 이를 사용함
기존 코드의 잘못된 타입 추론이 제거되면서 bevy와 minijinja가 영향을 받음
기존 솔버가 불필요하게 거부하던 코드도 컴파일할 수 있게 됨
아래 예제에서는 기존 구현이 `impls`의 where 경계를 증명하지 못했음

```rs
trait OtherTrait {
    type Assoc<'a>;
}
impl OtherTrait for u32 {
    type Assoc<'a> = &'a u32;
}

trait Trait {}
impl<T: OtherTrait> Trait for (T, for<'a> fn(<T as OtherTrait>::Assoc<'a>)) {}

fn impls<T: Trait>() {}

fn main() {
    impls::<(u32, for<'a> fn(&'a u32))>();
}
```
