use super::stack::Stack;

pub struct CpuLocal {
    pub kernel_stack: Stack,
    pub double_fault_stack: Stack,
}

impl CpuLocal {
    pub const fn new() -> Self {
        Self {
            kernel_stack: Stack::new(),
            double_fault_stack: Stack::new(),
        }
    }
}