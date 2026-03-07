# Result

```bash
=== Basic ABBA Pattern Detection ===

✓ "abba      " -> true (expected: true)
✓ "xyyx      " -> true (expected: true)
✓ "aaaa      " -> false (expected: false)
✓ "abca      " -> false (expected: false)
✓ "ababa     " -> false (expected: false)
✓ "hello     " -> false (expected: false)
✓ "          " -> false (expected: false)
✓ "abb       " -> false (expected: false)
✓ "aabb      " -> false (expected: false)
=== Filtering Words with ABBA Patterns ===

All words:
radar level civic kayak abba xyyx deed noon hello world rust code anna elle momm 

Words with ABBA pattern: 6 found
  - abba
  - xyyx
  - deed
  - noon
  - anna
  - elle
=== Finding ABBA Pattern Positions ===

String: "abba"
  Has ABBA: true
  ABBA positions:
    - Position 0: "abba"

String: "xyyxyyx"
  Has ABBA: true
  ABBA positions:
    - Position 0: "xyyx"
    - Position 3: "xyyx"

String: "abababa"
  Has ABBA: false

String: "xabbay"
  Has ABBA: true
  ABBA positions:
    - Position 1: "abba"

String: "no-pattern-here"
  Has ABBA: false


```

- Done! Created 3 working examples demonstrating the `has_abba` function using Rust 1.94's `array_windows()`:

| Example | Description | Run with |
|---------|-------------|----------|
| `example01_basic.rs` | Basic ABBA pattern detection with test cases | `cargo run --example example01_basic` |
| `example02_filter.rs` | Filter a word list for ABBA patterns | `cargo run --example example02_filter` |
| `example03_positions.rs` | Find positions where ABBA patterns occur | `cargo run --example example03_positions` |

