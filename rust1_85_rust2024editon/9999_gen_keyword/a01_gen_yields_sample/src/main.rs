// #![feature(coroutines, gen_blocks)]
#![feature(gen_blocks, yield_expr)]
fn main() {
    let count_to_ten = gen {
        for n in 0..10 {
            yield (n);
        }
    };
    let xs: Vec<_> = count_to_ten.into_iter().collect();
    println!("{:?}", xs);
}

