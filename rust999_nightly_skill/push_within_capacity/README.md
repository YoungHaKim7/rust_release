- https://doc.rust-lang.org/beta/std/vec/struct.Vec.html#method.push_within_capacity
- Appends an element and returns a reference to it if there is sufficient spare capacity, otherwise an error is returned with the element.

Unlike push this method will not reallocate when there’s insufficient capacity. The caller should use reserve or try_reserve to ensure that there is enough capacity.
Examples

A manual, panic-free alternative to FromIterator:

- 여유 용량이 충분하면 요소를 추가하고 그 참조를 반환하며, 그렇지 않으면 해당 요소와 함께 오류를 반환합니다.

푸시와 달리, 이 메서드는 용량이 부족해도 메모리를 재할당하지 않습니다. 충분한 용량을 확보하기 위해 호출자는 reserve 또는 try_reserve를 사용해야 합니다.
예

FromIterator의 수동적이고 당황스러운 대안:

# Goose언어를 대항하기 위한 해결책 4가지
- I saw a post about the Goose language on my feed a few weeks ago. Its design is excellent, it provides a default setup that is highly CPU cache friendly and malloc efficient, both at the language level and within the standard library. Consequently, typical code written in Goose will likely perform better than code written by the average user in C, C++, or Rust.

It is actually possible to achieve this in Rust, though it would mean bypassing Rust’s standard library, as the latter wasn't built with this specific layout and design philosophy. You could, however, create a "development framework", not a web framework, but a general purpose development framework, consisting of a set of configurations and libraries designed to build any kind of program using this CPU cache friendly design.

To build that development framework in Rust optimized for maximum CPU cache and malloc efficiency, you could :

- 1. Use `no_std`.

- 2. Implement an arena allocator that pre allocates a large block of virtual address space using Memmap (e.g., 1 GB). This could be single ended or double ended, structured as either a single contiguous block or a series of chunks.

- 3. Implement relative pointers, using PhantomData to manage lifetimes.

- 4. Define types like strings, vectors, and other growable structures using these relative pointers, for example :

```rs
struct Vec { 
    rel_ptr: RelPtr,
    len: usize,
    cap: usize
}
```
- The relative pointer would point to an offset from the start of the arena, rather than being a raw, fixed memory address. The new method for these types would then accept an allocator instance.

If the arena is single ended, only one growable structure can exist at the very end of the arena, if you need two structures to grow, you should place them in separate arenas. Within a single struct, only one field can be growable, if you need more than one, you should move the additional fields into a nested struct and assign it its own arena. It’s easy to use: if you need four growable data structures, simply create four arenas, for fixed size structures, you can just add them to an existing arena.

In a "double ended" arena, you can have two growable structures within a single arena, one at the very beginning and one at the very end.

grow won’t memcpy existing values ​​because the system uses relative pointers rather than fixed address pointers, the trade off is a minor mathematical operation during each access. For fixed size types, you can continue using fixed address pointers.

The Drop implementation for the custom collection doesn’t free physical memory, it merely decrements the "bump pointer" back to the position preceding the collection's first element. Consequently, the next allocation simply advances the bump pointer, overwriting the existing values, no system calls required. It is the Drop implementation within the arena allocator itself that releases the physical memory.

For maximum performance, you could implement an "arena pool, maintaining a set of arenas so that new allocations don't always start from zero and don't require system calls to the OS. Alternatively, to replicate the approach used in Goose, you could create a procedural macro that calculates the maximum number of growable collections declared within each function, determines the highest count, and then pre allocates a global pool of arenas of that size (but this will be hardly reliable because proc macro just see raw text).

Last time I checked, Rust already had in place construction optimization, when a function returns a value, it’s constructed directly on the target stack rather than being copied via `memcpy`. However, I haven’t verified whether this is guaranteed or if there are edge cases where it might fail to occur.

- Without a compiler, this is a bit complicated, but still possible. The issue I’ve encountered is, for example:

alloc fixed size 5x

growable 1x

fixed size 3x

If that last fixed size allocation is placed into the same arena, the growable segment can no longer expand because it would overwrite the data from the fixed size 3x allocation. A compiler like Goose can reorder these at compile time, moving the fixed size 3x segment to the top. You could achieve this by building your own parser to reorder the placement in the arena, for a simple case like the one above, you could do it manually, though that’s prone to human error. Furthermore, Rust doesn’t yet support stable user space plugins, building a custom parser would also be faster compile speed than using procedural macros.

Alternatively, you could implement a double ended arena: place the growable data at one end, and use the opposite end to append new collections to the arena
- https://users.rust-lang.org/t/do-you-consider-goose-instead-of-rust/143227/12

# goose 과대광고가 심하네 --
- I would not consider it as it describes itself as LLM slop

    This repo was produced almost entirely with AI, from a human language design. I had designed Goose several years ago

but if you ignore that part:

    It doesn't have traits, just "generics" (which look more like templates to me) with a (presumably) very large amount of post-monomorphisation errors and/or incorrect behaviour (a generic function may want to call foo on a generic type to do something, but some types may have a method called foo that does something unrelated but has the same name on accident)
    The return from other functions thing is absolutely horrifying
    The multithreading system is very limited, it can only do message parsing via channels between different programs, rust can do the exact same if you want, but it can also share data in just about every other way.
    There being one queue per type is a global state that can easily lead to very difficult to debug bugs.
    The language generally feels very limiting and incomplete.

The many stacks idea is nice, but it's not enough to make up for all the language's issues,

Although adding compiler support for many stacks to rust would be interesting,
And much easier than making an entire new language and compiler good enough to compete with rust.
- 스스로를 LLM 슬롭이라고 표현하고 있어서 그렇게 생각하지 않겠습니다

    이 저장소는 인간의 언어 디자인을 바탕으로 거의 전적으로 AI로 제작되었습니다. 나는 몇 년 전에 구스를 디자인했어요

하지만 그 부분을 무시하면:

    특성이 없고, 단지 '제네릭'만 있는데(개인적으로는 템플릿에 더 가깝게 보입니다), (아마도) 모노모피케이션 후에 발생하는 오류나 잘못된 동작이 매우 많습니다. (제네릭 함수가 제네릭 타입에 대해 foo를 호출해서 뭔가를 하려고 할 수 있습니다, 하지만 일부 타입은 foo라는 메서드를 가질 수 있는데, 이 메서드는 관련 없는 일을 하지만 실수로 같은 이름을 갖게 됩니다.)
    다른 함수들의 반환값 문제는 정말 끔찍합니다
    멀티스레딩 시스템은 매우 제한적이라, 서로 다른 프로그램 간 채널을 통한 메시지 파싱만 가능합니다. Rust가 똑같이 할 수는 있지만, 거의 모든 다른 방식으로 데이터를 공유할 수 있습니다.
    타입별로 큐가 하나씩 존재하는 것은 전역 상태가 되어 디버깅이 매우 어려운 버그를 쉽게 유발할 수 있습니다.
    그 언어는 대체로 매우 제한적이고 불완전하게 느껴집니다.

여러 개의 스택 아이디어는 좋지만, 언어의 모든 문제를 해결하기에는 부족합니다,

많은 스택에 대해 Rust에 컴파일러 지원을 추가하는 것이 흥미로울 수 있지만,
그리고 러스트와 경쟁할 만큼 완전히 새로운 언어와 컴파일러를 만드는 것보다 훨씬 쉽습니다.
