# cargo-binutils 

- https://github.com/rust-embedded/cargo-binutils

```

$ cargo size --example foo

```

- ```cargo nm```

```
$ cargo nm --release
0800040a T BusFault
0800040a T DebugMonitor
0800040a T DefaultHandler
0800065e T HardFault
0800040a T MemoryManagement
0800040a T NonMaskableInt
0800040a T PendSV
0800040c T Reset
0800040a T SVCall
0800040a T SysTick
0800040a T UsageFault
08000408 T UserHardFault
08000008 R __EXCEPTIONS
08000040 R __INTERRUPTS
08000004 R __RESET_VECTOR
08000000 R __STACK_START
```

- ```cargo objdump --release -- --disassemble --no-show-raw-insn```

```

$ cargo objdump --release -- --disassemble --no-show-raw-insn
target/thumbv7m-none-eabi/debug/app:    file format ELF32-arm-little

Disassembly of section .text:
main:
 8000400:       push    {r7, lr}
 8000402:       bl      #608
 8000406:       b       #-8 <main+0x2>

UserHardFault:
 8000408:       trap

UsageFault:
 800040a:       trap

Reset:
 800040c:       push.w  {r4, r5, r6, r7, r8, lr}
 8000410:       movw    r0, #0
 8000414:       movw    r2, #0
 8000418:       movt    r0, #8192
 800041c:       movt    r2, #8192
(..)  
```

- ```cargo size```

```

$ cargo size --release -- -A -x
target/thumbv7m-none-eabi/release/app  :
section               size         addr
.vector_table        0x400    0x8000000
.text                0x26a    0x8000400
.rodata                0x2    0x800066a
.data                    0   0x20000000
.bss                     0   0x20000000
.debug_str          0x107e            0
.debug_loc           0x3e2            0
.debug_abbrev        0x31b            0
.debug_info         0x19f9            0
.debug_ranges         0xe8            0
.debug_macinfo         0x1            0
.debug_pubnames      0x9ff            0
.debug_pubtypes      0x8dd            0
.ARM.attributes       0x2e            0
.debug_frame          0x6c            0
.debug_line          0x69b            0
.debug_aranges        0x40            0
Total               0x531a
```

# xxd-rs

- https://github.com/Nicoretti/xxd-rs

```
git clone https://github.com/Nicoretti/xxd-rs

cd xxd-rs

cargo install --path .
  
```
