// https://internals.rust-lang.org/t/is-it-possible-to-be-memory-safe-with-deallocated-self/8457/4
// https://github.com/rust-lang/rust/issues/55005
// Rust 1.64
use atomic::Ordering::Release;
fn use_n(n: usize) { /* omitted */
}

struct Foo {
    n: usize,
}

impl Foo {
    // may cause deallocation of`*self`
    fn do_dirty_work(&mut self) { /* omitted */
    }

    fn do_work(&mut self) {
        let n = self.n;
        self.do_dirty_work();
        use_n(self.n); // instead of use_n( self.n );
    }
}

// Arc::Drop contains this code:
//     if self.inner().strong.fetch_sub(1, Release) != 1 {
//     return;
// }

fn main() {}
