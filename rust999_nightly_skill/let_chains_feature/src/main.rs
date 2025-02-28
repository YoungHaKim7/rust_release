#![feature(let_chains)] // Enable the feature
fn main() {
    let deck = vec![5, 6, 10, 3, 2, 4]; // Example card values.
    let mut total = 0;
    let max = 21;

    let mut iter = deck.into_iter();

    // The loop continues only if:
    // 1. A card is drawn (Some(card)), AND
    // 2. Adding the card does not exceed 21.
    while let Some(card) = iter.next() && total + card <= max {
        total += card;
        println!("Drew card: {} (total: {})", card, total);
    }

    println!("Final total: {}", total);
}
