use x86_64::VirtAddr;

pub const STACK_SIZE: usize = 4096 * 5;

#[repr(align(16))]
pub struct Stack {
    bytes: [u8; STACK_SIZE],
}

impl Stack {
    pub const fn new() -> Self {
        Self {
            bytes: [0; STACK_SIZE],
        }
    }

    pub fn top(&self) -> VirtAddr {
        let start = VirtAddr::from_ptr(self.bytes.as_ptr());
        start + STACK_SIZE as u64
    }
}