use super::stack::Stack;

pub struct CpuLocal {
    /// Pile utilisée en transition ring 3 to 0 ; RSP0 du TSS
    pub kernel_stack: Stack,
    /// Pile pour le handler double faut, IST slot 0
    pub double_fault_stack: Stack,
    /// Pile pour le handler nmi, IST slot 1
    pub nmi_stack: Stack,
}

impl CpuLocal {
    pub const fn new() -> Self {
        Self {
            kernel_stack: Stack::new(),
            double_fault_stack: Stack::new(),
            nmi_stack: Stack::new(),
        }
    }
}