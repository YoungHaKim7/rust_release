# Result

```rs

    /// let x = String::from("bucket");
    /// let static_ref: &'static mut str = x.leak();
    /// assert_eq!(static_ref, "bucket");
    //#[stable(feature = "string_leak", since = "1.72.0")]
    // #[inline]
    pub fn leak<'a>(self) -> &'a mut str {
        let slice = self.vec.leak();
        unsafe { from_utf8_unchecked_mut(slice) }
    }

```

```bash
static_ref : bucket
```
