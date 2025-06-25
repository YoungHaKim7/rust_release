const THREE: usize = 3;

// SAFETY: the validity of the used registers
// is guaranteed according to the "sysv64" ABI
#[unsafe(naked)]
pub extern "sysv64" fn add_n(number: usize) -> usize {
    core::arch::naked_asm!(
        "add rdi, {}",
        "mov rax, rdi",
        "ret",
        const THREE,
    )
}

fn main() {
    println!("{}", add_n(10));
}
