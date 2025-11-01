# Rust Edition 2021(1.56.0 이후 버전은 Rust 2021 Edition)
- https://doc.rust-lang.org/edition-guide/rust-2021/index.html
- (211021) https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html

# (211022) Rust 1.56.0 및 Rust 2021 발표 (blog.rust-lang.org)
- https://news.hada.io/topic?id=5240
  - 12P by xguru 2021-10-22 
- Rust는 언어가 빠르게 발전하면서도 안정성을 유지할수 있도록 Edition 방식을 채택
  - → 하위호환이 안되는 기능들을 Opt-in 방식으로 에디션에 추가
- Rust 2021

  -ㅤDisjoint capture : Closure 가 실제 사용하는 필드만 캡쳐

  -ㅤIntoIterator for arrays : `array.into_iter()` 가 레퍼런스 대신 밸류로 iterate
  -ㅤ매크로에서 OR (`|`) 사용

  -ㅤ새 Cargo Feature Resolver (버전 2)가 기본 값
  -ㅤPrelude(무조건 임포트되는 표준 라이브러리)에 추가 : `TryInfo`, `TryFrom`, `FromIterator`

  -ㅤPanic macro가 무조건 포맷 스트링을 요구. 마치 `println!()` 처럼
  -ㅤ`ident#`, `ident"..."`, `ident'...'` 가 예약어에 추가

  -ㅤbare-trait-objects, ellipsis-inclusive-range-patterns 경고를 에러로
- cargo fix 를 통해서 대부분 자동으로 마이그레이션 지원     

# Rust 2021 Edition 계획 (blog.rust-lang.org)
- https://news.hada.io/topic?id=4241
  - 9P by xguru 2021-05-12 
- Rust는 언어가 빠르게 발전하면서도 안정성을 유지할수 있도록 Edition 방식을 채택

  -ㅤ하위호환이 안되는 기능들을 Opt-in 방식으로 에디션에 추가

  -ㅤ2018 에디션에 추가된 `async`는 아직 최신 버전에서도 정식 키워드가 아님

  -ㅤ특정 에디션에서 만들어진 Crate는 다른 에디션과 호환해야함

ㅤ - 에디션 이관은 쉽고 대부분 자동화

- 2021 에디션에 추가될 기능들
  -ㅤPrelude(무조건 임포트되는 표준 라이브러리)에 추가 : `TryInfo`, `TryFrom`, `FromIterator`

  -ㅤ새 Cargo Feature Resolver (버전 2)가 기본 값

  -ㅤ→Array 용 `IntoIterator`

  -ㅤClosure 가 실제 사용하는 필드만 캡쳐

  -ㅤ더 일관적인 panic!() 매크로

  -ㅤ미래에 추가될 문법을 위해서 접두사 붇은 식별자와 리터럴에 대한 구문을 예약

  -ㅤbare-trait-objects, ellipsis-inclusive-range-patterns 경고를 에러로

  -ㅤ매크로에서 OR (`|`) 사용 
