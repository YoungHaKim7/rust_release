#[derive(Clone, Copy)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn of<'a>(&self, s: &'a str) -> &'a str {
        &s[self.start..self.end]
    }
}

fn main() {
    let span = Span::new(1, 5);

    let text = "Hello World";
    println!("{}", span.of(text)); // ello
}
