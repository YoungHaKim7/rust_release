# Rust Releases! Rust 1.71.0

https://youtu.be/8DG1V9nNYeg

# Named levels of debug information

The ```-Cdebuginfo``` compiler option has previously only supported numbers 0..=2 for increasing amounts of debugging information, where Cargo defaults to 2 in dev and test profiles and 0 in release and bench profiles. These debug levels can now be set by name: "none" (0), "limited" (1), and "full" (2), as well as two new levels, "line-directives-only" and "line-tables-only".

https://doc.rust-lang.org/cargo/reference/profiles.html