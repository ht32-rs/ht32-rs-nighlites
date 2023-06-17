#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BFTMCR"]
    pub bftmcr: BFTMCR,
    #[doc = "0x04 - BFTMSR"]
    pub bftmsr: BFTMSR,
    #[doc = "0x08 - BFTMCNTR"]
    pub bftmcntr: BFTMCNTR,
    #[doc = "0x0c - BFTMCMPR"]
    pub bftmcmpr: BFTMCMPR,
}
#[doc = "BFTMCR (rw) register accessor: an alias for `Reg<BFTMCR_SPEC>`"]
pub type BFTMCR = crate::Reg<bftmcr::BFTMCR_SPEC>;
#[doc = "BFTMCR"]
pub mod bftmcr;
#[doc = "BFTMSR (rw) register accessor: an alias for `Reg<BFTMSR_SPEC>`"]
pub type BFTMSR = crate::Reg<bftmsr::BFTMSR_SPEC>;
#[doc = "BFTMSR"]
pub mod bftmsr;
#[doc = "BFTMCNTR (rw) register accessor: an alias for `Reg<BFTMCNTR_SPEC>`"]
pub type BFTMCNTR = crate::Reg<bftmcntr::BFTMCNTR_SPEC>;
#[doc = "BFTMCNTR"]
pub mod bftmcntr;
#[doc = "BFTMCMPR (rw) register accessor: an alias for `Reg<BFTMCMPR_SPEC>`"]
pub type BFTMCMPR = crate::Reg<bftmcmpr::BFTMCMPR_SPEC>;
#[doc = "BFTMCMPR"]
pub mod bftmcmpr;
