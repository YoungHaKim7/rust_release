# Result
- Note: This method is only available on platforms that support atomic operations on AtomicPtr.
  - 참고: 이 방법은 AtomicPtr에서 원자 연산을 지원하는 플랫폼에서만 사용할 수 있습니다

```bash
size_of::<i64> : 8 bytes
atom : 0x8
```

<hr />

# `fetch_ptr_add`

- Offsets the pointer’s address by adding val (in units of T), returning the previous pointer.

This is equivalent to using wrapping_add to atomically perform the equivalent of ptr = ptr.wrapping_add(val);.

This method operates in units of T, which means that it cannot be used to offset the pointer by an amount which is not a multiple of size_of::<T>(). This can sometimes be inconvenient, as you may want to work with a deliberately misaligned pointer. In such cases, you may use the fetch_byte_add method instead.

fetch_ptr_add takes an Ordering argument which describes the memory ordering of this operation. All ordering modes are possible. Note that using Acquire makes the store part of this operation Relaxed, and using Release makes the load part Relaxed.

Note: This method is only available on platforms that support atomic operations on AtomicPtr.

- 이전 포인터를 반환하는 val(T 단위)을 추가하여 포인터의 주소를 오프셋합니다.

이는 ptr = ptr. wrapping_add(val)에 해당하는 값을 원자적으로 수행하기 위해 rapping_add를 사용하는 것과 같습니다;

이 메서드는 T 단위로 작동하므로 포인터를 크기_of:()의 배수가 아닌 양만큼 오프셋하는 데 사용할 수 없습니다. 의도적으로 정렬되지 않은 포인터로 작업하는 것이 불편할 수 있습니다. 이러한 경우 대신 fetch_byte_add 메서드를 사용할 수 있습니다.

fetch_ptr_add는 이 작업의 메모리 순서를 설명하는 Ordering 인수를 사용합니다. 모든 순서 모드가 가능합니다. Acquire를 사용하면 이 작업의 스토어 부분이 완화되고 Release를 사용하면 로드 부분이 완화됩니다.

참고: 이 방법은 AtomicPtr에서 원자 연산을 지원하는 플랫폼에서만 사용할 수 있습니다
