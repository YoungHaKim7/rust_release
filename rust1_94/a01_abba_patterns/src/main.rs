fn has_abba(s: &str) -> bool {
    s.as_bytes()
        .array_windows()
        .any(|[a1, b1, b2, a2]| (a1 != b1) && (a1 == a2) && (b1 == b2))
}

fn find_abba_positions(s: &str) -> Vec<(usize, String)> {
    s.as_bytes()
        .array_windows()
        .enumerate()
        .filter_map(|(i, &[a1, b1, b2, a2])| {
            if (a1 != b1) && (a1 == a2) && (b1 == b2) {
                let pattern: String = vec![a1, b1, b2, a2]
                    .into_iter()
                    .map(|c| c as char)
                    .collect();
                Some((i, pattern))
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    // "example01_basic"
    println!("=== Basic ABBA Pattern Detection ===\n");

    let test_cases = vec![
        ("abba", true),   // Classic ABBA pattern
        ("xyyx", true),   // Another ABBA pattern
        ("aaaa", false),  // Not ABBA (all same)
        ("abca", false),  // Not ABBA (middle not same)
        ("ababa", false), // Windows: "abab", "baba" - neither are ABBA
        ("hello", false), // No ABBA pattern
        ("", false),      // Empty string
        ("abb", false),   // Too short
        ("aabb", false),  // Window is "aabb" - doesn't match ABBA
    ];

    for (input, expected) in test_cases {
        let result = has_abba(input);
        let status = if result == expected { "✓" } else { "✗" };
        println!(
            "{status} \"{:10}\" -> {} (expected: {})",
            input, result, expected
        );
    }

    // example02_filter.rs
    println!("=== Filtering Words with ABBA Patterns ===\n");

    let words = vec![
        "radar", "level", "civic", "kayak", // Palindromes
        "abba", "xyyx", "deed", "noon", // ABBA patterns
        "hello", "world", "rust", "code", // No ABBA
        "anna", "elle", "momm", // Mixed
    ];

    println!("All words:");
    for word in &words {
        print!("{word} ");
    }
    println!("\n");

    let words_with_abba: Vec<_> = words.iter().filter(|word| has_abba(word)).collect();

    println!("Words with ABBA pattern: {} found", words_with_abba.len());
    for word in words_with_abba {
        println!("  - {word}");
    }
    // Example 3: Find ABBA pattern positions
    // Demonstrates finding and displaying where ABBA patterns occur
    println!("=== Finding ABBA Pattern Positions ===\n");

    let test_strings = vec!["abba", "xyyxyyx", "abababa", "xabbay", "no-pattern-here"];

    for s in test_strings {
        let has = has_abba(s);
        let positions = find_abba_positions(s);

        println!("String: \"{s}\"");
        println!("  Has ABBA: {has}");

        if !positions.is_empty() {
            println!("  ABBA positions:");
            for (pos, pattern) in positions {
                println!("    - Position {pos}: \"{pattern}\"");
            }
        }
        println!();
    }
}
