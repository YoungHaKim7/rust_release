# ```cargo objdump --release -- --disassemble --no-show-raw-insn```

- https://github.com/rust-embedded/cargo-binutils

```
cargo objdump --release -- --disassemble --no-show-raw-insn
   Compiling test_xxd_rs v0.1.0 (/home/gy/my_project/Rust_Lang/rust_release/11_rust_xxd_rs/test_xxd_rs)
    Finished release [optimized] target(s) in 0.11s

test_xxd_rs:	file format elf64-x86-64

Disassembly of section .init:

0000000000006000 <_init>:
    6000:      	endbr64
    6004:      	subq	$0x8, %rsp
    6008:      	movq	0x51ce9(%rip), %rax     # 0x57cf8 <_GLOBAL_OFFSET_TABLE_+0x368>
    600f:      	testq	%rax, %rax
    6012:      	je	0x6016 <_init+0x16>
    6014:      	callq	*%rax
    6016:      	addq	$0x8, %rsp
    601a:      	retq

Disassembly of section .plt:

0000000000006020 <.plt>:
    6020:      	pushq	0x51972(%rip)           # 0x57998 <_GLOBAL_OFFSET_TABLE_+0x8>
    6026:      	jmpq	*0x51974(%rip)          # 0x579a0 <_GLOBAL_OFFSET_TABLE_+0x10>
    602c:      	nopl	(%rax)

0000000000006030 <__tls_get_addr@plt>:
    6030:      	jmpq	*0x51972(%rip)          # 0x579a8 <_GLOBAL_OFFSET_TABLE_+0x18>
    6036:      	pushq	$0x0
    603b:      	jmp	0x6020 <.plt>
    
```

# ```xxd-rs dump test_xxd_rs```

```
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s

$ xxd-rs dump target/debug/test_xxd_rs

0039DDD0: e14b 1400 0000 0000 0000 0000 0000 0000  .K.............
0039DDE0: 0100 0000 0000 0000 0100 0000 0000 0000  ................
0039DDF0: 8901 0000 0100 0000 0000 0000 0000 0000  ................
0039DE00: 0000 0000 0000 0000 6bfb 2e00 0000 0000  ........k.......
0039DE10: d0e3 0900 0000 0000 0000 0000 0000 0000  ................
0039DE20: 0100 0000 0000 0000 0000 0000 0000 0000  ................
0039DE30: 0100 0000 0200 0000 0000 0000 0000 0000  ................
0039DE40: 0000 0000 0000 0000 40df 3800 0000 0000  ........@.8.....
0039DE50: 7044 0000 0000 0000 2600 0000 da01 0000  pD......&.......
0039DE60: 0800 0000 0000 0000 1800 0000 0000 0000  ...............
0039DE70: 0900 0000 0300 0000 0000 0000 0000 0000  ................
0039DE80: 0000 0000 0000 0000 b023 3900 0000 0000  .........#9.....
0039DE90: a3af 0000 0000 0000 0000 0000 0000 0000  ................
0039DEA0: 0100 0000 0000 0000 0000 0000 0000 0000  ................
0039DEB0: 1100 0000 0300 0000 0000 0000 0000 0000  ................
0039DEC0: 0000 0000 0000 0000 53d3 3900 0000 0000  ........S.9.....
0039DED0: 9701 0000 0000 0000 0000 0000 0000 0000  ................
0039DEE0: 0100 0000 0000 0000 0000 0000 0000 0000  ................

```
