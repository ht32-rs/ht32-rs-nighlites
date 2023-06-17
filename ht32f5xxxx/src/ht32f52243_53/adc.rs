#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADCCR"]
    pub adccr: ADCCR,
    #[doc = "0x04 - ADCLST0"]
    pub adclst0: ADCLST0,
    #[doc = "0x08 - ADCLST1"]
    pub adclst1: ADCLST1,
    _reserved3: [u8; 0x14],
    #[doc = "0x20 - ADCSTR"]
    pub adcstr: ADCSTR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x30 - ADCDR0"]
    pub adcdr0: ADCDR0,
    #[doc = "0x34 - ADCDR1"]
    pub adcdr1: ADCDR1,
    #[doc = "0x38 - ADCDR2"]
    pub adcdr2: ADCDR2,
    #[doc = "0x3c - ADCDR3"]
    pub adcdr3: ADCDR3,
    #[doc = "0x40 - ADCDR4"]
    pub adcdr4: ADCDR4,
    #[doc = "0x44 - ADCDR5"]
    pub adcdr5: ADCDR5,
    #[doc = "0x48 - ADCDR6"]
    pub adcdr6: ADCDR6,
    #[doc = "0x4c - ADCDR7"]
    pub adcdr7: ADCDR7,
    _reserved12: [u8; 0x20],
    #[doc = "0x70 - ADCTCR"]
    pub adctcr: ADCTCR,
    #[doc = "0x74 - ADCTSR"]
    pub adctsr: ADCTSR,
    #[doc = "0x78 - ADCWCR"]
    pub adcwcr: ADCWCR,
    #[doc = "0x7c - ADCTR"]
    pub adctr: ADCTR,
    #[doc = "0x80 - ADCIER"]
    pub adcier: ADCIER,
    #[doc = "0x84 - ADCIRAW"]
    pub adciraw: ADCIRAW,
    #[doc = "0x88 - ADCISR"]
    pub adcisr: ADCISR,
    #[doc = "0x8c - ADCICLR"]
    pub adciclr: ADCICLR,
}
#[doc = "ADCCR (rw) register accessor: an alias for `Reg<ADCCR_SPEC>`"]
pub type ADCCR = crate::Reg<adccr::ADCCR_SPEC>;
#[doc = "ADCCR"]
pub mod adccr;
#[doc = "ADCLST0 (rw) register accessor: an alias for `Reg<ADCLST0_SPEC>`"]
pub type ADCLST0 = crate::Reg<adclst0::ADCLST0_SPEC>;
#[doc = "ADCLST0"]
pub mod adclst0;
#[doc = "ADCLST1 (rw) register accessor: an alias for `Reg<ADCLST1_SPEC>`"]
pub type ADCLST1 = crate::Reg<adclst1::ADCLST1_SPEC>;
#[doc = "ADCLST1"]
pub mod adclst1;
#[doc = "ADCSTR (rw) register accessor: an alias for `Reg<ADCSTR_SPEC>`"]
pub type ADCSTR = crate::Reg<adcstr::ADCSTR_SPEC>;
#[doc = "ADCSTR"]
pub mod adcstr;
#[doc = "ADCDR0 (rw) register accessor: an alias for `Reg<ADCDR0_SPEC>`"]
pub type ADCDR0 = crate::Reg<adcdr0::ADCDR0_SPEC>;
#[doc = "ADCDR0"]
pub mod adcdr0;
#[doc = "ADCDR1 (rw) register accessor: an alias for `Reg<ADCDR1_SPEC>`"]
pub type ADCDR1 = crate::Reg<adcdr1::ADCDR1_SPEC>;
#[doc = "ADCDR1"]
pub mod adcdr1;
#[doc = "ADCDR2 (rw) register accessor: an alias for `Reg<ADCDR2_SPEC>`"]
pub type ADCDR2 = crate::Reg<adcdr2::ADCDR2_SPEC>;
#[doc = "ADCDR2"]
pub mod adcdr2;
#[doc = "ADCDR3 (rw) register accessor: an alias for `Reg<ADCDR3_SPEC>`"]
pub type ADCDR3 = crate::Reg<adcdr3::ADCDR3_SPEC>;
#[doc = "ADCDR3"]
pub mod adcdr3;
#[doc = "ADCDR4 (rw) register accessor: an alias for `Reg<ADCDR4_SPEC>`"]
pub type ADCDR4 = crate::Reg<adcdr4::ADCDR4_SPEC>;
#[doc = "ADCDR4"]
pub mod adcdr4;
#[doc = "ADCDR5 (rw) register accessor: an alias for `Reg<ADCDR5_SPEC>`"]
pub type ADCDR5 = crate::Reg<adcdr5::ADCDR5_SPEC>;
#[doc = "ADCDR5"]
pub mod adcdr5;
#[doc = "ADCDR6 (rw) register accessor: an alias for `Reg<ADCDR6_SPEC>`"]
pub type ADCDR6 = crate::Reg<adcdr6::ADCDR6_SPEC>;
#[doc = "ADCDR6"]
pub mod adcdr6;
#[doc = "ADCDR7 (rw) register accessor: an alias for `Reg<ADCDR7_SPEC>`"]
pub type ADCDR7 = crate::Reg<adcdr7::ADCDR7_SPEC>;
#[doc = "ADCDR7"]
pub mod adcdr7;
#[doc = "ADCTCR (rw) register accessor: an alias for `Reg<ADCTCR_SPEC>`"]
pub type ADCTCR = crate::Reg<adctcr::ADCTCR_SPEC>;
#[doc = "ADCTCR"]
pub mod adctcr;
#[doc = "ADCTSR (rw) register accessor: an alias for `Reg<ADCTSR_SPEC>`"]
pub type ADCTSR = crate::Reg<adctsr::ADCTSR_SPEC>;
#[doc = "ADCTSR"]
pub mod adctsr;
#[doc = "ADCWCR (rw) register accessor: an alias for `Reg<ADCWCR_SPEC>`"]
pub type ADCWCR = crate::Reg<adcwcr::ADCWCR_SPEC>;
#[doc = "ADCWCR"]
pub mod adcwcr;
#[doc = "ADCTR (rw) register accessor: an alias for `Reg<ADCTR_SPEC>`"]
pub type ADCTR = crate::Reg<adctr::ADCTR_SPEC>;
#[doc = "ADCTR"]
pub mod adctr;
#[doc = "ADCIER (rw) register accessor: an alias for `Reg<ADCIER_SPEC>`"]
pub type ADCIER = crate::Reg<adcier::ADCIER_SPEC>;
#[doc = "ADCIER"]
pub mod adcier;
#[doc = "ADCIRAW (rw) register accessor: an alias for `Reg<ADCIRAW_SPEC>`"]
pub type ADCIRAW = crate::Reg<adciraw::ADCIRAW_SPEC>;
#[doc = "ADCIRAW"]
pub mod adciraw;
#[doc = "ADCISR (rw) register accessor: an alias for `Reg<ADCISR_SPEC>`"]
pub type ADCISR = crate::Reg<adcisr::ADCISR_SPEC>;
#[doc = "ADCISR"]
pub mod adcisr;
#[doc = "ADCICLR (rw) register accessor: an alias for `Reg<ADCICLR_SPEC>`"]
pub type ADCICLR = crate::Reg<adciclr::ADCICLR_SPEC>;
#[doc = "ADCICLR"]
pub mod adciclr;
