# stabilize the next-generation trait solver
- https://github.com/rust-lang/goals/issues/113

# (260822)▲Rust nightly, 차세대 트레이트 솔버 기본 활성화 (blog.rust-lang.org)
1P by GN⁺ 2달전 | 
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
