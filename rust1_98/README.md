# Announcing Rust 1.98.0| Aug. 20, 2026 · The Rust Release Team 
- https://blog.rust-lang.org/2026/08/20/Rust-1.98.0/


# 이제 UB나지 않는다.

```rust
use std::mem::ManuallyDrop;

fn main() {
    let mut x = ManuallyDrop::new(Box::new(1));
    unsafe { ManuallyDrop::drop(&mut x) };
    let x = x; // UB!
    println!("Manuall Drop Working");
}
```

- Rust 1.96.0 버전에서 이 문제를 수정하여 더 이상 미정의 동작(UB)이 아니게 되었습니다. 이번 릴리스에서는 문서를 업데이트하여 ManuallyDrop향후에도 이 코드가 미정의 동작이 아님을 안정적으로 보장합니다. 자세한 내용은 ManuallyDrop문서 와 관련 RFC 3336을 참조하십시오 .
- In Rust 1.96.0 we fixed this, so this code was no longer UB. In this release we have updated the ManuallyDrop documentation, providing a stable guarantee that this code will continue to not be UB in the future. See ManuallyDrop docs and the related RFC 3336 for more information.
- https://rust-lang.github.io/rfcs/3336-maybe-dangling.html
