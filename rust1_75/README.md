# Rust-1.75.0

https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html

# Rust 1.75.0: 54 highlights in 20 minutes | Nathan Stocks

https://youtu.be/Z8xig7wEV68?si=BUMlC4v5tvMQxaj0


# Words od Advice

- Users cannot add additional bounds to the return type.
  - 사용자는 반환 유형에 추가 경계를 추가할 수 없습니다.

- Don't use ```-> impl Trait``` or ```async fn``` in public traits.
  - 공공의 특성에서 ```-> impl Trait```이나 ```asyncfn```을 사용하지 마십시오.

- Dynamic dispatch is not yet supported(use #[async_trait])
  - 동적 전송이 아직 지원되지 않습니다(#[async_trait] 사용)

- Use #[trait_variant::make] for multithreaded runtimes
  - 다중 스레드 실행 시간에 #[trait_variant::make] 사용


<hr>

# opaque 에 대해 

- https://users.rust-lang.org/t/generate-c-api-for-struct-methods-with-opaque-pointers/83163

- https://github.com/rust-lang/rust/issues/74298

- https://stackoverflow.com/questions/75510856/expected-type-parameter-found-opaque-type

# tokio

- https://github.com/tokio-rs/tokio/blob/master/examples/chat.rs

- https://github.com/tokio-rs/tokio/blob/master/examples/hello_world.rs

