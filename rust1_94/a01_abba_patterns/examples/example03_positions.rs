//! Example 3: Find ABBA pattern positions
//! Demonstrates finding and displaying where ABBA patterns occur

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
    println!("=== Finding ABBA Pattern Positions ===\n");

    let test_strings = vec![
        "abba",
        "xyyxyyx",
        "abababa",
        "xabbay",
        "no-pattern-here",
    ];

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
