# Source
- https://medium.com/@maturationofthe/asynchronous-programming-in-rust-with-tokio-23917a3ad6ae

# `cargo info`

- https://crates.io/crates/cargo-info
  - Install

```bash
cargo install cargo-info
```

```bash
$ cargo info tokio

Crate:          tokio
Version:        1.40.0
Description:    An event-driven, non-blocking I/O platform for writing asynchro
nous I/O
backed applications.

Downloads:      224948536
Homepage:       https://tokio.rs
Documentation:
Repository:     https://github.com/tokio-rs/tokio
Updated:        Fri Aug 30 08:04:01 2024    (a month ago)
Version history:

  VERSION  RELEASED        DOWNLOADS

  1.40.0   a month ago     4508833
  1.39.3   2 months ago    2176895
  1.39.2   2 months ago    5488472
  1.39.1   2 months ago    795907
  1.39.0   2 months ago    35432                (yanked)

  ... use -VV to show all 157 versions


# cargo winit

$ cargo info winit
Crate:          winit
Version:        0.30.5
Description:    Cross-platform window creation library.
Downloads:      12252300
Homepage:
Documentation:  https://docs.rs/winit
Repository:     https://github.com/rust-windowing/winit
Updated:        Thu Aug  8 18:27:20 2024    (2 months ago)
Version history:

  VERSION  RELEASED        DOWNLOADS

  0.30.5   2 months ago    270856
  0.30.4   2 months ago    97117
  0.30.3   3 months ago    82975
  0.30.2   3 months ago    16314
  0.30.1   4 months ago    8503

  ... use -VV to show all 109 versions


# 다 나오게 하기
$ cargo info winit -VV
VERSION        RELEASED        DOWNLOADS

0.30.5         2 months ago    270856
0.30.4         2 months ago    97117
0.30.3         3 months ago    82975
0.30.2         3 months ago    16314
0.30.1         4 months ago    8503
```


# `cargo-tree`
- https://doc.rust-lang.org/cargo/commands/cargo-tree.html

```bash
$ cargo tree
cargo_info_test v0.1.0 (/home/y111/rust_release/0001_cargo_dev_etc_summary/cargo_info_test)
└── tokio v1.40.0
    ├── bytes v1.7.2
    ├── libc v0.2.159
    ├── mio v1.0.2
    │   └── libc v0.2.159
    ├── parking_lot v0.12.3
    │   ├── lock_api v0.4.12
    │   │   └── scopeguard v1.2.0
    │   │   [build-dependencies]
    │   │   └── autocfg v1.4.0
    │   └── parking_lot_core v0.9.10
    │       ├── cfg-if v1.0.0
    │       ├── libc v0.2.159
    │       └── smallvec v1.13.2
    ├── pin-project-lite v0.2.14
    ├── signal-hook-registry v1.4.2
    │   └── libc v0.2.159
    ├── socket2 v0.5.7
    │   └── libc v0.2.159
    └── tokio-macros v2.4.0 (proc-macro)
        ├── proc-macro2 v1.0.86
        │   └── unicode-ident v1.0.13
        ├── quote v1.0.37
        │   └── proc-macro2 v1.0.86 (*)
        └── syn v2.0.79
            ├── proc-macro2 v1.0.86 (*)
            ├── quote v1.0.37 (*)

```

# `cargo vendor`
- 인터넷이 안되는 환경에서 미리 받아 놓기 로컬에서 실행
  - vendor라는 폴더 생성되서 그 안에 미리 받아 놓음

```

.
|
-- vendor

    
```

