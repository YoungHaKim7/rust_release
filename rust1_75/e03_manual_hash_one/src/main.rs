use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hash, Hasher};

fn main() {
    let s = RandomState::new();
    let value = vec![1, 2, 3];

    let mut hasher = s.build_hasher();
    value.hash(&mut hasher);
    let hash = hasher.finish();
    dbg!(value);
    dbg!(hasher);
    dbg!(hash);
}
