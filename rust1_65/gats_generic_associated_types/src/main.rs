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

impl<'t, T> LendingIterator for WindowsMut<'t, T> {
    type Item<'a>
    where
        Self: 'a,
    = &'a mut [T];

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        let retrval = self.slice[self.start..].get_mut(..self.windows_size)?;
        self.start += 1;
        Some(retrval)
    }
}

fn main() {
    println!("Hello, world!");
}
