use std::{collections::HashMap, hash::RandomState};

fn main() {
    let s = RandomState::new();
    let mut map = HashMap::with_hasher(s);
    map.insert(1, 2);
    for i in map {
        println!("{i:?}");
    }
}
