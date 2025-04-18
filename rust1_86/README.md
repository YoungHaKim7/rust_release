- https://blog.rust-lang.org/2025/04/03/Rust-1.86.0/

# Trait up-casting vs downcast-rs crate
- https://www.reddit.com/r/rust/comments/1jqhefq/trait_upcasting_vs_downcastrs_crate/?rdt=57482
With Rust 1.86 now supporting trait upcasting, for a trait A: Any, to downcast to a concrete type implementing it, is it better to use downcast-rs for downcasting or to just always upcast &dyn A to &dyn Any and then downcast from that?

- So these are slightly different - downcast-rs is used to add the downcast_ref etc. methods to dyn MyTrait, and with native trait upcasting, it could (and should) be implemented in terms of that functionality, in which case it generates methods that are thin wrappers around calls to (self as &dyn Any).downcast_ref().

At that point, the only benefit is that you don't have to write those methods yourself, or ask your users to manually cast your trait pointers to &dyn Any etc.

That, or you may target MSRV < 1.86.
  - 따라서 이 방법들은 약간 다릅니다 - downcast-rs는 dynMyTrait에 downcast_ref 등 메서드를 추가하는 데 사용되며, 네이티브 특성 업캐스팅을 사용하면 해당 기능 측면에서 구현될 수 있으며, 이 경우 호출 주변에 얇은 래퍼 메서드를 생성합니다 (자신을 &dyn Any.downcast_ref()라고 부릅니다).

그 시점에서 유일한 이점은 이러한 메서드를 직접 작성하거나 사용자에게 특성 포인터를 수동으로 &dyn Any 등에 캐스팅하도록 요청할 필요가 없다는 것입니다.

또는 MSRV < 1.86을 목표로 할 수 있습니다.
