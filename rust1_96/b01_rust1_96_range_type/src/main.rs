// use std::ops::Range;
use std::range::Range;

// tuple Struct
// macro
#[derive(Clone)]
pub struct Span(Range<usize>);

impl Span {
    pub fn of<'a>(&self, s: &'a str) -> &'a str {
        &s[self.0.clone()]
    }
}

fn main() {
    let a = 1usize..5;

    // let b = 1_usize..5;

    let b_range = Span(a.into());

    let text = "Hello World";
    println!("{}", b_range.of(text));
}
