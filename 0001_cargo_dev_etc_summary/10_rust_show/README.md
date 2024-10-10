# Rustup show

- ```rustup show```

```
PS D:\rust_toolchain_toml> rustup show
Default host: x86_64-pc-windows-msvc
rustup home:  C:\Users\user\.rustup

installed toolchains
--------------------

stable-x86_64-pc-windows-msvc (default)
nightly-2023-02-21-x86_64-pc-windows-msvc
1.65.0-x86_64-pc-windows-msvc
1.68.0-x86_64-pc-windows-msvc

active toolchain
----------------

1.68.0-x86_64-pc-windows-msvc (overridden by 'D:\rust-toolchain.toml')
rustc 1.68.0 (2c8cc3432 2023-03-06)
```

# rustup toolchain remove nightly-2023-02-21 1.65.0(필요없는거 지우기)

- ```rustup toolchain remove 1.65.0```

```
PS D:\rust_toolchain_toml> rustup toolchain remove nightly-2023-02-21 1.65.0

info: uninstalling toolchain 'nightly-2023-02-21-x86_64-pc-windows-msvc'
info: toolchain 'nightly-2023-02-21-x86_64-pc-windows-msvc' uninstalled
info: uninstalling toolchain '1.65.0-x86_64-pc-windows-msvc'
info: toolchain '1.65.0-x86_64-pc-windows-msvc' uninstalled

PS D:\rust_toolchain_toml> rustup show
Default host: x86_64-pc-windows-msvc
rustup home:  C:\Users\user\.rustup

installed toolchains
--------------------

stable-x86_64-pc-windows-msvc (default)
1.68.0-x86_64-pc-windows-msvc

active toolchain
----------------

1.68.0-x86_64-pc-windows-msvc (overridden by 'D:\rust-toolchain.toml')
rustc 1.68.0 (2c8cc3432 2023-03-06)


```

- rustup show
```
rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/gy/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu (default)
nightly-x86_64-unknown-linux-gnu

installed targets for active toolchain
--------------------------------------

wasm32-unknown-unknown
x86_64-apple-ios
x86_64-unknown-linux-gnu

active toolchain
----------------

stable-x86_64-unknown-linux-gnu (default)
rustc 1.75.0 (82e1608df 2023-12-21)


```

- ```rustup target remove x86_64-apple-ios```

```
rustup target remove x86_64-apple-ios   
info: removing component 'rust-std' for 'x86_64-apple-ios'

```
