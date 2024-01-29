use std::collections::HashMap;

fn main() {
    let map = HashMap::from([("population", [500, 600])]);
    println!("Populations: {:?}", map.get("population").as_slice());
    println!("Populations: {:?}", map.get("president").as_slice());
}
