# link

<hr />

- You then distribute:

```text
macOS
    librust_ffi.dylib

Linux
    librust_ffi.so

Windows
    rust_ffi.dll
```

# ABI
- Yes. The right approach is to package the three native libraries behind **one stable C-compatible API**, so C, C++, Python, Go, Swift, etc. can consume the same Rust library.

```text
rust-ffi/
├── Cargo.toml
├── src/
│   └── lib.rs
├── include/
│   └── rust_ffi.h
├── target/
│   └── release/
│       ├── macOS/
│       │   └── librust_ffi.dylib
│       ├── linux/
│       │   └── librust_ffi.so
│       └── windows/
│           └── rust_ffi.dll
└── examples/
    ├── c/
    ├── cpp/
    ├── python/
    └── go/
```

### 1. Rust API

```rust
// src/lib.rs

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_sub(a: i32, b: i32) -> i32 {
    a - b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_mul(a: i32, b: i32) -> i32 {
    a * b
}
```

`Cargo.toml`:

```toml
[package]
name = "rust_ffi"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]
```

Build on each platform:

```bash
cargo build --release
```

You then distribute:

```text
macOS
    librust_ffi.dylib

Linux
    librust_ffi.so

Windows
    rust_ffi.dll
```

---

## 2. One header for every language

Create:

```c
// include/rust_ffi.h

#ifndef RUST_FFI_H
#define RUST_FFI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

int32_t rust_ffi_add(int32_t a, int32_t b);
int32_t rust_ffi_sub(int32_t a, int32_t b);
int32_t rust_ffi_mul(int32_t a, int32_t b);

#ifdef __cplusplus
}
#endif

#endif
```

Now your API is:

```text
                    rust_ffi.h
                       │
          ┌────────────┼────────────┐
          │            │            │
          ▼            ▼            ▼
          C           C++         Python
          │            │            │
          └────────────┼────────────┘
                       │
                       ▼
                 Rust library
                       │
          ┌────────────┼────────────┐
          ▼            ▼            ▼
       .dylib         .so         .dll
       macOS         Linux       Windows
```

---

# C

On Linux:

```c
#include <stdio.h>
#include "rust_ffi.h"

int main(void) {
    printf("%d\n", rust_ffi_add(10, 20));
    return 0;
}
```

Compile:

```bash
gcc main.c \
    -I../../include \
    -L../../target/release \
    -lrust_ffi \
    -o app
```

Run:

```bash
LD_LIBRARY_PATH=../../target/release ./app
```

Output:

```text
30
```

---

# C++

The exact same header works:

```cpp
#include <iostream>
#include "rust_ffi.h"

int main() {
    std::cout << rust_ffi_add(100, 200) << '\n';
}
```

The important part is:

```c
#ifdef __cplusplus
extern "C" {
#endif
```

Without this, C++ name mangling can prevent the linker from finding the Rust symbol.

---

# Python

Python can load the same Rust `.so` / `.dylib` / `.dll` directly.

```python
import ctypes
import platform

if platform.system() == "Darwin":
    lib = ctypes.CDLL("./librust_ffi.dylib")
elif platform.system() == "Linux":
    lib = ctypes.CDLL("./librust_ffi.so")
elif platform.system() == "Windows":
    lib = ctypes.CDLL("./rust_ffi.dll")
else:
    raise RuntimeError("Unsupported OS")

lib.rust_ffi_add.argtypes = [
    ctypes.c_int32,
    ctypes.c_int32,
]

lib.rust_ffi_add.restype = ctypes.c_int32

result = lib.rust_ffi_add(10, 20)

print(result)
```

Output:

```text
30
```

So:

```text
Python
   │
   │ ctypes
   ▼
┌──────────────────┐
│ Rust shared lib  │
│                  │
│ rust_ffi_add()   │
└──────────────────┘
```

---

# Go

Go can also load the library through C interoperability:

```go
package main

/*
#cgo CFLAGS: -I../../include
#cgo LDFLAGS: -L../../target/release -lrust_ffi

#include "rust_ffi.h"
*/
import "C"

import "fmt"

func main() {
    result := C.rust_ffi_add(10, 20)

    fmt.Println(result)
}
```

Linux:

```bash
LD_LIBRARY_PATH=../../target/release go run .
```

---

# The important part: distribute it as a package

Instead of giving users:

```text
librust_ffi.so
```

by itself, I recommend distributing:

```text
rust-ffi/
├── include/
│   └── rust_ffi.h
│
├── lib/
│   ├── macos/
│   │   └── librust_ffi.dylib
│   │
│   ├── linux/
│   │   └── librust_ffi.so
│   │
│   └── windows/
│       └── rust_ffi.dll
│
└── README.md
```

Then other languages have a **stable interface**:

```text
                    rust_ffi.h
                        │
                        ▼
              ┌──────────────────┐
              │   C ABI boundary │
              └────────┬─────────┘
                       │
                 Rust functions
                       │
          ┌────────────┼────────────┐
          │            │            │
       macOS         Linux       Windows
          │            │            │
       .dylib         .so         .dll
          │            │            │
          ▼            ▼            ▼
       Swift        Python        C/C++
                       │
                       ▼
                      Go
```

- The **ABI boundary** is the critical piece. The `.dylib`, `.so`, and `.dll` are merely different platform implementations of the same ABI.

- If your goal is to make this into something people can literally **`git clone`, build, and import from C/C++/Python/Go/Swift**, I can lay out the complete repository—including `build.rs`, CMake, Makefile, platform-specific library loading, versioning, and GitHub Releases.

# `rust_ffi.h`
Here is the complete `rust_ffi.h` for the Rust library API we defined:

```c
#ifndef RUST_FFI_H
#define RUST_FFI_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/*
 * Rust FFI API
 *
 * These functions are implemented in Rust and exported
 * using the C ABI.
 */

/* Add two 32-bit integers. */
int32_t rust_ffi_add(int32_t a, int32_t b);

/* Subtract b from a. */
int32_t rust_ffi_sub(int32_t a, int32_t b);

/* Multiply two 32-bit integers. */
int32_t rust_ffi_mul(int32_t a, int32_t b);

#ifdef __cplusplus
} /* extern "C" */
#endif

#endif /* RUST_FFI_H */
```

The corresponding Rust implementation is:

```rust
#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_sub(a: i32, b: i32) -> i32 {
    a - b
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_ffi_mul(a: i32, b: i32) -> i32 {
    a * b
}
```

The important ABI correspondence is:

```text
C header                         Rust

int32_t                          i32
   │                               │
   └──────────── C ABI ────────────┘

int32_t rust_ffi_add(...)  <──>  extern "C" fn rust_ffi_add(...)
```

And `extern "C"` in the header protects the functions from **C++ name mangling**, so the same header can be included by both C and C++:

```cpp
#include "rust_ffi.h"

int main() {
    return rust_ffi_add(10, 20);
}
```

The header itself does **not** contain the Rust implementation. It is the contract that tells another language exactly what functions and types the Rust shared library exports.

