use std::sync::OnceLock;

static CELL: OnceLock<usize> = OnceLock::new();

fn main() {
    // `OnceLock` has not been written to yet.
    assert!(CELL.get().is_none());

    // Spawn a thread and write to `OnceLock`.
    std::thread::spawn(|| {
        let value = CELL.get_or_init(|| 12345);
        assert_eq!(value, &12345);
    })
    .join()
    .unwrap();

    // `OnceLock` now contains the value.
    assert_eq!(CELL.get(), Some(&12345),);
}
