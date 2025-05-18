use std::{io::Read, process::Command};

fn main() {
    let (mut recv, send) = std::io::pipe()?;

    let mut command = Command::new("path/to/bin")
        // Both stdout and stderr will write to the same pipe, combining the two.
        .stdout(send.try_clone()?)
        .stderr(send)
        .spawn()?;

    let mut output = Vec::new();
    recv.read_to_end(&mut output)?;

    // It's important that we read from the pipe before the process exits, to avoid
    // filling the OS buffers if the program emits too much output.
    assert!(command.wait()?.success());
}
