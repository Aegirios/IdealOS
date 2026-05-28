use x86_64::instructions::port::Port;

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_ICW4: u8 = 0x01;
const ICW1_INIT: u8 = 0x10;
const ICW4_8086: u8 = 0x01;
// Bon plus besoin d'annoter, je comprendsbien le hexadécimal finalement

pub const PIC1_VECTOR_BASE: u8 = 0x20; // 32
pub const PIC2_VECTOR_BASE: u8 = 0x28; // 40
//Finalement, non...
pub const IRQ_TIMER:u8 = PIC1_VECTOR_BASE; // 32
pub const IRQ_KEYBOARD:u8 = PIC1_VECTOR_BASE + 1; // 33
pub const IRQ_CASCADE:u8 = PIC1_VECTOR_BASE + 2; // 34
pub const IRQ_SPURIOUS:u8 = PIC2_VECTOR_BASE + 7; // 47

pub fn init() {
    unsafe {
        // Partie générée par Claude ; à revoir, réécrire
        Port::<u8>::new(PIC1_COMMAND).write(ICW1_INIT | ICW1_ICW4);
        io_wait();
        Port::<u8>::new(PIC2_COMMAND).write(ICW1_INIT | ICW1_ICW4);
        io_wait();

        Port::<u8>::new(PIC1_DATA).write(PIC1_VECTOR_BASE);
        io_wait();
        Port::<u8>::new(PIC2_DATA).write(PIC2_VECTOR_BASE);
        io_wait();

        Port::<u8>::new(PIC1_DATA).write(0b0000_0100);
        io_wait();
        Port::<u8>::new(PIC2_DATA).write(0x02);
        io_wait();

        Port::<u8>::new(PIC1_DATA).write(ICW4_8086);
        io_wait();
        Port::<u8>::new(PIC2_DATA).write(ICW4_8086);
        io_wait();

        // Activter timer en bit 0 ou clavier en bit 1 SEULEMENT
        Port::<u8>::new(PIC1_DATA).write(0b1111_1100);
        Port::<u8>::new(PIC2_DATA).write(0b1111_1111);
    }
}

pub fn end_of_interrupt(irq: u8) {
    unsafe {
        if irq >= 8 {
            Port::<u8>::new(PIC2_COMMAND).write(0x20);
        }
        Port::<u8>::new(PIC1_COMMAND).write(0x20);
    }
}

#[allow(dead_code)]
pub fn disable() {
    unsafe {
        Port::<u8>::new(PIC1_DATA).write(0xFF);
        Port::<u8>::new(PIC2_DATA).write(0xFF);
    }
}

#[inline]
unsafe fn io_wait() {
    Port::<u8>::new(0x80).write(0);
}