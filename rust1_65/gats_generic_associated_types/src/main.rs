trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct WindowsMut<'t, T> {
    slice: &'t mut [T],
    start: usize,
    windows_size: usize,
}

impl<'t, T> Iterator for WindowsMut<'t, T> {
    type Item = &'t mut [T];

    fn next<'a>(&'a mut self) -> Option<Self::Item> {
        let retrval = self.slice[self.start..].get_mut(..self.windows_size)?;
        self.start += 1;
        Some(retrval)
    }
}

fn main() {
    println!("Hello, world!");
}
