#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct TaskContext {
    // Registres généraux ; sauvefardés par le prologue asm
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub rbp: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub rbx: u64,
    pub rax: u64,
    // poussés par le cpu pendant l'interrupt
    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub kernel_rsp: u64,
    pub stack_top: u64,
}

impl TaskContext {
    pub fn new_kernel(entry: u64, stack_top: u64) -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0,
            r8:  0, r9:  0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip:entry,
            cs: 0x08,
            // IFà 1 interruptions activées, reserved bits corrects
            rflags: 0x200,
            kernel_rsp: 0,
            // Sélecteur data kernel ring 0 index 2
            stack_top,
        }
    }
}