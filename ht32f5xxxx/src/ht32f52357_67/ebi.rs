#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EBICR"]
    pub ebicr: EBICR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - EBISR"]
    pub ebisr: EBISR,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - EBIATR"]
    pub ebiatr: EBIATR,
    #[doc = "0x14 - EBIRTR"]
    pub ebirtr: EBIRTR,
    #[doc = "0x18 - EBIWTR"]
    pub ebiwtr: EBIWTR,
    #[doc = "0x1c - EBIPR"]
    pub ebipr: EBIPR,
}
#[doc = "EBICR (rw) register accessor: an alias for `Reg<EBICR_SPEC>`"]
pub type EBICR = crate::Reg<ebicr::EBICR_SPEC>;
#[doc = "EBICR"]
pub mod ebicr;
#[doc = "EBISR (rw) register accessor: an alias for `Reg<EBISR_SPEC>`"]
pub type EBISR = crate::Reg<ebisr::EBISR_SPEC>;
#[doc = "EBISR"]
pub mod ebisr;
#[doc = "EBIATR (rw) register accessor: an alias for `Reg<EBIATR_SPEC>`"]
pub type EBIATR = crate::Reg<ebiatr::EBIATR_SPEC>;
#[doc = "EBIATR"]
pub mod ebiatr;
#[doc = "EBIRTR (rw) register accessor: an alias for `Reg<EBIRTR_SPEC>`"]
pub type EBIRTR = crate::Reg<ebirtr::EBIRTR_SPEC>;
#[doc = "EBIRTR"]
pub mod ebirtr;
#[doc = "EBIWTR (rw) register accessor: an alias for `Reg<EBIWTR_SPEC>`"]
pub type EBIWTR = crate::Reg<ebiwtr::EBIWTR_SPEC>;
#[doc = "EBIWTR"]
pub mod ebiwtr;
#[doc = "EBIPR (rw) register accessor: an alias for `Reg<EBIPR_SPEC>`"]
pub type EBIPR = crate::Reg<ebipr::EBIPR_SPEC>;
#[doc = "EBIPR"]
pub mod ebipr;
