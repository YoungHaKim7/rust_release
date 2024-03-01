# Rust 1.46에 나온 const fn에 대해 fn가 뭐가 다르지?

- PSA: What is `const fn`?

https://www.reddit.com/r/rust/comments/iksmgk/psa_what_is_const_fn/

- And now main will work! But there's a catch. `const fn` is currently much more limited than a normal function. This simple example works and you can also now use loop and if in constant functions but there's a lot that won't work. Perhaps most notably, iterators won't work at all. Which amongst other things means no for loop. Work is continuing to allow more features in `const fn` but there will always be some functions that cannot be `const fn` (e.g. ones that rely on platform APIs).

  - In summary, that's all `const fn` means. You can use it in more places than an ordinary function.

  - It can be used as a
    - static variable,
    - a const variable,
    - as a length for an array, etc.

- So should you mark every function as `const fn` so long as it compiles? No! `const fn` is a contract between you and the people that call your function. You are declaring that you will never change the function in ways that are invalid for `const fn`. This contract may prevent some future optimizations so think carefully before using it.

- At this point you might be wondering why I haven't mentioned the phrase "compile time". That's because it's not what const fn is really about. Just about all functions can be run either partially or fully at compile time (the optimizer permitting) and a `const fn` may be run at runtime. Of course sometimes you do want to make 100% sure a function is run at compile time which is a useful side-effect of doing `const BAR: usize = foo();`. Note though that if you just do `let bar = foo();` (where foo is a const fn) it's pretty much the same as using an ordinary function which may or may not be run at compile time.

- In the future there might be inline const that explicitly declares that you want to run part of your program at compile time. For example:

- 그리고 이제 메인이 작동할 것입니다! 하지만 한 가지 문제점이 있습니다. const fn은 현재 일반 함수보다 훨씬 더 제한적입니다. 이 간단한 예제는 작동하며 이제 루프를 사용할 수도 있으며 일정한 함수를 사용할 수도 있지만 작동하지 않는 것이 많습니다. 아마도 가장 주목할 만한 것은 반복기가 전혀 작동하지 않을 것입니다. 무엇보다 루프에 대한 기능이 없음을 의미합니다. 작업은 const fn에서 더 많은 기능을 계속 허용하지만 항상 const fn(예: 플랫폼 API에 의존하는 기능)이 있을 것입니다.

- 정리하면,

  - const fn은 그게 전부입니다.
    - 일반적인 함수보다 더 많은 곳에서 사용할 수 있습니다.
  - 정적 변수,
  - const 변수,
  - 배열의 길이
    등으로 사용할 수 있습니다.

- 그래서 컴파일만 된다면 모든 함수를 const fn으로 표기해야 하나요? 아니요!

# `const fn`은 당신과 당신의 함수를 호출하는 사람들 사이의 계약입니다.

- 당신은 const fn에 유효하지 않은 방식으로 함수를 절대 변경하지 않겠다고 선언하고 있습니다. 이 계약은 미래의 최적화를 방지할 수 있으므로 사용하기 전에 신중하게 생각해야 합니다.

- 이 시점에서 제가 왜 "compile 시간"이라는 문구를 언급하지 않았는지 궁금하실 것입니다. const fn이 실제로 그런 것이 아니기 때문입니다. 거의 모든 함수는 컴파일 시 부분적으로 또는 전체적으로 실행될 수 있으며(옵티마이저 허용) const fn은 런타임에 실행될 수 있습니다. 물론 때로는 함수가 컴파일 시 실행되는지 100% 확인하고 싶을 때도 있는데, 이는 const BAR의 유용한 부작용입니다: usize = foo (); (foo는 const fn입니다). bar = foo ()을 그냥 두면 컴파일 시 실행될 수도 있고 실행되지 않을 수도 있는 일반 함수를 사용하는 것과 거의 같습니다.

- 미래에는 컴파일 시 프로그램의 일부를 실행할 것을 명시적으로 선언하는 인라인 컨스트럭트가 있을 수 있습니다. 예를 들어, 다음과 같습니다:
