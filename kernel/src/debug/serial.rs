const SERIAL_PORT: u16 = 0x3F8;

pub struct Serial;

impl Serial {
    pub fn init() {
        unsafe {
            use x86_64::instructions::port::Port;
            Port::new(SERIAL_PORT + 1).write(0x00u8); // Désactiver interruptions
            Port::new(SERIAL_PORT + 3).write(0x80u8); // Enable DLAB
            Port::new(SERIAL_PORT + 0).write(0x03u8); // 38400 baud (DLL)
            Port::new(SERIAL_PORT + 1).write(0x00u8); // DLH
            Port::new(SERIAL_PORT + 3).write(0x03u8); // 8N1, DLAB off
            Port::new(SERIAL_PORT + 2).write(0xC7u8); // FIFO 14 byte trigger
            Port::new(SERIAL_PORT + 4).write(0x0Bu8); // OUT2 actif
        }
    }

    fn is_ready() -> bool {
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port: Port<u8> = Port::new(SERIAL_PORT + 5);
            (port.read() & 0x20) != 0
        }
    }

    pub fn write_byte(byte: u8) {
        while !Self::is_ready() {}
        unsafe {
            use x86_64::instructions::port::Port;
            let mut port = Port::new(SERIAL_PORT);
            port.write(byte);
        }
    }
}
