pub enum Color {
    Blue,
    Red,
    Violet,
}

pub enum Flower {
    Rose,
    Tulip,
    Violet,
}

pub fn roses_are_red_violets_are_blue_printer(
    (first_flower, first_flower_color): (Flower, Color),
    (second_flower, second_flower_color): (Flower, Color),
    pick_up_lines: &[&str],
) {
    if let Flower::Rose = first_flower
        && let Color::Red = first_flower_color
        && let Flower::Violet = second_flower
        && let Color::Blue = second_flower_color
        && let &[first_pick_up_line, ..] = pick_up_lines
    {
        println!("Roses are red, violets are blue, {}", first_pick_up_line);
    }
}

fn main() {
    roses_are_red_violets_are_blue_printer(
        (Flower::Rose, Color::Red),
        (Flower::Violet, Color::Blue),
        &["sugar is sweet and so are you"],
    );
}
