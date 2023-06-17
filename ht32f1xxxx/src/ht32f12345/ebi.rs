#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EBI_CR"]
    pub ebi_cr: EBI_CR,
    #[doc = "0x04 - EBI_PCR"]
    pub ebi_pcr: EBI_PCR,
    #[doc = "0x08 - EBI_SR"]
    pub ebi_sr: EBI_SR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - EBI_ATR0"]
    pub ebi_atr0: EBI_ATR0,
    #[doc = "0x14 - EBI_RTR0"]
    pub ebi_rtr0: EBI_RTR0,
    #[doc = "0x18 - EBI_WTR0"]
    pub ebi_wtr0: EBI_WTR0,
    #[doc = "0x1c - EBI_PR0"]
    pub ebi_pr0: EBI_PR0,
    #[doc = "0x20 - EBI_ATR1"]
    pub ebi_atr1: EBI_ATR1,
    #[doc = "0x24 - EBI_RTR1"]
    pub ebi_rtr1: EBI_RTR1,
    #[doc = "0x28 - EBI_WTR1"]
    pub ebi_wtr1: EBI_WTR1,
    #[doc = "0x2c - EBI_PR1"]
    pub ebi_pr1: EBI_PR1,
    #[doc = "0x30 - EBI_ATR2"]
    pub ebi_atr2: EBI_ATR2,
    #[doc = "0x34 - EBI_RTR2"]
    pub ebi_rtr2: EBI_RTR2,
    #[doc = "0x38 - EBI_WTR2"]
    pub ebi_wtr2: EBI_WTR2,
    #[doc = "0x3c - EBI_PR2"]
    pub ebi_pr2: EBI_PR2,
    #[doc = "0x40 - EBI_ATR3"]
    pub ebi_atr3: EBI_ATR3,
    #[doc = "0x44 - EBI_RTR3"]
    pub ebi_rtr3: EBI_RTR3,
    #[doc = "0x48 - EBI_WTR3"]
    pub ebi_wtr3: EBI_WTR3,
    #[doc = "0x4c - EBI_PR3"]
    pub ebi_pr3: EBI_PR3,
    #[doc = "0x50 - EBI_IENR"]
    pub ebi_ienr: EBI_IENR,
    #[doc = "0x54 - EBI_IFR"]
    pub ebi_ifr: EBI_IFR,
    #[doc = "0x58 - EBI_IFCR"]
    pub ebi_ifcr: EBI_IFCR,
}
#[doc = "EBI_CR (rw) register accessor: an alias for `Reg<EBI_CR_SPEC>`"]
pub type EBI_CR = crate::Reg<ebi_cr::EBI_CR_SPEC>;
#[doc = "EBI_CR"]
pub mod ebi_cr;
#[doc = "EBI_PCR (rw) register accessor: an alias for `Reg<EBI_PCR_SPEC>`"]
pub type EBI_PCR = crate::Reg<ebi_pcr::EBI_PCR_SPEC>;
#[doc = "EBI_PCR"]
pub mod ebi_pcr;
#[doc = "EBI_SR (rw) register accessor: an alias for `Reg<EBI_SR_SPEC>`"]
pub type EBI_SR = crate::Reg<ebi_sr::EBI_SR_SPEC>;
#[doc = "EBI_SR"]
pub mod ebi_sr;
#[doc = "EBI_ATR0 (rw) register accessor: an alias for `Reg<EBI_ATR0_SPEC>`"]
pub type EBI_ATR0 = crate::Reg<ebi_atr0::EBI_ATR0_SPEC>;
#[doc = "EBI_ATR0"]
pub mod ebi_atr0;
#[doc = "EBI_ATR1 (rw) register accessor: an alias for `Reg<EBI_ATR1_SPEC>`"]
pub type EBI_ATR1 = crate::Reg<ebi_atr1::EBI_ATR1_SPEC>;
#[doc = "EBI_ATR1"]
pub mod ebi_atr1;
#[doc = "EBI_ATR2 (rw) register accessor: an alias for `Reg<EBI_ATR2_SPEC>`"]
pub type EBI_ATR2 = crate::Reg<ebi_atr2::EBI_ATR2_SPEC>;
#[doc = "EBI_ATR2"]
pub mod ebi_atr2;
#[doc = "EBI_ATR3 (rw) register accessor: an alias for `Reg<EBI_ATR3_SPEC>`"]
pub type EBI_ATR3 = crate::Reg<ebi_atr3::EBI_ATR3_SPEC>;
#[doc = "EBI_ATR3"]
pub mod ebi_atr3;
#[doc = "EBI_RTR0 (rw) register accessor: an alias for `Reg<EBI_RTR0_SPEC>`"]
pub type EBI_RTR0 = crate::Reg<ebi_rtr0::EBI_RTR0_SPEC>;
#[doc = "EBI_RTR0"]
pub mod ebi_rtr0;
#[doc = "EBI_RTR1 (rw) register accessor: an alias for `Reg<EBI_RTR1_SPEC>`"]
pub type EBI_RTR1 = crate::Reg<ebi_rtr1::EBI_RTR1_SPEC>;
#[doc = "EBI_RTR1"]
pub mod ebi_rtr1;
#[doc = "EBI_RTR2 (rw) register accessor: an alias for `Reg<EBI_RTR2_SPEC>`"]
pub type EBI_RTR2 = crate::Reg<ebi_rtr2::EBI_RTR2_SPEC>;
#[doc = "EBI_RTR2"]
pub mod ebi_rtr2;
#[doc = "EBI_RTR3 (rw) register accessor: an alias for `Reg<EBI_RTR3_SPEC>`"]
pub type EBI_RTR3 = crate::Reg<ebi_rtr3::EBI_RTR3_SPEC>;
#[doc = "EBI_RTR3"]
pub mod ebi_rtr3;
#[doc = "EBI_WTR0 (rw) register accessor: an alias for `Reg<EBI_WTR0_SPEC>`"]
pub type EBI_WTR0 = crate::Reg<ebi_wtr0::EBI_WTR0_SPEC>;
#[doc = "EBI_WTR0"]
pub mod ebi_wtr0;
#[doc = "EBI_WTR1 (rw) register accessor: an alias for `Reg<EBI_WTR1_SPEC>`"]
pub type EBI_WTR1 = crate::Reg<ebi_wtr1::EBI_WTR1_SPEC>;
#[doc = "EBI_WTR1"]
pub mod ebi_wtr1;
#[doc = "EBI_WTR2 (rw) register accessor: an alias for `Reg<EBI_WTR2_SPEC>`"]
pub type EBI_WTR2 = crate::Reg<ebi_wtr2::EBI_WTR2_SPEC>;
#[doc = "EBI_WTR2"]
pub mod ebi_wtr2;
#[doc = "EBI_WTR3 (rw) register accessor: an alias for `Reg<EBI_WTR3_SPEC>`"]
pub type EBI_WTR3 = crate::Reg<ebi_wtr3::EBI_WTR3_SPEC>;
#[doc = "EBI_WTR3"]
pub mod ebi_wtr3;
#[doc = "EBI_PR0 (rw) register accessor: an alias for `Reg<EBI_PR0_SPEC>`"]
pub type EBI_PR0 = crate::Reg<ebi_pr0::EBI_PR0_SPEC>;
#[doc = "EBI_PR0"]
pub mod ebi_pr0;
#[doc = "EBI_PR1 (rw) register accessor: an alias for `Reg<EBI_PR1_SPEC>`"]
pub type EBI_PR1 = crate::Reg<ebi_pr1::EBI_PR1_SPEC>;
#[doc = "EBI_PR1"]
pub mod ebi_pr1;
#[doc = "EBI_PR2 (rw) register accessor: an alias for `Reg<EBI_PR2_SPEC>`"]
pub type EBI_PR2 = crate::Reg<ebi_pr2::EBI_PR2_SPEC>;
#[doc = "EBI_PR2"]
pub mod ebi_pr2;
#[doc = "EBI_PR3 (rw) register accessor: an alias for `Reg<EBI_PR3_SPEC>`"]
pub type EBI_PR3 = crate::Reg<ebi_pr3::EBI_PR3_SPEC>;
#[doc = "EBI_PR3"]
pub mod ebi_pr3;
#[doc = "EBI_IENR (rw) register accessor: an alias for `Reg<EBI_IENR_SPEC>`"]
pub type EBI_IENR = crate::Reg<ebi_ienr::EBI_IENR_SPEC>;
#[doc = "EBI_IENR"]
pub mod ebi_ienr;
#[doc = "EBI_IFR (rw) register accessor: an alias for `Reg<EBI_IFR_SPEC>`"]
pub type EBI_IFR = crate::Reg<ebi_ifr::EBI_IFR_SPEC>;
#[doc = "EBI_IFR"]
pub mod ebi_ifr;
#[doc = "EBI_IFCR (rw) register accessor: an alias for `Reg<EBI_IFCR_SPEC>`"]
pub type EBI_IFCR = crate::Reg<ebi_ifcr::EBI_IFCR_SPEC>;
#[doc = "EBI_IFCR"]
pub mod ebi_ifcr;
