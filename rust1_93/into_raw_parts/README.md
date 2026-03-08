# Result


```bash


```

- Decomposes a `String` into its raw components: `(pointer, length, capacity)`.
 - Returns the raw pointer to the underlying data, the length of the string (in bytes), and the allocated capacity of the data (in bytes). These are the same arguments in the same order as the arguments to [`from_raw_parts`].

- After calling this function, the caller is responsible for the memory previously managed by the `String`. The only way to do this is to convert the raw pointer, length, and capacity back into a `String` with the [`from_raw_parts`] function, allowing the destructor to perform the cleanup.

- `String` 을 원시 구성 요소로 분해합니다: '(pointer, length, capacity, 점, 길이, 용량)'.
 - 기본 데이터에 대한 원시 포인터, 문자열의 길이(바이트 단위), 데이터의 할당된 용량(바이트 단위)을 반환합니다. 이 인수들은 ['from_raw_parts'] 인수와 동일한 순서로 반환됩니다.

- 이 함수를 호출한 후 발신자는 이전에 '스트링'에서 관리하던 메모리를 책임집니다. 이를 수행하는 유일한 방법은 원시 포인터, 길이, 용량을 ['from_raw_parts'] 함수가 있는 '스트링'으로 다시 변환하여 파괴자가 정리를 수행할 수 있도록 하는 것입니다.

# `from_raw_parts`
- [`from_raw_parts`: String::from_raw_parts](https://doc.rust-lang.org/stable/std/string/struct.String.html#method.from_raw_parts)

- Creates a new String from a pointer, a length and a capacity.

- **Safety**

- This is highly unsafe, due to the number of invariants that aren’t checked:

  - all safety requirements for `Vec::<u8>::from_raw_parts`.
  - all safety requirements for `String::from_utf8_unchecked`.

- Violating these may cause problems like corrupting the allocator’s internal data structures. For example, it is normally not safe to build a String from a pointer to a C char array containing UTF-8 unless you are certain that array was originally allocated by the Rust standard library’s allocator.

- The ownership of buf is effectively transferred to the String which may then deallocate, reallocate or change the contents of memory pointed to by the pointer at will. Ensure that nothing else uses the pointer after calling this function.

- 포인터, 길이 및 용량에서 새 문자열을 만듭니다.

- **안전**

- 이것은 확인되지 않은 불변량의 수 때문에 매우 안전하지 않습니다:

  - `Vec:<u8>::from_raw_parts`의 모든 안전 요구 사항.
  - `String::from_utf8_unchecked`의 모든 안전 요구 사항.

- 이를 위반하면 할당자의 내부 데이터 구조가 손상되는 등의 문제가 발생할 수 있습니다. 예를 들어, Rust 표준 라이브러리의 할당자가 원래 배열을 할당했는지 확실하지 않은 한 포인터에서 UTF-8이 포함된 C char 배열로 문자열을 생성하는 것은 일반적으로 안전하지 않습니다.

- buf의 소유권은 효과적으로 문자열로 이전되며, 문자열은 포인터가 가리키는 메모리의 내용을 임의로 할당 해제, 재할당 또는 변경할 수 있습니다. 이 함수를 호출한 후 포인터를 사용하는 다른 항목은 없는지 확인합니다.

- https://doc.rust-lang.org/stable/std/string/struct.String.html#method.from_raw_parts

```rs
unsafe {
    let s = string::from("hello");

    // deconstruct the string into parts.
    let (ptr, len, capacity) = s.into_raw_parts();

    let s = string::from_raw_parts(ptr, len, capacity);

    assert_eq!(string::from("hello"), s);
}
```
