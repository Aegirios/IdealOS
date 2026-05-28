pub mod idt;
pub mod pic;

use crate::debug::log::Logger;

pub fn init() {
    Logger::log("≺PIC≻ Remapping IRQs...");
    pic::init();
    Logger::log("≺PIC≻ OK");

    unsafe { idt::init(); }

    x86_64::instructions::interrupts::enable();
    Logger::log("≺INT≻ sti ; interrupts enabled");
}