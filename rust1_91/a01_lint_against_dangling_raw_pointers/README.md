# 로컬 변수에서 매달린 원시 포인터에 보푸라기를 추가합니다:

```bash
warning: a dangling pointer will be produced because the local variable `x` will be dropped
경고: 로컬 변수 'x'가 떨어지기 때문에 매달린 포인터가 생성됩니다
 --> src/main.rs:3:5
  |
1 | fn f() -> *const u8 {
  |           --------- return type of the function is `*const u8`
                         함수의 반환 유형은 '*const u8'입니다
2 |     let x = 0;
  |         - `x` is part the function and will be dropped at the end of the function
              'x'는 함수의 일부이며 함수가 끝날 때 삭제됩니다
3 |     &x
  |     ^^
  |
  = note: pointers do not have a lifetime; after returning, the `u8` will be deallocated at the end of the function because nothing is referencing it as far as the type system is concerned
         참고: 포인터는 수명이 없습니다. 반환된 후에는 타입 시스템에 관한 한 'u8'을 참조하는 것이 없기 때문에 함수의 끝에서 'u8'이 할당 해제됩니다
  = note: `#[warn(dangling_pointers_from_locals)]` on by default


# ~~~~ Result
Add lint against dangling raw pointers from local variables :
	0x16fa8dd1f

```

