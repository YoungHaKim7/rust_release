# Result

```

borrow of partially moved value: `array`
  --> src/main.rs:13:30
   |
10 |         [_, y] => {}
   |             - value partially moved here
...
13 |     println!("array : {:?}", array);
   |                              ^^^^^ value borrowed here after partial move
   |
   = note: partial move occurs because `array[..]` has type `Widget`, which does not implement the `Copy` trait
   = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
```

```bash
array : [Widget(1), Widget(2)]
```



