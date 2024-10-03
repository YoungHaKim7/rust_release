use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(path: &Path) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{contents}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let path = &Path::new("/etc/hosts");
        println!("{path:?}");
        read_file(path);
    }
}
