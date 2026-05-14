pub mod tss;
mod gdt;
mod stack;
mod percpu;

pub fn init() {
    gdt::init();
}