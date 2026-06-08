use std::ops::Range;

#[derive(Clone)]
pub struct Span(Range<usize>);

impl Span {
    pub fn of<'a>(&self, s: &'a str) -> &'a str {
        &s[self.0.clone()]
    }
}

fn main() {
    let a = 1usize..5;

    let b_range = Span(a);

    let text = "Hello World";
    println!("{}", b_range.of(text)); // ello
}
