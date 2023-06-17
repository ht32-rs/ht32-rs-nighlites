#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SPICR0"]
    pub spicr0: SPICR0,
    #[doc = "0x04 - SPICR1"]
    pub spicr1: SPICR1,
    #[doc = "0x08 - SPIIER"]
    pub spiier: SPIIER,
    #[doc = "0x0c - SPICPR"]
    pub spicpr: SPICPR,
    #[doc = "0x10 - SPIDR"]
    pub spidr: SPIDR,
    #[doc = "0x14 - SPISR"]
    pub spisr: SPISR,
    #[doc = "0x18 - SPIFCR"]
    pub spifcr: SPIFCR,
    #[doc = "0x1c - SPIFSR"]
    pub spifsr: SPIFSR,
    #[doc = "0x20 - SPIFTOCR"]
    pub spiftocr: SPIFTOCR,
}
#[doc = "SPICR0 (rw) register accessor: an alias for `Reg<SPICR0_SPEC>`"]
pub type SPICR0 = crate::Reg<spicr0::SPICR0_SPEC>;
#[doc = "SPICR0"]
pub mod spicr0;
#[doc = "SPICR1 (rw) register accessor: an alias for `Reg<SPICR1_SPEC>`"]
pub type SPICR1 = crate::Reg<spicr1::SPICR1_SPEC>;
#[doc = "SPICR1"]
pub mod spicr1;
#[doc = "SPIIER (rw) register accessor: an alias for `Reg<SPIIER_SPEC>`"]
pub type SPIIER = crate::Reg<spiier::SPIIER_SPEC>;
#[doc = "SPIIER"]
pub mod spiier;
#[doc = "SPICPR (rw) register accessor: an alias for `Reg<SPICPR_SPEC>`"]
pub type SPICPR = crate::Reg<spicpr::SPICPR_SPEC>;
#[doc = "SPICPR"]
pub mod spicpr;
#[doc = "SPIDR (rw) register accessor: an alias for `Reg<SPIDR_SPEC>`"]
pub type SPIDR = crate::Reg<spidr::SPIDR_SPEC>;
#[doc = "SPIDR"]
pub mod spidr;
#[doc = "SPISR (rw) register accessor: an alias for `Reg<SPISR_SPEC>`"]
pub type SPISR = crate::Reg<spisr::SPISR_SPEC>;
#[doc = "SPISR"]
pub mod spisr;
#[doc = "SPIFCR (rw) register accessor: an alias for `Reg<SPIFCR_SPEC>`"]
pub type SPIFCR = crate::Reg<spifcr::SPIFCR_SPEC>;
#[doc = "SPIFCR"]
pub mod spifcr;
#[doc = "SPIFSR (rw) register accessor: an alias for `Reg<SPIFSR_SPEC>`"]
pub type SPIFSR = crate::Reg<spifsr::SPIFSR_SPEC>;
#[doc = "SPIFSR"]
pub mod spifsr;
#[doc = "SPIFTOCR (rw) register accessor: an alias for `Reg<SPIFTOCR_SPEC>`"]
pub type SPIFTOCR = crate::Reg<spiftocr::SPIFTOCR_SPEC>;
#[doc = "SPIFTOCR"]
pub mod spiftocr;
