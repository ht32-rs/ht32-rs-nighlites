#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EBI_CR"]
    pub ebi_cr: EBI_CR,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - EBI_SR"]
    pub ebi_sr: EBI_SR,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - EBI_ATR"]
    pub ebi_atr: EBI_ATR,
    #[doc = "0x14 - EBI_RTR"]
    pub ebi_rtr: EBI_RTR,
    #[doc = "0x18 - EBI_WTR"]
    pub ebi_wtr: EBI_WTR,
    #[doc = "0x1c - EBI_PR"]
    pub ebi_pr: EBI_PR,
}
#[doc = "EBI_CR (rw) register accessor: an alias for `Reg<EBI_CR_SPEC>`"]
pub type EBI_CR = crate::Reg<ebi_cr::EBI_CR_SPEC>;
#[doc = "EBI_CR"]
pub mod ebi_cr;
#[doc = "EBI_SR (rw) register accessor: an alias for `Reg<EBI_SR_SPEC>`"]
pub type EBI_SR = crate::Reg<ebi_sr::EBI_SR_SPEC>;
#[doc = "EBI_SR"]
pub mod ebi_sr;
#[doc = "EBI_ATR (rw) register accessor: an alias for `Reg<EBI_ATR_SPEC>`"]
pub type EBI_ATR = crate::Reg<ebi_atr::EBI_ATR_SPEC>;
#[doc = "EBI_ATR"]
pub mod ebi_atr;
#[doc = "EBI_RTR (rw) register accessor: an alias for `Reg<EBI_RTR_SPEC>`"]
pub type EBI_RTR = crate::Reg<ebi_rtr::EBI_RTR_SPEC>;
#[doc = "EBI_RTR"]
pub mod ebi_rtr;
#[doc = "EBI_WTR (rw) register accessor: an alias for `Reg<EBI_WTR_SPEC>`"]
pub type EBI_WTR = crate::Reg<ebi_wtr::EBI_WTR_SPEC>;
#[doc = "EBI_WTR"]
pub mod ebi_wtr;
#[doc = "EBI_PR (rw) register accessor: an alias for `Reg<EBI_PR_SPEC>`"]
pub type EBI_PR = crate::Reg<ebi_pr::EBI_PR_SPEC>;
#[doc = "EBI_PR"]
pub mod ebi_pr;
