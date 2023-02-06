fn main() {
    let my_array: [usize; 5] = std::array::from_fn(|i| i);
    println!("{my_array:?}");
}
