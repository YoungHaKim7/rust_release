fn main() {
    println!("{:.1e}", 999);
}

// rust 1.75 bug(incorrect)

// 1.00e3

// ~~~~~~~~~~~~~~~~~~~~~

// rust 1.76 After(correct)

// 1.0e3
