#[derive(Debug)]
struct Color(String);

#[derive(Debug)]
struct Paint {
    color: Color,
}

fn colors(paints: &[Paint]) -> impl Iterator<Item = &Color> {
    paints.iter().map(|p| &p.color)
}

fn main() {
    println!("impl Traits Return : {:?}", Color("Red".to_string()));
}
