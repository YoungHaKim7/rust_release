use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut ls = Command::new("ls");
    let mut cat = Command::new("cat");
    cat.stdin(Stdio::piped());

    let out = ls.output().unwrap();

    let mut cat = cat.spawn().unwrap();

    cat.stdin.take().unwrap().write_all(&out.stdout).unwrap();

    let out = cat.wait_with_output().unwrap();

    println!("{}", String::from_utf8(out.stdout).unwrap());
}
