# Result

```bash
$ just r

rm -rf target
mkdir -p target
nasm -f elf64 ./src/main.asm -o ./target/main.o
./src/main.asm:1: warning: label alone on a line without a colon might be in er
ror [-w+label-orphan]
./src/main.asm:25: warning: label alone on a line without a colon might be in e
rror [-w+label-orphan]
ld -o ./target/main ./target/main.o
./target/main



Hello, world!¶

```
