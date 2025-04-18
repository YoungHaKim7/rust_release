- https://blog.rust-lang.org/2025/04/03/Rust-1.86.0/

# Trait up-casting vs downcast-rs crate
- https://www.reddit.com/r/rust/comments/1jqhefq/trait_upcasting_vs_downcastrs_crate/?rdt=57482
With Rust 1.86 now supporting trait upcasting, for a `trait A: Any` , to downcast to a concrete type implementing it, is it better to use `downcast-rs` for downcasting or to just always upcast `&dyn A` to `&dyn Any` and then downcast from that?

- So these are slightly different - `downcast-rs` is used to add the `downcast_ref` etc. methods to `dyn MyTrait`, and with native trait upcasting, it could (and should) be implemented in terms of that functionality, in which case it generates methods that are thin wrappers around calls to `(self as &dyn Any).downcast_ref()`.

At that point, the only benefit is that you don't have to write those methods yourself, or ask your users to manually cast your trait pointers to `&dyn Any` etc.

That, or you may target MSRV < 1.86.
  - 따라서 이 방법들은 약간 다릅니다 - `downcast-rs`는 dynMyTrait에 `downcast_ref` 등 메서드를 추가하는 데 사용되며, 네이티브 특성 업캐스팅을 사용하면 해당 기능 측면에서 구현될 수 있으며, 이 경우 호출 주변에 얇은 래퍼 메서드를 생성합니다 `(자신을 &dyn Any.downcast_ref()`라고 부릅니다).

그 시점에서 유일한 이점은 이러한 메서드를 직접 작성하거나 사용자에게 특성 포인터를 수동으로 &dyn Any 등에 캐스팅하도록 요청할 필요가 없다는 것입니다.

또는 MSRV < 1.86을 목표로 할 수 있습니다.

- I was wondering if, downcast-rs, as it's implemented today: would it be more efficient to use that (for performing downcasts), or just switch to using native upcasting to dyn Any and then downcasting. I agree with you that, if it is (or will be) implemented internally using native upcasting, then they'll be more or less the same.
  - 오늘날 downcast-rs가 구현되고 있는 것처럼, 그것을 사용하는 것이 더 효율적인지 궁금합니다. 아니면 네이티브 업캐스팅을 사용하여 다이앤 애니로 전환한 후 다운캐스팅으로 전환하는 것이 더 효율적인지 궁금합니다. 네이티브 업캐스팅을 사용하여 내부적으로 구현된다면, 그들은 대체로 동일할 것이라는 점에 동의합니다.
 
  - There's maybe one extra layer of pointer indirection with native upcasting, but ideally, any downcasting you're doing shouldn't be performance-critical enough that it would make a difference.
  - 네이티브 업캐스팅을 사용하는 포인터 인다이렉션 레이어가 하나 더 있을 수 있지만, 이상적으로는 수행 중인 다운캐스팅이 성능에 큰 영향을 미치지 않아야 합니다.
-  Thank you. I took a deeper look at what downcast-rs expands to, and you're right, it provides `downcast*` methods on the `dyn Trait` objects, which internally just call the `as_any` method of the Downcast trait, and then downcast from that dyn Any object. So there're method calls (which may be inlined) but only one indirection.

But more than this, I think I'll stick to using downcast-rs for the below reason: 
  - 감사합니다. downcast-rs가 확장되는 것을 자세히 살펴봤는데, 당신 말이 맞아요. 이는 dyn Trait 객체에 대해 downcast* 메서드를 제공합니다. 내부적으로는 downcast trait의 as_any 메서드라고 부르고, 그 dyn Any 객체에서 downcast* 메서드를 제공합니다. 따라서 메서드 호출이 있지만 (인라인될 수 있는) 단 하나의 인다이렉션만 있습니다.

하지만 이보다 더 많은 이유로 downcast-rs를 계속 사용할 것 같습니다:

```rs
#[cfg(test)]
mod tests {
    use std::any::Any;

    use downcast_rs::{impl_downcast, Downcast};

    trait Trait: Downcast {}
    impl_downcast!(Trait);

    struct S;
    impl Trait for S {}

    #[test]
    fn test_downcast() {
        let s = S;
        let t: Box<dyn Trait> = Box::new(s);
        assert!(t.is::<S>());
        assert!((t as &dyn Any).is::<S>());
    }
}
```

-  Here, the type checker first tells me that `t` as `&dyn Any` (on the last assert) is incorrect, suggesting that I borrow t, and when I change it to `assert!((&t as &dyn Any).is::<S>());`, the type checker is happy. But this is incorrect since I'm casting &Box<_> to &dyn Any, and the downcast (is) will fail at run time. The right assert would be `assert!((&*t as &dyn Any).is::<S>());`.

But with using downcast-rs where I don't need to upcast to &dyn Any, it seems that such mistakes cannot happen. The crate provides downcast methods on the boxed objects directly. 
  - 여기서 타입 체커는 먼저 t as &dyn Any (마지막 어설션에서)가 틀렸다고 말하며, t를 빌리는 것을 제안합니다. (&t as &dyn Any.is:<S>())를 어설션으로 변경하면 타입 체커가 행복해집니다. 하지만 이는 제가 &dyn Any에게 &Box<_>를 캐스팅하고 있기 때문에 잘못된 것이며, 다운캐스트(is)는 실행 시 실패합니다. 올바른 어설션은 어설션입니다!(&*t as &dyn Any).is:<S>().

하지만 다운캐스트-rs를 사용할 때는 &dyn Any로 업캐스트할 필요가 없기 때문에 이러한 실수는 발생할 수 없는 것 같습니다. 상자는 박스형 객체에 대해 직접 다운캐스트 메서드를 제공합니다.
