use x86_64::VirtAddr;

pub const STACK_SIZE: usize = 4096 * 5; // 20 KB

/// Pile alignée 16 octets, stockée en BSS
#[repr(C, align(16))]
pub struct Stack {
    bytes: [u8; STACK_SIZE],
}

impl Stack {
    pub const fn new() -> Self {
        Self { bytes: [0; STACK_SIZE] }
    }

    /// Adresse du sommet
    #[inline]
    pub fn top(&self) -> VirtAddr {
        VirtAddr::from_ptr(self.bytes.as_ptr()) + STACK_SIZE as u64
    }
}