fn main() {
    let x = 1.0f64;
    let clampled = x.clamp(0.0, 1.0f64.next_down());
    println!("x : {}", x);
    println!("clampled : {}", clampled);

}
