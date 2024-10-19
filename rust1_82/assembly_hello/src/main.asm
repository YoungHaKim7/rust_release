section.text
global _start

_start:
    ; mov rax, WRITE_SYSCALL // rax holds the syscall number
    mov rax, 1

    ; mov rdi, STDOUT_HANDLE  // rdi is `fd` (first argument)
    mov rdi, 1

    ; mov rdx, MSG.len()     // rdx is `count` (third argument)
    mov rdx, 13

    ; mov rsi, MSG           // rsi is `buf *` (second argument)
    mov rsi, msg

    ; syscall            // invoke the syscall
    syscall

    ; exit
    mov rax, 60
    xor rdi, rdi
    syscall

section.data
msg:
    db "Hello, world!\n", 0
    
