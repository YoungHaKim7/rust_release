// This example uses `gen` blocks, introduced in this RFC.
fn rl_encode<I: IntoIterator<Item = u8>>(xs: I) -> impl Iterator<Item = u8> {
    gen {
        let mut xs = xs.into_iter();
        let (Some(mut cur), mut n) = (xs.next(), 0) else {
            return;
        };
        for x in xs {
            if x == cur && n < u8::MAX {
                n += 1;
            } else {
                yield n;
                yield cur;
                (cur, n) = (x, 0);
            }
        }
        yield n;
        yield cur;
    }
    .into_iter()
}

// This example uses a manual implementation of `Iterator`.
fn rl_encode_iterator<I: IntoIterator<Item = u8>>(xs: I) -> impl Iterator<Item = u8> {
    struct RlEncode<I: IntoIterator<Item = u8>> {
        into_xs: Option<I>,
        xs: Option<<I as IntoIterator>::IntoIter>,
        cur: Option<<I as IntoIterator>::Item>,
        n: u8,
        yield_x: Option<<I as IntoIterator>::Item>,
    }
    impl<I: IntoIterator<Item = u8>> Iterator for RlEncode<I> {
        type Item = u8;
        fn next(&mut self) -> Option<Self::Item> {
            let xs = self.xs.get_or_insert_with(|| unsafe {
                self.into_xs.take().unwrap_unchecked().into_iter()
            });
            if let Some(x) = self.yield_x.take() {
                return Some(x);
            }
            loop {
                match (xs.next(), self.cur) {
                    (Some(x), Some(cx)) if x == cx && self.n < u8::MAX => self.n += 1,
                    (Some(x), Some(cx)) => {
                        let n_ = self.n;
                        (self.cur, self.n) = (Some(x), 0);
                        self.yield_x = Some(cx);
                        return Some(n_);
                    }
                    (Some(x), None) => {
                        (self.cur, self.n) = (Some(x), 0);
                    }
                    (None, Some(cx)) => {
                        self.cur = None;
                        self.yield_x = Some(cx);
                        return Some(self.n);
                    }
                    (None, None) => return None,
                }
            }
        }
    }
    RlEncode {
        into_xs: Some(xs),
        xs: None,
        cur: None,
        n: 0,
        yield_x: None,
    }
}

// This example uses `iter::from_fn`.
fn rl_encode_iter_from_fn<I: IntoIterator<Item = u8>>(xs: I) -> impl Iterator<Item = u8> {
    let (mut cur, mut n, mut yield_x) = (None, 0, None);
    let (mut into_xs, mut xs) = (Some(xs), None);
    core::iter::from_fn(move || {
        loop {
            let xs =
                xs.get_or_insert_with(|| unsafe { into_xs.take().unwrap_unchecked().into_iter() });
            if let Some(x) = yield_x.take() {
                return Some(x);
            }
            match (xs.next(), cur) {
                (Some(x), Some(cx)) if x == cx && n < u8::MAX => n += 1,
                (Some(x), Some(cx)) => {
                    let n_ = n;
                    (cur, n) = (Some(x), 0);
                    yield_x = Some(cx);
                    return Some(n_);
                }
                (Some(x), None) => (cur, n) = (Some(x), 0),
                (None, Some(cx)) => {
                    cur = None;
                    yield_x = Some(cx);
                    return Some(n);
                }
                (None, None) => return None,
            }
        }
    })
}
fn main() {
    println!("Hello, world!");
}
