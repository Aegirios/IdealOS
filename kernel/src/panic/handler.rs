use crate::debug::log::Logger;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    Logger::log("≟≟≟ KERNEL PANIC ≟≟≟");
    Logger::log("(attach GDB on :1234 for full context)");
    loop {
        unsafe {
            core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}