#[derive(Debug)]
struct Color(String);

#[derive(Debug)]
struct Paint {
    color: Color,
}

trait ColorCollection {
    fn colors(&self) -> impl Iterator<Item = &Color>;
}

impl ColorCollection for &[Paint] {
    fn colors(&self) -> impl Iterator<Item = &Color> {
        self.iter().map(|p| &p.color)
    }
}

fn main() {
    println!("impl Traits Return : {:?}", Color("Red".to_string()));
}
