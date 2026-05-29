use super::context::TaskContext;
#[unsafe(naked)]
pub unsafe extern "C" fn switch_context(
    old_ctx: *mut TaskContext,
    new_ctx: *const TaskContext,
) {
    core::arch::naked_asm!(
        "mov [rdi + 0*8],  r15",
        "mov [rdi + 1*8],  r14",
        "mov [rdi + 2*8],  r13",
        "mov [rdi + 3*8],  r12",
        "mov [rdi + 4*8],  r11",
        "mov [rdi + 5*8],  r10",
        "mov [rdi + 6*8],  r9",
        "mov [rdi + 7*8],  r8",
        "mov [rdi + 8*8],  rbp",
        "mov [rdi + 14*8], rax",
        "mov rax, rdi",
        "mov [rax + 9*8],  rdi",
        "mov [rax + 10*8], rsi",
        "mov [rax + 11*8], rdx",
        "mov [rax + 12*8], rcx",
        "mov [rax + 13*8], rbx",

         "mov r15, [rsi + 0*8]",
        "mov r14, [rsi + 1*8]",
        "mov r13, [rsi + 2*8]",
        "mov r12, [rsi + 3*8]",
        "mov r11, [rsi + 4*8]",
        "mov r10, [rsi + 5*8]",
        "mov r9,  [rsi + 6*8]",
        "mov r8,  [rsi + 7*8]",
        "mov rbp, [rsi + 8*8]",
        "mov rdi, [rsi + 9*8]",
         "mov rdx, [rsi + 11*8]",
        "mov rcx, [rsi + 12*8]",
        "mov rbx, [rsi + 13*8]",
        "mov rax, [rsi + 14*8]",
        "mov rsi, [rsi + 10*8]",

        "ret",
    );
}