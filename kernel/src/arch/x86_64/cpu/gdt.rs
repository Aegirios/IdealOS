use lazy_static::lazy_static;

use x86_64::{
    instructions::{
        segmentation::{Segment, CS},
        tables::load_tss,
    },
    structures::gdt::{
        Descriptor,
        GlobalDescriptorTable,
        SegmentSelector,
    },
};
use crate::debug::log::Logger;
use super::tss::TSS;

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, SegmentSelector) = {
        let mut gdt = GlobalDescriptorTable::new();

        let code = gdt.append(Descriptor::kernel_code_segment());

        (gdt, code)
    };
}

pub fn init() {
    Logger::log("GDT start");

    GDT.0.load();

    Logger::log("GDT loaded");

    unsafe {
        CS::set_reg(GDT.1);
    }

    Logger::log("CS set OK");
}