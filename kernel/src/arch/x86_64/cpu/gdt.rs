use x86_64::{
    instructions::{
        segmentation::{Segment, CS},
        tables::load_tss,
    },
    structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
    VirtAddr,
};

use crate::debug::log::Logger;

/// Indices des sélecteurs pour la GDT, en octets correspondant à l'append
pub const KERNEL_CODE_SEL: u16 = 1 * 8;
pub const KERNEL_DATA_SEL: u16 = 2 * 8;
pub const USER_DATA_SEL: u16 = 3 * 8 | 3;
pub const USER_CODE_SEL: u16 = 4 * 8 | 3;
// TSS xuf deux slots
pub const TSS_SEL: u16 = 5 * 8;

static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

static mut SEL_KERNEL_CODE: SegmentSelector = SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0);
static mut SEL_TSS: SegmentSelector = SegmentSelector::new(0, x86_64::PrivilegeLevel::Ring0);

/// Appelé une suele fois lors du boot, juste avant les interruptions
pub unsafe fn init() {
    Logger::log("≺GDT≻ Building table...");

    //On remplit les piles du TSS avant de créer le descripteur tss
    super::tss::init();
    Logger::log("≺GDT≻ TSS stacks initialized");

    // On construit la GDT, avec en slot 0 le null descriptor oblig. déjà présent dans ::new
    SEL_KERNEL_CODE =GDT.append(Descriptor::kernel_code_segment());
    let _sel_kdata = GDT.append(Descriptor::kernel_data_segment());
    let _sel_udata =GDT.append(Descriptor::user_data_segment());
    let _sel_ucode =GDT.append(Descriptor::user_code_segment());

    // Le desc TSS réf l'adresse du TSS statique ?
    SEL_TSS = GDT.append(Descriptor::tss_segment(&super::tss::TSS));
    Logger::log("≺GDT≻ Descriptors appended");

    // Loading gdt ; chargement du gdt
    GDT.load();
    Logger::log("≺GDT≻ lgdt done");

    // Recharger cs
    CS::set_reg(SEL_KERNEL_CODE);
    Logger::log("≺GDT≻ CS reloaded");

    // load tr
    load_tss(SEL_TSS);
    Logger::log("≺GDT≻ TSS loaded ; ltr");

    Logger::log("≺GDT≻ OK");
}