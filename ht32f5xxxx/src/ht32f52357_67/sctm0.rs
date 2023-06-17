#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CNTCFR"]
    pub cntcfr: CNTCFR,
    #[doc = "0x04 - MDCFR"]
    pub mdcfr: MDCFR,
    #[doc = "0x08 - TRCFR"]
    pub trcfr: TRCFR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - CTR"]
    pub ctr: CTR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - CHICFR"]
    pub chicfr: CHICFR,
    _reserved5: [u8; 0x1c],
    #[doc = "0x40 - CHOCFR"]
    pub chocfr: CHOCFR,
    _reserved6: [u8; 0x0c],
    #[doc = "0x50 - CHCTR"]
    pub chctr: CHCTR,
    #[doc = "0x54 - CHPOLR"]
    pub chpolr: CHPOLR,
    _reserved8: [u8; 0x1c],
    #[doc = "0x74 - DICTR"]
    pub dictr: DICTR,
    #[doc = "0x78 - EVGR"]
    pub evgr: EVGR,
    #[doc = "0x7c - INTSR"]
    pub intsr: INTSR,
    #[doc = "0x80 - CNTR"]
    pub cntr: CNTR,
    #[doc = "0x84 - PSCR"]
    pub pscr: PSCR,
    #[doc = "0x88 - CRR"]
    pub crr: CRR,
    _reserved14: [u8; 0x04],
    #[doc = "0x90 - CHCCR"]
    pub chccr: CHCCR,
}
#[doc = "CNTCFR (rw) register accessor: an alias for `Reg<CNTCFR_SPEC>`"]
pub type CNTCFR = crate::Reg<cntcfr::CNTCFR_SPEC>;
#[doc = "CNTCFR"]
pub mod cntcfr;
#[doc = "MDCFR (rw) register accessor: an alias for `Reg<MDCFR_SPEC>`"]
pub type MDCFR = crate::Reg<mdcfr::MDCFR_SPEC>;
#[doc = "MDCFR"]
pub mod mdcfr;
#[doc = "TRCFR (rw) register accessor: an alias for `Reg<TRCFR_SPEC>`"]
pub type TRCFR = crate::Reg<trcfr::TRCFR_SPEC>;
#[doc = "TRCFR"]
pub mod trcfr;
#[doc = "CTR (rw) register accessor: an alias for `Reg<CTR_SPEC>`"]
pub type CTR = crate::Reg<ctr::CTR_SPEC>;
#[doc = "CTR"]
pub mod ctr;
#[doc = "CHICFR (rw) register accessor: an alias for `Reg<CHICFR_SPEC>`"]
pub type CHICFR = crate::Reg<chicfr::CHICFR_SPEC>;
#[doc = "CHICFR"]
pub mod chicfr;
#[doc = "CHOCFR (rw) register accessor: an alias for `Reg<CHOCFR_SPEC>`"]
pub type CHOCFR = crate::Reg<chocfr::CHOCFR_SPEC>;
#[doc = "CHOCFR"]
pub mod chocfr;
#[doc = "CHCTR (rw) register accessor: an alias for `Reg<CHCTR_SPEC>`"]
pub type CHCTR = crate::Reg<chctr::CHCTR_SPEC>;
#[doc = "CHCTR"]
pub mod chctr;
#[doc = "CHPOLR (rw) register accessor: an alias for `Reg<CHPOLR_SPEC>`"]
pub type CHPOLR = crate::Reg<chpolr::CHPOLR_SPEC>;
#[doc = "CHPOLR"]
pub mod chpolr;
#[doc = "DICTR (rw) register accessor: an alias for `Reg<DICTR_SPEC>`"]
pub type DICTR = crate::Reg<dictr::DICTR_SPEC>;
#[doc = "DICTR"]
pub mod dictr;
#[doc = "EVGR (rw) register accessor: an alias for `Reg<EVGR_SPEC>`"]
pub type EVGR = crate::Reg<evgr::EVGR_SPEC>;
#[doc = "EVGR"]
pub mod evgr;
#[doc = "INTSR (rw) register accessor: an alias for `Reg<INTSR_SPEC>`"]
pub type INTSR = crate::Reg<intsr::INTSR_SPEC>;
#[doc = "INTSR"]
pub mod intsr;
#[doc = "CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "CNTR"]
pub mod cntr;
#[doc = "PSCR (rw) register accessor: an alias for `Reg<PSCR_SPEC>`"]
pub type PSCR = crate::Reg<pscr::PSCR_SPEC>;
#[doc = "PSCR"]
pub mod pscr;
#[doc = "CRR (rw) register accessor: an alias for `Reg<CRR_SPEC>`"]
pub type CRR = crate::Reg<crr::CRR_SPEC>;
#[doc = "CRR"]
pub mod crr;
#[doc = "CHCCR (rw) register accessor: an alias for `Reg<CHCCR_SPEC>`"]
pub type CHCCR = crate::Reg<chccr::CHCCR_SPEC>;
#[doc = "CHCCR"]
pub mod chccr;
