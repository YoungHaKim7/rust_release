use std::process::{Command, Stdio};
use std::str;

fn main() {
    let ps_child = Command::new("ps") // `ps` command...
        .arg("axww") // with argument `axww`...
        .stdout(Stdio::piped()) // of which we will pipe the output.
        .spawn() // Once configured, we actually spawn the command...
        .unwrap(); // and assert everything went right.
    let grep_child_one = Command::new("grep")
        .arg("mongod")
        .stdin(Stdio::from(ps_child.stdout.unwrap())) // Pipe through.
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let grep_child_two = Command::new("grep")
        .arg("-v")
        .arg("grep")
        .stdin(Stdio::from(grep_child_one.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let output = grep_child_two.wait_with_output().unwrap();
    let result = str::from_utf8(&output.stdout).unwrap();
    println!("{}", result);
}
