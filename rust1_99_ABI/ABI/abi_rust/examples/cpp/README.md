# Result


```bash
$ g++ main.cpp -std=c++2b \
      -I../../include \
      -L../../target/release \
      -labi_rust \
       -o app

$ LD_LIBRARY_PATH=../../target/release ./app
30
```
