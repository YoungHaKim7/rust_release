//! Example 1: Basic ABBA pattern detection
//! Demonstrates the has_abba function with simple test strings

fn has_abba(s: &str) -> bool {
    s.as_bytes()
        .array_windows()
        .any(|[a1, b1, b2, a2]| (a1 != b1) && (a1 == a2) && (b1 == b2))
}

fn main() {
    println!("=== Basic ABBA Pattern Detection ===\n");

    let test_cases = vec![
        ("abba", true),      // Classic ABBA pattern
        ("xyyx", true),      // Another ABBA pattern
        ("aaaa", false),     // Not ABBA (all same)
        ("abca", false),     // Not ABBA (middle not same)
        ("ababa", false),    // Windows: "abab", "baba" - neither are ABBA
        ("hello", false),    // No ABBA pattern
        ("", false),         // Empty string
        ("abb", false),      // Too short
        ("aabb", false),     // Window is "aabb" - doesn't match ABBA
    ];

    for (input, expected) in test_cases {
        let result = has_abba(input);
        let status = if result == expected { "✓" } else { "✗" };
        println!("{status} \"{:10}\" -> {} (expected: {})", input, result, expected);
    }
}
