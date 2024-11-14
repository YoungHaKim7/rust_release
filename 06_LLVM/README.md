# Mentorship Session: Using Clang and LLVM to Build the Linux Kernel | The Linux Foundation
- https://youtu.be/y458QwEfSsU?si=2BPk9Q2EUfqWYPmP
  - https://nathanchance.dev/

# What is LLVM?(3min 41sec)

||LLVM|GNU|
|-|-|-|
|C compiler|clang|gcc|
|Assembler|"Integrated"|as|
|Linker|ld.lld|ld|
|Binary utilities|llvm-nm,<br />llvm-objcopy,<br />llvm-objdump,<br />llvm-strip, ...<br />|nv, objcopy,<br /> objdump,<br /> strip, ...<br />|

# Install 

- Debian /Ubuntu

```bash
apt install clang lld llvm
```

- or apt.llvm.org for newer releases(https://apt.llvm.org/)

# clang
• C language frontend for LLVM (other language frontends exist, such as rustc)
Advertised as generally drop-in compatible with GCC
• Somewhat the case in reality, some flags may not be implemented, so kernel handles this in a few different ways
• Multi-targeted binary, change target with - target instead of separate binaries
clang --target=aarch64-linux-gnu vs. aarch64-linux-gnu-gcc
• Takes C files, generates LLVM IR (intermediate representation), mutates that through a series of optimization passes, then hands it off for a target specific backend to perform target specific optimizations and actually generate the machine code


# Id.Ild
*
•
LLVM's linker
Advertised as being compatible with the GNU linker (in a similar manner as clang)
Multi-targeted binary like clang
Generally faster at linking than GNU Id, especially when debug information is involved
