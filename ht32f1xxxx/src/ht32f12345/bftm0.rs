#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BFTM_CR"]
    pub bftm_cr: BFTM_CR,
    #[doc = "0x04 - BFTM_SR"]
    pub bftm_sr: BFTM_SR,
    #[doc = "0x08 - BFTM_CNTR"]
    pub bftm_cntr: BFTM_CNTR,
    #[doc = "0x0c - BFTM_CMPR"]
    pub bftm_cmpr: BFTM_CMPR,
}
#[doc = "BFTM_CR (rw) register accessor: an alias for `Reg<BFTM_CR_SPEC>`"]
pub type BFTM_CR = crate::Reg<bftm_cr::BFTM_CR_SPEC>;
#[doc = "BFTM_CR"]
pub mod bftm_cr;
#[doc = "BFTM_SR (rw) register accessor: an alias for `Reg<BFTM_SR_SPEC>`"]
pub type BFTM_SR = crate::Reg<bftm_sr::BFTM_SR_SPEC>;
#[doc = "BFTM_SR"]
pub mod bftm_sr;
#[doc = "BFTM_CNTR (rw) register accessor: an alias for `Reg<BFTM_CNTR_SPEC>`"]
pub type BFTM_CNTR = crate::Reg<bftm_cntr::BFTM_CNTR_SPEC>;
#[doc = "BFTM_CNTR"]
pub mod bftm_cntr;
#[doc = "BFTM_CMPR (rw) register accessor: an alias for `Reg<BFTM_CMPR_SPEC>`"]
pub type BFTM_CMPR = crate::Reg<bftm_cmpr::BFTM_CMPR_SPEC>;
#[doc = "BFTM_CMPR"]
pub mod bftm_cmpr;
