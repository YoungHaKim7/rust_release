# cargo.toml에 대한 모든 글

https://doc.rust-lang.org/cargo/index.html

- cargo add
  - https://doc.rust-lang.org/cargo/commands/cargo-add.html

<hr>

# cargo add 1개 편하게 넣기

```bash
cargo add eframe -F "__screenshot"
    Updating crates.io index
      Adding eframe v0.22.0 to dependencies.
             Features:
             + __screenshot
             + accesskit
             + default_fonts
             + glow
             - android-game-activity
             - android-native-activity
             - directories-next
             - document-features
             - persistence
             - puffin
             - ron
             - serde
             - tts
             - wayland
             - web_screen_reader
             - wgpu
    Updating crates.io index
```

- cargo.toml

```toml
[package]
name = "egui_window"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
eframe = { version = "0.22.0", features = ["__screenshot"] }

```

# Rust _____cargo add 2개 동시에 넣고 Features에 2개 동시에 추가하기 편하다.!!

```
$ cargo add serde tokio -F serde/derive -F tokio/full

    Updating crates.io index
      Adding serde v1.0.159 to dependencies.
             Features:
             + derive
             + serde_derive
             + std
             - alloc
             - rc
             - unstable
      Adding tokio v1.27.0 to dependencies.
             Features:
             + bytes
             + fs
             + full
             + io-std
             + io-util
             + libc
             + macros
             + net
             + num_cpus
             + parking_lot
             + process
```

https://youtu.be/S-O9QkrlfYw

## 다른 방법

```
$ cargo add tokio --features macros,rt-multi-thread
```

```
$ cargo add axum
```

https://docs.rs/axum/0.6.14/axum/


# cargo add dev-Dependencies에 넣는 방법


```
$ cargo add tokio --dev
```

```
[dev-dependencies]
tokio = "1.28.2"
```

- F와 --dev활용법

```
$ cargo add tokio --dev -F tokio/full


[dev-dependencies]
tokio = { version = "1.28.2", features = ["full"] }

```

https://doc.rust-lang.org/cargo/commands/cargo-add.html

https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies

# ```cargo tree```

```bash
  

$ cargo tree

tsoding_4at_simple_multi_User_Chat v0.1.0 (/home/gy/my_project/rust/111222/tsoding_4at_simple_multi_User_Chat)
└── crossterm v0.27.0
    ├── bitflags v2.4.1
    ├── libc v0.2.150
    ├── mio v0.8.10
    │   ├── libc v0.2.150
    │   └── log v0.4.20
    ├── parking_lot v0.12.1
    │   ├── lock_api v0.4.11
    │   │   └── scopeguard v1.2.0
    │   │   [build-dependencies]
    │   │   └── autocfg v1.1.0
    │   └── parking_lot_core v0.9.9
    │       ├── cfg-if v1.0.0
    │       ├── libc v0.2.150
    │       └── smallvec v1.11.2
    ├── signal-hook v0.3.17
    │   ├── libc v0.2.150
    │   └── signal-hook-registry v1.4.1
    │       └── libc v0.2.150
    └── signal-hook-mio v0.2.3
        ├── libc v0.2.150
        ├── mio v0.8.10 (*)
        └── signal-hook v0.3.17 (*)

```


# rustup 활용법 & 업데이드 및 기타etc

https://github.com/YoungHaKim7/rust_release
