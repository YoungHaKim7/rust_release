//! Example 2: Filter words with ABBA patterns
//! Demonstrates filtering a list of words to find those with ABBA patterns

fn has_abba(s: &str) -> bool {
    s.as_bytes()
        .array_windows()
        .any(|[a1, b1, b2, a2]| (a1 != b1) && (a1 == a2) && (b1 == b2))
}

fn main() {
    println!("=== Filtering Words with ABBA Patterns ===\n");

    let words = vec![
        "radar", "level", "civic", "kayak",    // Palindromes
        "abba", "xyyx", "deed", "noon",        // ABBA patterns
        "hello", "world", "rust", "code",      // No ABBA
        "anna", "elle", "momm",                // Mixed
    ];

    println!("All words:");
    for word in &words {
        print!("{word} ");
    }
    println!("\n");

    let words_with_abba: Vec<_> = words
        .iter()
        .filter(|word| has_abba(word))
        .collect();

    println!("Words with ABBA pattern: {} found", words_with_abba.len());
    for word in words_with_abba {
        println!("  - {word}");
    }
}
