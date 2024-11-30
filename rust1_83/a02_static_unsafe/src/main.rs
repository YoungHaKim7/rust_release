static mut S: i32 = 0;

// 상수 값 실패 상수 액세스 돌연변이 글로벌 메모리 평가
// evaluation of constant calue failed constant accesses mutable global memory
//const C1: i32 = unsafe { S };

// 잘못된 값 구성: 'const'에서 변경 가능한 메모리에 대한 참조가 발생했습니다
// constructing invalid value: encountered reference to mutable memory in `const`
// const C1: &i32 = unsafe { &S };

fn main() {
    println!("Hello, world!");
}
