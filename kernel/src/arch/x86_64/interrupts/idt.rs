use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

use crate::arch::x86_64::cpu::tss::{DOUBLE_FAULT_IST_INDEX, NMI_IST_INDEX};
use crate::arch::x86_64::halt::halt_loop;
use crate::debug::log::Logger;
use super::pic;
use core::cell::UnsafeCell;

struct IdtWrapper(UnsafeCell<InterruptDescriptorTable>);
unsafe impl Sync for IdtWrapper {}

static IDT: IdtWrapper = IdtWrapper(UnsafeCell::new(InterruptDescriptorTable::new()));

/// appelé une seule fois avant sti
pub unsafe fn init() {
    Logger::log("≺IDT≻ Building...");
    let idt = &mut  *IDT.0.get();

    idt.divide_error.set_handler_fn(ex_divide_error);
    idt.debug.set_handler_fn(ex_debug);
    idt.non_maskable_interrupt.set_handler_fn(ex_nmi);
    idt.breakpoint.set_handler_fn(ex_breakpoint);
    idt.overflow.set_handler_fn(ex_overflow);
    idt.bound_range_exceeded.set_handler_fn(ex_bound_range);
    idt.invalid_opcode.set_handler_fn(ex_invalid_opcode);
    idt.device_not_available.set_handler_fn(ex_device_na);
    idt.double_fault
        .set_handler_fn(ex_double_fault)
        .set_stack_index(DOUBLE_FAULT_IST_INDEX);
    idt.invalid_tss.set_handler_fn(ex_invalid_tss);
    idt.segment_not_present.set_handler_fn(ex_seg_not_present);
    idt.stack_segment_fault.set_handler_fn(ex_stack_seg);
    idt.general_protection_fault.set_handler_fn(ex_gpf);
    idt.page_fault.set_handler_fn(ex_page_fault);
    idt.x87_floating_point.set_handler_fn(ex_x87);
    idt.alignment_check.set_handler_fn(ex_alignment);
    idt.machine_check.set_handler_fn(ex_machine_check);
    idt.simd_floating_point.set_handler_fn(ex_simd);
    idt.virtualization.set_handler_fn(ex_virt);
    idt.security_exception.set_handler_fn(ex_security);

    // NMI sur pile IST dédiée
    idt.non_maskable_interrupt
        .set_handler_fn(ex_nmi)
        .set_stack_index(NMI_IST_INDEX);

    idt[pic::IRQ_TIMER].set_handler_fn(irq_timer);
    idt[pic::IRQ_KEYBOARD].set_handler_fn(irq_keyboard);
    idt[pic::IRQ_CASCADE].set_handler_fn(irq_spurious);
    idt[pic::IRQ_SPURIOUS].set_handler_fn(irq_spurious);

    Logger::log("≺IDT≻ lidt...");
    idt.load();
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


extern "x86-interrupt" fn irq_timer(frame: InterruptStackFrame) {
    pic::end_of_interrupt(pic::IRQ_TIMER);

    if let Some((old_ctx, new_ctx)) = unsafe { crate::scheduler::on_tick() } {
        unsafe {
            (*old_ctx).rip = frame.instruction_pointer.as_u64();
            (*old_ctx).rsp = frame.stack_pointer.as_u64();
            (*old_ctx).rflags = frame.cpu_flags.bits();
            (*old_ctx).cs = frame.code_segment.0 as u64;
            crate::scheduler::switch::switch_context(old_ctx, new_ctx);
            // on ne peut pas modifier frame directement (x86-interrupt ABI) ; le vrai switch de RIP RSP se fera au prochain tick via le contexte
        }
    }
}

extern "x86-interrupt" fn irq_keyboard(_f: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    let _sc: u8 = unsafe {
        Port::new(0x60).read()
    };
    pic::end_of_interrupt(pic::IRQ_KEYBOARD);
}

extern "x86-interrupt" fn irq_spurious(_f: InterruptStackFrame) {}