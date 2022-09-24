// https://doc.rust-lang.org/rust-by-example/std_misc/file/open.html
use std::fs::File;
use std::io::prelude::*;
use std::{fmt::Write, fs, path::Path};

fn example(dir: impl AsRef<Path>, name: &str, num: i32) {
    let mut file = dir.as_ref().join("hello").into_os_string();

    write!(file, "-{name}-{num}.txt").unwrap();

    fs::write(file, "Hi!").unwrap();
}

fn main() {
    example(Path::new("text/hello.txt"), "hi young", 3);
    let path = Path::new("text/hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}
