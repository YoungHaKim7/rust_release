# Result

- `clang`

```bash
# 젤 먼저
cargo build --release

$ clang main.c -std=c23 \
          -I../../include \
          -L../../target/release \
          -labi_rust \
          -o app

$ ls
app*       main.c     README.md

$ ./app
30
  
```
- gcc

```bash
$ gcc main.c \
      -I../../include \
      -L../../target/release \
      -labi_rust \
       -o app

$ LD_LIBRARY_PATH=../../target/release ./app
30
```
