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
    _reserved4: [u8; 0x2c],
    #[doc = "0x40 - CH0OCFR"]
    pub ch0ocfr: CH0OCFR,
    #[doc = "0x44 - CH1OCFR"]
    pub ch1ocfr: CH1OCFR,
    #[doc = "0x48 - CH2OCFR"]
    pub ch2ocfr: CH2OCFR,
    #[doc = "0x4c - CH3OCFR"]
    pub ch3ocfr: CH3OCFR,
    #[doc = "0x50 - CHCTR"]
    pub chctr: CHCTR,
    #[doc = "0x54 - CHPOLR"]
    pub chpolr: CHPOLR,
    _reserved10: [u8; 0x1c],
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
    _reserved16: [u8; 0x04],
    #[doc = "0x90 - CH0CR"]
    pub ch0cr: CH0CR,
    #[doc = "0x94 - CH1CR"]
    pub ch1cr: CH1CR,
    #[doc = "0x98 - CH2CR"]
    pub ch2cr: CH2CR,
    #[doc = "0x9c - CH3CR"]
    pub ch3cr: CH3CR,
    #[doc = "0xa0 - CH0ACR"]
    pub ch0acr: CH0ACR,
    #[doc = "0xa4 - CH1ACR"]
    pub ch1acr: CH1ACR,
    #[doc = "0xa8 - CH2ACR"]
    pub ch2acr: CH2ACR,
    #[doc = "0xac - CH3ACR"]
    pub ch3acr: CH3ACR,
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
#[doc = "CH0OCFR (rw) register accessor: an alias for `Reg<CH0OCFR_SPEC>`"]
pub type CH0OCFR = crate::Reg<ch0ocfr::CH0OCFR_SPEC>;
#[doc = "CH0OCFR"]
pub mod ch0ocfr;
#[doc = "CH1OCFR (rw) register accessor: an alias for `Reg<CH1OCFR_SPEC>`"]
pub type CH1OCFR = crate::Reg<ch1ocfr::CH1OCFR_SPEC>;
#[doc = "CH1OCFR"]
pub mod ch1ocfr;
#[doc = "CH2OCFR (rw) register accessor: an alias for `Reg<CH2OCFR_SPEC>`"]
pub type CH2OCFR = crate::Reg<ch2ocfr::CH2OCFR_SPEC>;
#[doc = "CH2OCFR"]
pub mod ch2ocfr;
#[doc = "CH3OCFR (rw) register accessor: an alias for `Reg<CH3OCFR_SPEC>`"]
pub type CH3OCFR = crate::Reg<ch3ocfr::CH3OCFR_SPEC>;
#[doc = "CH3OCFR"]
pub mod ch3ocfr;
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
#[doc = "CH0CR (rw) register accessor: an alias for `Reg<CH0CR_SPEC>`"]
pub type CH0CR = crate::Reg<ch0cr::CH0CR_SPEC>;
#[doc = "CH0CR"]
pub mod ch0cr;
#[doc = "CH1CR (rw) register accessor: an alias for `Reg<CH1CR_SPEC>`"]
pub type CH1CR = crate::Reg<ch1cr::CH1CR_SPEC>;
#[doc = "CH1CR"]
pub mod ch1cr;
#[doc = "CH2CR (rw) register accessor: an alias for `Reg<CH2CR_SPEC>`"]
pub type CH2CR = crate::Reg<ch2cr::CH2CR_SPEC>;
#[doc = "CH2CR"]
pub mod ch2cr;
#[doc = "CH3CR (rw) register accessor: an alias for `Reg<CH3CR_SPEC>`"]
pub type CH3CR = crate::Reg<ch3cr::CH3CR_SPEC>;
#[doc = "CH3CR"]
pub mod ch3cr;
#[doc = "CH0ACR (rw) register accessor: an alias for `Reg<CH0ACR_SPEC>`"]
pub type CH0ACR = crate::Reg<ch0acr::CH0ACR_SPEC>;
#[doc = "CH0ACR"]
pub mod ch0acr;
#[doc = "CH1ACR (rw) register accessor: an alias for `Reg<CH1ACR_SPEC>`"]
pub type CH1ACR = crate::Reg<ch1acr::CH1ACR_SPEC>;
#[doc = "CH1ACR"]
pub mod ch1acr;
#[doc = "CH2ACR (rw) register accessor: an alias for `Reg<CH2ACR_SPEC>`"]
pub type CH2ACR = crate::Reg<ch2acr::CH2ACR_SPEC>;
#[doc = "CH2ACR"]
pub mod ch2acr;
#[doc = "CH3ACR (rw) register accessor: an alias for `Reg<CH3ACR_SPEC>`"]
pub type CH3ACR = crate::Reg<ch3acr::CH3ACR_SPEC>;
#[doc = "CH3ACR"]
pub mod ch3acr;
