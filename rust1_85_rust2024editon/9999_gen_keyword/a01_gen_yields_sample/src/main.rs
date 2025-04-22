#![feature(coroutines, gen_blocks)]
fn main() {
    let count_to_ten = gen {
        for n in 0..10 {
            yield (n);
        }
    };
    let xs: Vec<_> = count_to_ten.into_iter().collect();
    println!("{:?}", xs);
}

