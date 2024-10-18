use std::ffi::c_int;

const WRITE_SYSCALL: c_int = 0x01; // syscall 1 is `write`
const STDOUT_HANDLE: c_int = 0x01; // `stdout` has file handle 1
const MSG: &str = "Hello, world!\n";

fn main() {
    let written: usize;

    // Signature: `ssize_t write(int fd, const void buf[], size_t count)`
    unsafe {
        core::arch::asm!(
            "mov rax, {SYSCALL} // rax holds the syscall number",
            "mov rdi, {OUTPUT}  // rdi is `fd` (first argument)",
            "mov rdx, {LEN}     // rdx is `count` (third argument)",
            "syscall            // invoke the syscall",
            "mov {written}, rax // save the return value",
            SYSCALL = const WRITE_SYSCALL,
            OUTPUT = const STDOUT_HANDLE,
            LEN = const MSG.len(),
            in("rsi") MSG.as_ptr(), // rsi is `buf *` (second argument)
            written = out(reg) written,
        );
    }

    assert_eq!(written, MSG.len());
}
