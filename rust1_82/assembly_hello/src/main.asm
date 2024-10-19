section .text
global  _start

_start:
    mov rax, 1 ; mov rax, WRITE_SYSCALL // rax holds the syscall number

    mov rdi, 1 ; mov rdi, STDOUT_HANDLE  // rdi is `fd` (first argument)

    mov rdx, 13 ; mov rdx, MSG.len()     // rdx is `count` (third argument)

    mov rsi, msg ; mov rsi, MSG           // rsi is `buf *` (second argument)

    syscall ; syscall            // invoke the syscall

    mov rax, 60 ; exit
    xor rdi, rdi
    syscall

section .data

msg:
    db "Hello, world!\n", 0
