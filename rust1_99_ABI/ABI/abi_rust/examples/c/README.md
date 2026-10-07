# Result


```bash
$ gcc main.c \
      -I../../include \
      -L../../target/release \
      -labi_rust \
       -o app

$ LD_LIBRARY_PATH=../../target/release ./app
30
```
