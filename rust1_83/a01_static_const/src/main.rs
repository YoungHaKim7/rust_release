static S: i32 = 23;
const C: &i32 = &S;

fn main() {
    let a = C;
    println!("C : {a}");
}
