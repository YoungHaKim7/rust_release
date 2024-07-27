pub fn size_prefix(n: u32) -> &'static str {
    const K: u32 = 10u32.pow(3);
    const M: u32 = 10u32.pow(6);
    const G: u32 = 10u32.pow(9);
    match n {
        ..K => "",
        K..M => "k",
        M..G => "M",
        G.. => "G",
    }
}

fn main() {
    println!("{}", size_prefix(300)); // Output: ""
    println!("{}", size_prefix(1_200)); // Output: "k"
    println!("{}", size_prefix(1_200_000)); // Output: "M"
    println!("{}", size_prefix(1_200_000_000));
}
