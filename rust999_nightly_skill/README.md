# nightly_skill

- if let while chains
  - https://rust-trends.com/posts/rust-s-if-let-while-chains/

# Rust Edition 2024 -> 🦸‍♂️  다음 준비하는 프로젝트들 🔥 🚀 
- Project goals update — November 2025
Dec. 16, 2025 · Tomas Sedovic on behalf of the Goals team
- https://blog.rust-lang.org/2025/12/16/Project-Goals-2025-November-Update.md/

# 해외 유튜브 영상들
- [261101) Rust’s Next Big Changes: Will It Get Easier? @m.ebrahimi12](https://youtu.be/k9zj8i8FBzE?si=Mo3v7pvGf3EM6wBr)

# Rust 프로젝트 목표: 이동 불가 타입과 소멸자 실행 보장 (github.com/rust-lang)
1P by GN⁺ 3달전 | 260817
- 모든 값이 이동되거나 `mem::forget` 으로 소멸자 실행 없이 잊힐 수 있다는 Rust의 전제를 완화해, 타입이 이동과 망각을 거부할 수 있도록 하는 2026~2027년 프로젝트가 승인됨
- 새로운 자동 트레이트 `Move`·`Destruct`·`Forget` 으로 이동, 암묵적 드롭, 망각 가능 여부를 명시하고 각 역량을 계층적으로 부여함
- `!Move`는 자기 참조 타입의 주소가 생존 기간 내내 고정되도록 하며, 이동 불가를 장소의 속성으로 표현하는 `Pin`의 복잡성을 줄이는 대안임
- `!Forget`은 소멸자 실행을 보장해 작업 핸들의 소멸자가 join을 수행하는 **안전한 비동기 범위 지정 `spawn`** 같은 패턴을 가능하게 함
- 컴파일러 MVP와 RFC, Linux 커널 검증, 소멸자 보장 설계에 집중하되 `Future` 변경과 `Pin` 마이그레이션은 별도 프로젝트로 남겨둠
- https://news.hada.io/topic?id=32132