pub fn size_prefix(n: &u32) -> &str {
    const K: u32 = 10u32.pow(3); // 1,000
    const M: u32 = 10u32.pow(6); // 1,000,000
    const G: u32 = 10u32.pow(9); // 1,000,000,000
    match n {
        ..K => "NULL",
        K..M => "k",
        M..G => "M",
        G.. => "G 존나게 크다",
    }
}

fn main() {
    // 1'000'000'000
    let n = 1_000_000_000;
    println!("{} {}", n, size_prefix(&n));
    let n1 = 1_000;
    println!("{} {}", n1, size_prefix(&n1));
}
