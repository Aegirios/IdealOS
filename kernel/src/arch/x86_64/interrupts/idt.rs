use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::arch::x86_64::cpu::tss::{DOUBLE_FAULT_IST_INDEX, NMI_IST_INDEX};
use crate::arch::x86_64::halt::halt_loop;
use crate::debug::log::Logger;
use super::pic;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

/// appelé une seule fois avant sti
pub unsafe fn init() {
    Logger::log("≺IDT≻ Building...");

    IDT.divide_error.set_handler_fn(ex_divide_error);
    IDT.debug.set_handler_fn(ex_debug);
    IDT.non_maskable_interrupt.set_handler_fn(ex_nmi);
    IDT.breakpoint.set_handler_fn(ex_breakpoint);
    IDT.overflow.set_handler_fn(ex_overflow);
    IDT.bound_range_exceeded.set_handler_fn(ex_bound_range);
    IDT.invalid_opcode.set_handler_fn(ex_invalid_opcode);
    IDT.device_not_available.set_handler_fn(ex_device_na);
    IDT.double_fault
        .set_handler_fn(ex_double_fault)
        .set_stack_index(DOUBLE_FAULT_IST_INDEX);
    IDT.invalid_tss.set_handler_fn(ex_invalid_tss);
    IDT.segment_not_present.set_handler_fn(ex_seg_not_present);
    IDT.stack_segment_fault.set_handler_fn(ex_stack_seg);
    IDT.general_protection_fault.set_handler_fn(ex_gpf);
    IDT.page_fault.set_handler_fn(ex_page_fault);
    IDT.x87_floating_point.set_handler_fn(ex_x87);
    IDT.alignment_check.set_handler_fn(ex_alignment);
    IDT.machine_check.set_handler_fn(ex_machine_check);
    IDT.simd_floating_point.set_handler_fn(ex_simd);
    IDT.virtualization.set_handler_fn(ex_virt);
    IDT.security_exception.set_handler_fn(ex_security);

    // NMI sur pile IST dédiée
    IDT.non_maskable_interrupt
        .set_handler_fn(ex_nmi)
        .set_stack_index(NMI_IST_INDEX);

    IDT[pic::IRQ_TIMER].set_handler_fn(irq_timer);
    IDT[pic::IRQ_KEYBOARD].set_handler_fn(irq_keyboard);
    IDT[pic::IRQ_CASCADE].set_handler_fn(irq_spurious);
    IDT[pic::IRQ_SPURIOUS].set_handler_fn(irq_spurious);

    Logger::log("≺IDT≻ lidt...");
    IDT.load();
    Logger::log("≺IDT≻ OK");
}

macro_rules! ex {
    ($name:ident, $msg:literal) => {
        extern "x86-interrupt" fn $name(_f: InterruptStackFrame) {
            Logger::log(concat!("≺EX≻ ", $msg));
            halt_loop();
        }
    };
    ($name:ident, $msg:literal, ec) => {
        extern "x86-interrupt" fn $name(_f: InterruptStackFrame, _c: u64) {
            Logger::log(concat!("≺EX≻ ", $msg));
            halt_loop();
        }
    };
}

ex!(ex_divide_error, "#DE");
ex!(ex_debug, "#DB");
ex!(ex_overflow, "#OF");
ex!(ex_bound_range, "#BR");
ex!(ex_invalid_opcode,"#UD");
ex!(ex_device_na, "#NM");
ex!(ex_x87, "#MF");
ex!(ex_simd, "#XF");
ex!(ex_virt, "#VE");
ex!(ex_invalid_tss, "#TS", ec);
ex!(ex_seg_not_present,"#NP", ec);
ex!(ex_stack_seg, "#SS", ec);
ex!(ex_gpf, "#GP", ec);
ex!(ex_alignment, "#AC", ec);
ex!(ex_security, "#SX", ec);

extern "x86-interrupt" fn ex_nmi(_f: InterruptStackFrame) {
    Logger::log("≺EX≻ NMI");
    halt_loop();
}

extern "x86-interrupt" fn ex_breakpoint(_f: InterruptStackFrame) {
    Logger::log("≺EX≻ #BP breakpoint");
    // ne pas halter
}

extern "x86-interrupt" fn ex_double_fault(_f: InterruptStackFrame, _c: u64) -> ! {
    Logger::log("≺eX≻ #DF DOUBLE FAULT");
    halt_loop()
}

extern "x86-interrupt" fn ex_machine_check(_f: InterruptStackFrame) -> ! {
    Logger::log("≺EX≻ #MC Machine Check");
    halt_loop()
}

extern "x86-interrupt" fn ex_page_fault(_f: InterruptStackFrame, _c: PageFaultErrorCode) {
    Logger::log("≺EX≻ #PF Page Fault");
    halt_loop();
}

extern "x86-interrupt" fn irq_timer(_f: InterruptStackFrame) {
    pic::end_of_interrupt(pic::IRQ_TIMER);
}

extern "x86-interrupt" fn irq_keyboard(_f: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let _sc: u8 = unsafe {
        Port::new(0x60).read()
    };
    pic::end_of_interrupt(pic::IRQ_KEYBOARD);
}

extern "x86-interrupt" fn irq_spurious(_f: InterruptStackFrame) {}