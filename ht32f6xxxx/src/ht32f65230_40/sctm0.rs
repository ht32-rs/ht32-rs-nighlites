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
    #[doc = "0x20 - CH0ICFR"]
    pub ch0icfr: CH0ICFR,
    #[doc = "0x24 - CH1ICFR"]
    pub ch1icfr: CH1ICFR,
    _reserved6: [u8; 0x18],
    #[doc = "0x40 - CHOCFR"]
    pub chocfr: CHOCFR,
    _reserved7: [u8; 0x0c],
    #[doc = "0x50 - CHCTR"]
    pub chctr: CHCTR,
    #[doc = "0x54 - CHPOLR"]
    pub chpolr: CHPOLR,
    _reserved9: [u8; 0x1c],
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
    _reserved15: [u8; 0x04],
    #[doc = "0x90 - CH0CCR"]
    pub ch0ccr: CH0CCR,
    #[doc = "0x94 - CH1CCR"]
    pub ch1ccr: CH1CCR,
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
#[doc = "CH0ICFR (rw) register accessor: an alias for `Reg<CH0ICFR_SPEC>`"]
pub type CH0ICFR = crate::Reg<ch0icfr::CH0ICFR_SPEC>;
#[doc = "CH0ICFR"]
pub mod ch0icfr;
#[doc = "CH1ICFR (rw) register accessor: an alias for `Reg<CH1ICFR_SPEC>`"]
pub type CH1ICFR = crate::Reg<ch1icfr::CH1ICFR_SPEC>;
#[doc = "CH1ICFR"]
pub mod ch1icfr;
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
#[doc = "CH0CCR (rw) register accessor: an alias for `Reg<CH0CCR_SPEC>`"]
pub type CH0CCR = crate::Reg<ch0ccr::CH0CCR_SPEC>;
#[doc = "CH0CCR"]
pub mod ch0ccr;
#[doc = "CH1CCR (rw) register accessor: an alias for `Reg<CH1CCR_SPEC>`"]
pub type CH1CCR = crate::Reg<ch1ccr::CH1CCR_SPEC>;
#[doc = "CH1CCR"]
pub mod ch1ccr;
