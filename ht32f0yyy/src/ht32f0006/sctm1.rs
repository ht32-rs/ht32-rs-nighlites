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
    #[doc = "0x20 - SCTM_CHICFR"]
    pub sctm_chicfr: SCTM_CHICFR,
    _reserved5: [u8; 0x1c],
    #[doc = "0x40 - SCTM_CHOCFR"]
    pub sctm_chocfr: SCTM_CHOCFR,
    _reserved6: [u8; 0x0c],
    #[doc = "0x50 - SCTM_CHCTR"]
    pub sctm_chctr: SCTM_CHCTR,
    #[doc = "0x54 - SCTM_CHPOLR"]
    pub sctm_chpolr: SCTM_CHPOLR,
    _reserved8: [u8; 0x1c],
    #[doc = "0x74 - SCTM_DICTR"]
    pub sctm_dictr: SCTM_DICTR,
    #[doc = "0x78 - SCTM_EVGR"]
    pub sctm_evgr: SCTM_EVGR,
    #[doc = "0x7c - SCTM_INTSR"]
    pub sctm_intsr: SCTM_INTSR,
    #[doc = "0x80 - SCTM_CNTR"]
    pub sctm_cntr: SCTM_CNTR,
    #[doc = "0x84 - SCTM_PSCR"]
    pub sctm_pscr: SCTM_PSCR,
    #[doc = "0x88 - SCTM_CRR"]
    pub sctm_crr: SCTM_CRR,
    _reserved14: [u8; 0x04],
    #[doc = "0x90 - SCTM_CHCCR"]
    pub sctm_chccr: SCTM_CHCCR,
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
#[doc = "SCTM_CHICFR (rw) register accessor: an alias for `Reg<SCTM_CHICFR_SPEC>`"]
pub type SCTM_CHICFR = crate::Reg<sctm_chicfr::SCTM_CHICFR_SPEC>;
#[doc = "SCTM_CHICFR"]
pub mod sctm_chicfr;
#[doc = "SCTM_CHOCFR (rw) register accessor: an alias for `Reg<SCTM_CHOCFR_SPEC>`"]
pub type SCTM_CHOCFR = crate::Reg<sctm_chocfr::SCTM_CHOCFR_SPEC>;
#[doc = "SCTM_CHOCFR"]
pub mod sctm_chocfr;
#[doc = "SCTM_CHCTR (rw) register accessor: an alias for `Reg<SCTM_CHCTR_SPEC>`"]
pub type SCTM_CHCTR = crate::Reg<sctm_chctr::SCTM_CHCTR_SPEC>;
#[doc = "SCTM_CHCTR"]
pub mod sctm_chctr;
#[doc = "SCTM_CHPOLR (rw) register accessor: an alias for `Reg<SCTM_CHPOLR_SPEC>`"]
pub type SCTM_CHPOLR = crate::Reg<sctm_chpolr::SCTM_CHPOLR_SPEC>;
#[doc = "SCTM_CHPOLR"]
pub mod sctm_chpolr;
#[doc = "SCTM_DICTR (rw) register accessor: an alias for `Reg<SCTM_DICTR_SPEC>`"]
pub type SCTM_DICTR = crate::Reg<sctm_dictr::SCTM_DICTR_SPEC>;
#[doc = "SCTM_DICTR"]
pub mod sctm_dictr;
#[doc = "SCTM_EVGR (rw) register accessor: an alias for `Reg<SCTM_EVGR_SPEC>`"]
pub type SCTM_EVGR = crate::Reg<sctm_evgr::SCTM_EVGR_SPEC>;
#[doc = "SCTM_EVGR"]
pub mod sctm_evgr;
#[doc = "SCTM_INTSR (rw) register accessor: an alias for `Reg<SCTM_INTSR_SPEC>`"]
pub type SCTM_INTSR = crate::Reg<sctm_intsr::SCTM_INTSR_SPEC>;
#[doc = "SCTM_INTSR"]
pub mod sctm_intsr;
#[doc = "SCTM_CNTR (rw) register accessor: an alias for `Reg<SCTM_CNTR_SPEC>`"]
pub type SCTM_CNTR = crate::Reg<sctm_cntr::SCTM_CNTR_SPEC>;
#[doc = "SCTM_CNTR"]
pub mod sctm_cntr;
#[doc = "SCTM_PSCR (rw) register accessor: an alias for `Reg<SCTM_PSCR_SPEC>`"]
pub type SCTM_PSCR = crate::Reg<sctm_pscr::SCTM_PSCR_SPEC>;
#[doc = "SCTM_PSCR"]
pub mod sctm_pscr;
#[doc = "SCTM_CRR (rw) register accessor: an alias for `Reg<SCTM_CRR_SPEC>`"]
pub type SCTM_CRR = crate::Reg<sctm_crr::SCTM_CRR_SPEC>;
#[doc = "SCTM_CRR"]
pub mod sctm_crr;
#[doc = "SCTM_CHCCR (rw) register accessor: an alias for `Reg<SCTM_CHCCR_SPEC>`"]
pub type SCTM_CHCCR = crate::Reg<sctm_chccr::SCTM_CHCCR_SPEC>;
#[doc = "SCTM_CHCCR"]
pub mod sctm_chccr;
