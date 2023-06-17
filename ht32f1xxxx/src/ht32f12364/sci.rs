#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCI_CR"]
    pub sci_cr: SCI_CR,
    #[doc = "0x04 - SCI_SR"]
    pub sci_sr: SCI_SR,
    #[doc = "0x08 - SCI_CCR"]
    pub sci_ccr: SCI_CCR,
    #[doc = "0x0c - SCI_ETU"]
    pub sci_etu: SCI_ETU,
    #[doc = "0x10 - SCI_GT"]
    pub sci_gt: SCI_GT,
    #[doc = "0x14 - SCI_WT"]
    pub sci_wt: SCI_WT,
    #[doc = "0x18 - SCI_IER"]
    pub sci_ier: SCI_IER,
    #[doc = "0x1c - SCI_IPR"]
    pub sci_ipr: SCI_IPR,
    #[doc = "0x20 - SCI_TXB"]
    pub sci_txb: SCI_TXB,
    #[doc = "0x24 - SCI_RXB"]
    pub sci_rxb: SCI_RXB,
    #[doc = "0x28 - SCI_PSC"]
    pub sci_psc: SCI_PSC,
}
#[doc = "SCI_CR (rw) register accessor: an alias for `Reg<SCI_CR_SPEC>`"]
pub type SCI_CR = crate::Reg<sci_cr::SCI_CR_SPEC>;
#[doc = "SCI_CR"]
pub mod sci_cr;
#[doc = "SCI_SR (rw) register accessor: an alias for `Reg<SCI_SR_SPEC>`"]
pub type SCI_SR = crate::Reg<sci_sr::SCI_SR_SPEC>;
#[doc = "SCI_SR"]
pub mod sci_sr;
#[doc = "SCI_CCR (rw) register accessor: an alias for `Reg<SCI_CCR_SPEC>`"]
pub type SCI_CCR = crate::Reg<sci_ccr::SCI_CCR_SPEC>;
#[doc = "SCI_CCR"]
pub mod sci_ccr;
#[doc = "SCI_ETU (rw) register accessor: an alias for `Reg<SCI_ETU_SPEC>`"]
pub type SCI_ETU = crate::Reg<sci_etu::SCI_ETU_SPEC>;
#[doc = "SCI_ETU"]
pub mod sci_etu;
#[doc = "SCI_GT (rw) register accessor: an alias for `Reg<SCI_GT_SPEC>`"]
pub type SCI_GT = crate::Reg<sci_gt::SCI_GT_SPEC>;
#[doc = "SCI_GT"]
pub mod sci_gt;
#[doc = "SCI_WT (rw) register accessor: an alias for `Reg<SCI_WT_SPEC>`"]
pub type SCI_WT = crate::Reg<sci_wt::SCI_WT_SPEC>;
#[doc = "SCI_WT"]
pub mod sci_wt;
#[doc = "SCI_IER (rw) register accessor: an alias for `Reg<SCI_IER_SPEC>`"]
pub type SCI_IER = crate::Reg<sci_ier::SCI_IER_SPEC>;
#[doc = "SCI_IER"]
pub mod sci_ier;
#[doc = "SCI_IPR (rw) register accessor: an alias for `Reg<SCI_IPR_SPEC>`"]
pub type SCI_IPR = crate::Reg<sci_ipr::SCI_IPR_SPEC>;
#[doc = "SCI_IPR"]
pub mod sci_ipr;
#[doc = "SCI_TXB (rw) register accessor: an alias for `Reg<SCI_TXB_SPEC>`"]
pub type SCI_TXB = crate::Reg<sci_txb::SCI_TXB_SPEC>;
#[doc = "SCI_TXB"]
pub mod sci_txb;
#[doc = "SCI_RXB (rw) register accessor: an alias for `Reg<SCI_RXB_SPEC>`"]
pub type SCI_RXB = crate::Reg<sci_rxb::SCI_RXB_SPEC>;
#[doc = "SCI_RXB"]
pub mod sci_rxb;
#[doc = "SCI_PSC (rw) register accessor: an alias for `Reg<SCI_PSC_SPEC>`"]
pub type SCI_PSC = crate::Reg<sci_psc::SCI_PSC_SPEC>;
#[doc = "SCI_PSC"]
pub mod sci_psc;
