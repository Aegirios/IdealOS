#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

use bootloader_api::{entry_point, BootInfo, BootloaderConfig};
use bootloader_api::config::Mapping;
use kernel::debug::log::Logger;
use kernel::arch::x86_64::halt::halt_loop;

// Configuration bootloader embarquée dans le binaire kernel demandant explicitement le mapping phsique complet
static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    Logger::init();
    Logger::log("⟾ ✵✵✵⨑ µKernel ∱✵✵✵ ⟽");
    // Bordel, y'a tellement de caractères utf 8
    Logger::log("≺BOOT≻ Serial OK");

    unsafe { kernel::arch::x86_64::cpu::init(); }

    Logger::log("≺BOOT≻ Starting MM...");
    kernel::mm::init(boot_info);
    Logger::log("≺BOOT≻ MM OK");

    Logger::log("≺BOOT≻ Starting interrupts...");
    kernel::arch::x86_64::interrupts::init();
    Logger::log("≺BOOT≻ Interrupts OK");

    Logger::log("≺BOOT≻ Running self-tests...");
    self_test();

    Logger::log("≺BOOT≻ All systems nominal ; idle");
    halt_loop();
}

fn self_test() {
    use kernel::capabilities::{self, CapabilityKind, Rights};
    use kernel::ipc;

    let cap = capabilities::create(CapabilityKind::Service { service_id: 1 }, Rights::READ, 2);
    assert!(cap.is_some());
    let id = cap.unwrap();
    assert!(capabilities::check(id, Rights::READ));
    assert!(!capabilities::check(id, Rights::WRITE));
    capabilities::revoke(id);
    assert!(!capabilities::check(id, Rights::READ));

    assert!(ipc::create_endpoint().is_some());

    Logger::log("≺SELFTEST≻ OK");
}