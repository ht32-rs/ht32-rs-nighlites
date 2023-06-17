#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SCTM_CNTCFR"]
    pub sctm_cntcfr: SCTM_CNTCFR,
    #[doc = "0x04 - SCTM_MDCFR"]
    pub sctm_mdcfr: SCTM_MDCFR,
    #[doc = "0x08 - SCTM_TRCFR"]
    pub sctm_trcfr: SCTM_TRCFR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - SCTM_CTR"]
    pub sctm_ctr: SCTM_CTR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - SCTM_CH0ICFR"]
    pub sctm_ch0icfr: SCTM_CH0ICFR,
    _reserved5: [u8; 0x1c],
    #[doc = "0x40 - SCTM_CH0OCFR"]
    pub sctm_ch0ocfr: SCTM_CH0OCFR,
    _reserved6: [u8; 0x0c],
    #[doc = "0x50 - GPTM_CHCTR"]
    pub gptm_chctr: GPTM_CHCTR,
    #[doc = "0x54 - GPTM_CHPOLR"]
    pub gptm_chpolr: GPTM_CHPOLR,
    _reserved8: [u8; 0x1c],
    #[doc = "0x74 - GPTM_DICTR"]
    pub gptm_dictr: GPTM_DICTR,
    #[doc = "0x78 - GPTM_EVGR"]
    pub gptm_evgr: GPTM_EVGR,
    #[doc = "0x7c - GPTM_INTSR"]
    pub gptm_intsr: GPTM_INTSR,
    #[doc = "0x80 - GPTM_CNTR"]
    pub gptm_cntr: GPTM_CNTR,
    #[doc = "0x84 - GPTM_PSCR"]
    pub gptm_pscr: GPTM_PSCR,
    #[doc = "0x88 - GPTM_CRR"]
    pub gptm_crr: GPTM_CRR,
    _reserved14: [u8; 0x04],
    #[doc = "0x90 - GPTM_CH0CCR"]
    pub gptm_ch0ccr: GPTM_CH0CCR,
}
#[doc = "SCTM_CNTCFR (rw) register accessor: an alias for `Reg<SCTM_CNTCFR_SPEC>`"]
pub type SCTM_CNTCFR = crate::Reg<sctm_cntcfr::SCTM_CNTCFR_SPEC>;
#[doc = "SCTM_CNTCFR"]
pub mod sctm_cntcfr;
#[doc = "SCTM_MDCFR (rw) register accessor: an alias for `Reg<SCTM_MDCFR_SPEC>`"]
pub type SCTM_MDCFR = crate::Reg<sctm_mdcfr::SCTM_MDCFR_SPEC>;
#[doc = "SCTM_MDCFR"]
pub mod sctm_mdcfr;
#[doc = "SCTM_TRCFR (rw) register accessor: an alias for `Reg<SCTM_TRCFR_SPEC>`"]
pub type SCTM_TRCFR = crate::Reg<sctm_trcfr::SCTM_TRCFR_SPEC>;
#[doc = "SCTM_TRCFR"]
pub mod sctm_trcfr;
#[doc = "SCTM_CTR (rw) register accessor: an alias for `Reg<SCTM_CTR_SPEC>`"]
pub type SCTM_CTR = crate::Reg<sctm_ctr::SCTM_CTR_SPEC>;
#[doc = "SCTM_CTR"]
pub mod sctm_ctr;
#[doc = "SCTM_CH0ICFR (rw) register accessor: an alias for `Reg<SCTM_CH0ICFR_SPEC>`"]
pub type SCTM_CH0ICFR = crate::Reg<sctm_ch0icfr::SCTM_CH0ICFR_SPEC>;
#[doc = "SCTM_CH0ICFR"]
pub mod sctm_ch0icfr;
#[doc = "SCTM_CH0OCFR (rw) register accessor: an alias for `Reg<SCTM_CH0OCFR_SPEC>`"]
pub type SCTM_CH0OCFR = crate::Reg<sctm_ch0ocfr::SCTM_CH0OCFR_SPEC>;
#[doc = "SCTM_CH0OCFR"]
pub mod sctm_ch0ocfr;
#[doc = "GPTM_CHCTR (rw) register accessor: an alias for `Reg<GPTM_CHCTR_SPEC>`"]
pub type GPTM_CHCTR = crate::Reg<gptm_chctr::GPTM_CHCTR_SPEC>;
#[doc = "GPTM_CHCTR"]
pub mod gptm_chctr;
#[doc = "GPTM_CHPOLR (rw) register accessor: an alias for `Reg<GPTM_CHPOLR_SPEC>`"]
pub type GPTM_CHPOLR = crate::Reg<gptm_chpolr::GPTM_CHPOLR_SPEC>;
#[doc = "GPTM_CHPOLR"]
pub mod gptm_chpolr;
#[doc = "GPTM_DICTR (rw) register accessor: an alias for `Reg<GPTM_DICTR_SPEC>`"]
pub type GPTM_DICTR = crate::Reg<gptm_dictr::GPTM_DICTR_SPEC>;
#[doc = "GPTM_DICTR"]
pub mod gptm_dictr;
#[doc = "GPTM_EVGR (rw) register accessor: an alias for `Reg<GPTM_EVGR_SPEC>`"]
pub type GPTM_EVGR = crate::Reg<gptm_evgr::GPTM_EVGR_SPEC>;
#[doc = "GPTM_EVGR"]
pub mod gptm_evgr;
#[doc = "GPTM_INTSR (rw) register accessor: an alias for `Reg<GPTM_INTSR_SPEC>`"]
pub type GPTM_INTSR = crate::Reg<gptm_intsr::GPTM_INTSR_SPEC>;
#[doc = "GPTM_INTSR"]
pub mod gptm_intsr;
#[doc = "GPTM_CNTR (rw) register accessor: an alias for `Reg<GPTM_CNTR_SPEC>`"]
pub type GPTM_CNTR = crate::Reg<gptm_cntr::GPTM_CNTR_SPEC>;
#[doc = "GPTM_CNTR"]
pub mod gptm_cntr;
#[doc = "GPTM_PSCR (rw) register accessor: an alias for `Reg<GPTM_PSCR_SPEC>`"]
pub type GPTM_PSCR = crate::Reg<gptm_pscr::GPTM_PSCR_SPEC>;
#[doc = "GPTM_PSCR"]
pub mod gptm_pscr;
#[doc = "GPTM_CRR (rw) register accessor: an alias for `Reg<GPTM_CRR_SPEC>`"]
pub type GPTM_CRR = crate::Reg<gptm_crr::GPTM_CRR_SPEC>;
#[doc = "GPTM_CRR"]
pub mod gptm_crr;
#[doc = "GPTM_CH0CCR (rw) register accessor: an alias for `Reg<GPTM_CH0CCR_SPEC>`"]
pub type GPTM_CH0CCR = crate::Reg<gptm_ch0ccr::GPTM_CH0CCR_SPEC>;
#[doc = "GPTM_CH0CCR"]
pub mod gptm_ch0ccr;
