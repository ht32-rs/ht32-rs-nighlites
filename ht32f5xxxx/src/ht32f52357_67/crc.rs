#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRCCR"]
    pub crccr: CRCCR,
    #[doc = "0x04 - CRCSDR"]
    pub crcsdr: CRCSDR,
    #[doc = "0x08 - CRCCSR"]
    pub crccsr: CRCCSR,
    #[doc = "0x0c - CRCDR"]
    pub crcdr: CRCDR,
}
#[doc = "CRCCR (rw) register accessor: an alias for `Reg<CRCCR_SPEC>`"]
pub type CRCCR = crate::Reg<crccr::CRCCR_SPEC>;
#[doc = "CRCCR"]
pub mod crccr;
#[doc = "CRCSDR (rw) register accessor: an alias for `Reg<CRCSDR_SPEC>`"]
pub type CRCSDR = crate::Reg<crcsdr::CRCSDR_SPEC>;
#[doc = "CRCSDR"]
pub mod crcsdr;
#[doc = "CRCCSR (rw) register accessor: an alias for `Reg<CRCCSR_SPEC>`"]
pub type CRCCSR = crate::Reg<crccsr::CRCCSR_SPEC>;
#[doc = "CRCCSR"]
pub mod crccsr;
#[doc = "CRCDR (rw) register accessor: an alias for `Reg<CRCDR_SPEC>`"]
pub type CRCDR = crate::Reg<crcdr::CRCDR_SPEC>;
#[doc = "CRCDR"]
pub mod crcdr;
