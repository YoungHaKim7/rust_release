# Manage cargo cache

- https://blog.rust-lang.org/2023/12/11/cargo-cache-cleaning.html

- manage cargo cache (${CARGO_HOME}, ~/.cargo/), print sizes of dirs and remove dirs selectively
  - https://github.com/matthiaskrgr/cargo-cache

- Install

```
cargo install cargo-cache
```


- ```cargo cache --autoclean```

```bash

$ cargo cache --autoclean
Clearing cache...

Cargo cache '/Users/globalyoung/.cargo':

Total:                                      2.75 GB => 1.82 GB
  61 installed binaries:                             372.34 MB
  Registry:                                 1.49 GB => 1.33 GB
    2 registry indices:                              688.84 MB
    5023 crate archives:                             639.58 MB
    101 => 0 crate source checkouts:         160.96 MB => 0  B
  Git db:                               891.26 MB => 121.41 MB
    18 bare git repos:                               121.41 MB
    19 => 0 git repo checkouts:              769.85 MB => 0  B

Size changed 2.75 GB => 1.82 GB (-930.80 MB, -33.81%)

```

- ```cargo cache```


```bash
$ cargo cache

Cargo cache '/Users/globalyoung/.cargo':

Total:                              2.75 GB
  61 installed binaries:          372.34 MB
  Registry:                         1.49 GB
    2 registry indices:           688.84 MB
    5023 crate archives:          639.58 MB
    101 crate source checkouts:   160.96 MB
  Git db:                         891.26 MB
    18 bare git repos:            121.41 MB
    19 git repo checkouts:        769.85 MB

```
