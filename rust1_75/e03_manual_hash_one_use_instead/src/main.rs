use std::{collections::hash_map::RandomState, hash::BuildHasher};

fn main() {
    let s = RandomState::new();
    let value = vec![1, 2, 3];

    let hash = s.hash_one(&value);
    dbg!(s);
    dbg!(value);
    dbg!(hash);
}
