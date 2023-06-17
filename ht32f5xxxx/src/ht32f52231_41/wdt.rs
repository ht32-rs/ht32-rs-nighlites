#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDTCR"]
    pub wdtcr: WDTCR,
    #[doc = "0x04 - WDTMR0"]
    pub wdtmr0: WDTMR0,
    #[doc = "0x08 - WDTMR1"]
    pub wdtmr1: WDTMR1,
    #[doc = "0x0c - WDTSR"]
    pub wdtsr: WDTSR,
    #[doc = "0x10 - WDTPR"]
    pub wdtpr: WDTPR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - WDTCSR"]
    pub wdtcsr: WDTCSR,
}
#[doc = "WDTCR (rw) register accessor: an alias for `Reg<WDTCR_SPEC>`"]
pub type WDTCR = crate::Reg<wdtcr::WDTCR_SPEC>;
#[doc = "WDTCR"]
pub mod wdtcr;
#[doc = "WDTMR0 (rw) register accessor: an alias for `Reg<WDTMR0_SPEC>`"]
pub type WDTMR0 = crate::Reg<wdtmr0::WDTMR0_SPEC>;
#[doc = "WDTMR0"]
pub mod wdtmr0;
#[doc = "WDTMR1 (rw) register accessor: an alias for `Reg<WDTMR1_SPEC>`"]
pub type WDTMR1 = crate::Reg<wdtmr1::WDTMR1_SPEC>;
#[doc = "WDTMR1"]
pub mod wdtmr1;
#[doc = "WDTSR (rw) register accessor: an alias for `Reg<WDTSR_SPEC>`"]
pub type WDTSR = crate::Reg<wdtsr::WDTSR_SPEC>;
#[doc = "WDTSR"]
pub mod wdtsr;
#[doc = "WDTPR (rw) register accessor: an alias for `Reg<WDTPR_SPEC>`"]
pub type WDTPR = crate::Reg<wdtpr::WDTPR_SPEC>;
#[doc = "WDTPR"]
pub mod wdtpr;
#[doc = "WDTCSR (rw) register accessor: an alias for `Reg<WDTCSR_SPEC>`"]
pub type WDTCSR = crate::Reg<wdtcsr::WDTCSR_SPEC>;
#[doc = "WDTCSR"]
pub mod wdtcsr;
