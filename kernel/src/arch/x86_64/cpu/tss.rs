use lazy_static::lazy_static;

use x86_64::{
    structures::tss::TaskStateSegment,
};

use super::{
    percpu::CpuLocal,
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    pub static ref CPU_LOCAL: CpuLocal = CpuLocal::new();

    pub static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // pour les passages inter rings
        tss.privilege_stack_table[0] =
            CPU_LOCAL.kernel_stack.top();

        tss.interrupt_stack_table[
            DOUBLE_FAULT_IST_INDEX as usize
        ] = CPU_LOCAL.double_fault_stack.top();

        tss
    };
}