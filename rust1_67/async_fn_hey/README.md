# A must_use attribute on async functions now does what you expect.

#[must_use]
async fn hey() -> Thing { … }

Now, #[must_use] applies to `Thing`, instead of to the `impl Future<Output=Thing>` (which wasn't very useful).
