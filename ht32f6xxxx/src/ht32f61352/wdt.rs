#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT_CR"]
    pub wdt_cr: WDT_CR,
    #[doc = "0x04 - WDT_MR0"]
    pub wdt_mr0: WDT_MR0,
    #[doc = "0x08 - WDT_MR1"]
    pub wdt_mr1: WDT_MR1,
    #[doc = "0x0c - WDT_SR"]
    pub wdt_sr: WDT_SR,
    #[doc = "0x10 - WDT_PR"]
    pub wdt_pr: WDT_PR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - WDT_CSR"]
    pub wdt_csr: WDT_CSR,
}
#[doc = "WDT_CR (rw) register accessor: an alias for `Reg<WDT_CR_SPEC>`"]
pub type WDT_CR = crate::Reg<wdt_cr::WDT_CR_SPEC>;
#[doc = "WDT_CR"]
pub mod wdt_cr;
#[doc = "WDT_MR0 (rw) register accessor: an alias for `Reg<WDT_MR0_SPEC>`"]
pub type WDT_MR0 = crate::Reg<wdt_mr0::WDT_MR0_SPEC>;
#[doc = "WDT_MR0"]
pub mod wdt_mr0;
#[doc = "WDT_MR1 (rw) register accessor: an alias for `Reg<WDT_MR1_SPEC>`"]
pub type WDT_MR1 = crate::Reg<wdt_mr1::WDT_MR1_SPEC>;
#[doc = "WDT_MR1"]
pub mod wdt_mr1;
#[doc = "WDT_SR (rw) register accessor: an alias for `Reg<WDT_SR_SPEC>`"]
pub type WDT_SR = crate::Reg<wdt_sr::WDT_SR_SPEC>;
#[doc = "WDT_SR"]
pub mod wdt_sr;
#[doc = "WDT_PR (rw) register accessor: an alias for `Reg<WDT_PR_SPEC>`"]
pub type WDT_PR = crate::Reg<wdt_pr::WDT_PR_SPEC>;
#[doc = "WDT_PR"]
pub mod wdt_pr;
#[doc = "WDT_CSR (rw) register accessor: an alias for `Reg<WDT_CSR_SPEC>`"]
pub type WDT_CSR = crate::Reg<wdt_csr::WDT_CSR_SPEC>;
#[doc = "WDT_CSR"]
pub mod wdt_csr;
