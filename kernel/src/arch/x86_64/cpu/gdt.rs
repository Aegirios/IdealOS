use x86_64::{
    instructions::{
        segmentation::{Segment, CS},
        tables::load_tss,
    },
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    VirtAddr,
};
use core::cell::UnsafeCell;
use crate::debug::log::Logger;

struct Wrap<T>(UnsafeCell<T>);
unsafe impl<T> Sync for Wrap<T> {}

static GDT: Wrap<GlobalDescriptorTable> = Wrap(UnsafeCell::new(GlobalDescriptorTable::new()));
static SEL_KERNEL_CODE: Wrap<SegmentSelector> = Wrap(UnsafeCell::new(SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0)));
static SEL_TSS: Wrap<SegmentSelector> = Wrap(UnsafeCell::new(SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0)));

/// Indices des sélecteurs pour la GDT, en octets correspondant à l'append
pub const KERNEL_CODE_IDX: u16 = 1 * 8;
pub const KERNEL_DATA_IDX: u16 = 2 * 8;
pub const USER_DATA_IDX: u16 = 3 * 8 | 3;
pub const USER_CODE_IDX: u16 = 4 * 8 | 3;

/// Appelé une suele fois lors du boot, juste avant les interruptions
pub unsafe fn init() {
    Logger::log("≺GDT≻ Building table...");

    //On remplit les piles du TSS avant de créer le descripteur tss
    super::tss::init();
    Logger::log("≺GDT≻ TSS stacks initialized");

    let gdt = &mut *GDT.0.get();
    *SEL_KERNEL_CODE.0.get() = gdt.append(Descriptor::kernel_code_segment());
    let _ = gdt.append(Descriptor::kernel_data_segment());
    let _ = gdt.append(Descriptor::user_data_segment());
    let _ = gdt.append(Descriptor::user_code_segment());
    *SEL_TSS.0.get() = gdt.append(Descriptor::tss_segment(&super::tss::TSS));
    Logger::log("≺GDT≻ Descriptors appended");

    // Loading gdt ; chargement du gdt
    gdt.load();
    Logger::log("≺GDT≻ lgdt done");

    // Recharger cs
    CS::set_reg(*SEL_KERNEL_CODE.0.get());
    Logger::log("≺GDT≻ CS reloaded");

    // load tr
    load_tss( *SEL_TSS.0.get());
    Logger::log("≺GDT≻ TSS loaded ; ltr");

    Logger::log("≺GDT≻ OK");
}

pub fn kernel_code_selector() -> SegmentSelector {
    unsafe { *SEL_KERNEL_CODE.0.get() }
}