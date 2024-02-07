fn iter_to_array<Element, const N: usize>(mut iter: impl Iterator<Item = Element>) -> [Element; N] {
    // Here I use `()` to make array zero-sized -> no real use in runtime.
    // `map` creates new array, which we fill by values of iterator.
    let res: [_; N] = std::array::from_fn(|_| iter.next().unwrap());
    // Ensure that iterator finished
    // assert!(matches!(iter.next(), None));
    assert!(iter.next().is_none());
    res
}

fn main() {
    let my_vec = vec![1, 2, 3, 4, 5, 7];
    let my_array: [&i32; 6] = iter_to_array(my_vec.iter());
    println!("my array : {:?}", my_array);
}
