use std::process::Command;

fn main() {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let hello = output.stdout;

    // [104, 101, 108, 108, 111, 10]
    println!("{:?}", hello);
    let convert_str = str::from_utf8(&hello);

    println!("{:?}", convert_str);
}
