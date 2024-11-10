use std::{collections::HashMap, hash::RandomState};

fn main() {
    let s = RandomState::new();
    let mut map = HashMap::with_capacity_and_hasher(10, s);
    map.insert(1, 2);
    for i in map {
        println!("{i:?}");
    }
}
