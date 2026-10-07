# Announcing Rust 1.99.0
Oct. 1, 2026 · The Rust Release Team
- https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/

# MAIN NEWS  What’s New in Rust 1.99.0
- Last week had some great announcements. Rust 1.99.0 got released, and Turso announced joining Supabase barely a month after Dioxus joined Cognition. Seems like Rust startups are having their moment. Anyway, 1.99.0 shipped, and two new features stuck with me.

- First, extern “C” variadics are now stable. Variadic functions are functions that can accept a variable number of arguments without needing multiple overloads. Before, you could only call such functions written in C. Now you can write them yourself in Rust.

- You write the function body in Rust, and it can be called from Rust, C, or any other language that understands the C ABI. Real-world cases include things like printf, execl, or logging helpers where the number of extra values can vary.
- 지난주에 멋진 발표들이 있었습니다. Rust 1.99.0이 출시되었고, Turso는 Dioxus가 Cognition에 합류한 지 한 달도 채 되지 않아 Supabase에 합류했다고 발표했습니다. 러스트 스타트업들이 한창 주목받고 있는 것 같네요. 어쨌든 1.99.0이 배포되었고, 두 가지 새로운 기능이 기억에 남네요.

- 먼저, extern "C" 가변 인자는 이제 안정화되었습니다. 가변 인수 함수는 여러 개의 오버로드 없이 가변 개수의 인수를 받을 수 있는 함수입니다. 이전에는 C로 작성된 함수만 호출할 수 있었습니다. 이제 Rust로 직접 작성할 수 있습니다.

- 함수 본문은 러스트로 작성하며, 러스트, C, 또는 C ABI를 지원하는 다른 어떤 언어에서도 호출할 수 있습니다. 실제 사례로는 printf, execl, 로깅 도구 등이 있으며, 이들 도구는 추가 값의 개수가 다양할 수 있습니다.

- Here’s a simple example:

```rs
/// SAFETY: must be called with (at least) 2 arguments
#[unsafe(no_mangle)]
pub unsafe extern "C" fn log_values(count: i32, mut args: ...) {
    println!("--- Log start ---");

    for i in 0..count {
        let value: i32 = unsafe { args.next_arg() };
        println!("  value[{i}] = {value}");
    }

    println!("--- Log end ---");
}

fn main() {
    unsafe {
        log_values(3, 10, 20, 30);
        log_values(1, 99);
        log_values(0);
    }
}
```
The release also stabilized support for defining naked variadic functions with non-C ABIs, with the only caveat that they must be written in inline assembly.

The second feature I know a lot of you will appreciate is the ability to get layout information such as size, alignment, etc from raw pointers to unsized types.

Pre 1.99.0 it wasn’t possible to reliably query the layout of unsized or maybe types such as slices ([T]), str, or trait objects from a raw pointer. Their size isn’t known at compile time and depends on runtime metadata, so the safety rules had to be carefully settled first.

Rust 1.99 stabilizes three functions for this: 
- `std::mem::size_of_val_raw`, 
- `std::mem::align_of_val_raw`, and
- `std::alloc::Layout::for_value_raw`.

They work for both Sized and unsized types and mirror the existing safe versions 

- `size_of_val`, 
- `align_of_val`, and 
- `Layout::for_value`.

There are also several other stabilized APIs, you can check the full release notes for everything.
- 또한, 비-C ABI를 사용해 나체 가변 인자 함수를 정의하는 지원을 안정화했으며, 단 한 가지 조건은 인라인 어셈블리로 작성되어야 한다는 점입니다.

많은 분들이 좋아하실 두 번째 기능은, 크기나 정렬 같은 레이아웃 정보를 원시 포인터에서 크기 없는 타입으로 얻을 수 있다는 점입니다.

1.99.0 이전 버전에서는 원시 포인터로부터 크기가 지정되지 않았거나 슬라이스([T]), str, trait 객체와 같은 타입의 레이아웃을 신뢰성 있게 조회할 수 없었습니다. 컴파일 시점에 크기가 알려져 있지 않고 런타임 메타데이터에 따라 달라지기 때문에, 안전 규칙을 먼저 신중하게 정해야 했습니다.

Rust 1.99에서는 다음 세 가지 함수가 안정화되었습니다: `std::mem::size_of_val_raw`, `std::mem::align_of_val_raw`, 그리고 std::alloc::레이아웃::값_인-원시.
 `std::mem::size_of_val_raw`, `std::mem::align_of_val_raw`, and `std::alloc::Layout::for_value_raw`.


이 함수들은 크기가 있는 타입과 없는 타입 모두에 대해 작동하며, 기존의 safe 버전인 `size_of_val, align_of_val`, `Layout::for_value`와 동일한 기능을 제공합니다.

이 외에도 여러 안정화된 API가 있으며, 자세한 내용은 전체 릴리스 노트를 참고하시면 됩니다.
- https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/?utm_source=substack&utm_medium=email
