#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCTM_CNTCFR"]
    pub mctm_cntcfr: MCTM_CNTCFR,
    #[doc = "0x04 - MCTM_MDCFR"]
    pub mctm_mdcfr: MCTM_MDCFR,
    #[doc = "0x08 - MCTM_TRCFR"]
    pub mctm_trcfr: MCTM_TRCFR,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - MCTM_CTR"]
    pub mctm_ctr: MCTM_CTR,
    _reserved4: [u8; 0x0c],
    #[doc = "0x20 - MCTM_CH0ICFR"]
    pub mctm_ch0icfr: MCTM_CH0ICFR,
    #[doc = "0x24 - MCTM_CH1ICFR"]
    pub mctm_ch1icfr: MCTM_CH1ICFR,
    #[doc = "0x28 - MCTM_CH2ICFR"]
    pub mctm_ch2icfr: MCTM_CH2ICFR,
    #[doc = "0x2c - MCTM_CH3ICFR"]
    pub mctm_ch3icfr: MCTM_CH3ICFR,
    _reserved8: [u8; 0x10],
    #[doc = "0x40 - MCTM_CH0OCFR"]
    pub mctm_ch0ocfr: MCTM_CH0OCFR,
    #[doc = "0x44 - MCTM_CH1OCFR"]
    pub mctm_ch1ocfr: MCTM_CH1OCFR,
    #[doc = "0x48 - MCTM_CH2OCFR"]
    pub mctm_ch2ocfr: MCTM_CH2OCFR,
    #[doc = "0x4c - MCTM_CH3OCFR"]
    pub mctm_ch3ocfr: MCTM_CH3OCFR,
    #[doc = "0x50 - MCTM_CHCTR"]
    pub mctm_chctr: MCTM_CHCTR,
    #[doc = "0x54 - MCTM_CHPOLR"]
    pub mctm_chpolr: MCTM_CHPOLR,
    _reserved14: [u8; 0x14],
    #[doc = "0x6c - MCTM_CHBRKCFR"]
    pub mctm_chbrkcfr: MCTM_CHBRKCFR,
    #[doc = "0x70 - MCTM_CHBRKCTR"]
    pub mctm_chbrkctr: MCTM_CHBRKCTR,
    #[doc = "0x74 - MCTM_DICTR"]
    pub mctm_dictr: MCTM_DICTR,
    #[doc = "0x78 - MCTM_EVGR"]
    pub mctm_evgr: MCTM_EVGR,
    #[doc = "0x7c - MCTM_INTSR"]
    pub mctm_intsr: MCTM_INTSR,
    #[doc = "0x80 - MCTM_CNTR"]
    pub mctm_cntr: MCTM_CNTR,
    #[doc = "0x84 - MCTM_PSCR"]
    pub mctm_pscr: MCTM_PSCR,
    #[doc = "0x88 - MCTM_CRR"]
    pub mctm_crr: MCTM_CRR,
    #[doc = "0x8c - MCTM_REPR"]
    pub mctm_repr: MCTM_REPR,
    #[doc = "0x90 - MCTM_CH0CCR"]
    pub mctm_ch0ccr: MCTM_CH0CCR,
    #[doc = "0x94 - MCTM_CH1CCR"]
    pub mctm_ch1ccr: MCTM_CH1CCR,
    #[doc = "0x98 - MCTM_CH2CCR"]
    pub mctm_ch2ccr: MCTM_CH2CCR,
    #[doc = "0x9c - MCTM_CH3CCR"]
    pub mctm_ch3ccr: MCTM_CH3CCR,
}
#[doc = "MCTM_CNTCFR (rw) register accessor: an alias for `Reg<MCTM_CNTCFR_SPEC>`"]
pub type MCTM_CNTCFR = crate::Reg<mctm_cntcfr::MCTM_CNTCFR_SPEC>;
#[doc = "MCTM_CNTCFR"]
pub mod mctm_cntcfr;
#[doc = "MCTM_MDCFR (rw) register accessor: an alias for `Reg<MCTM_MDCFR_SPEC>`"]
pub type MCTM_MDCFR = crate::Reg<mctm_mdcfr::MCTM_MDCFR_SPEC>;
#[doc = "MCTM_MDCFR"]
pub mod mctm_mdcfr;
#[doc = "MCTM_TRCFR (rw) register accessor: an alias for `Reg<MCTM_TRCFR_SPEC>`"]
pub type MCTM_TRCFR = crate::Reg<mctm_trcfr::MCTM_TRCFR_SPEC>;
#[doc = "MCTM_TRCFR"]
pub mod mctm_trcfr;
#[doc = "MCTM_CTR (rw) register accessor: an alias for `Reg<MCTM_CTR_SPEC>`"]
pub type MCTM_CTR = crate::Reg<mctm_ctr::MCTM_CTR_SPEC>;
#[doc = "MCTM_CTR"]
pub mod mctm_ctr;
#[doc = "MCTM_CH0ICFR (rw) register accessor: an alias for `Reg<MCTM_CH0ICFR_SPEC>`"]
pub type MCTM_CH0ICFR = crate::Reg<mctm_ch0icfr::MCTM_CH0ICFR_SPEC>;
#[doc = "MCTM_CH0ICFR"]
pub mod mctm_ch0icfr;
#[doc = "MCTM_CH1ICFR (rw) register accessor: an alias for `Reg<MCTM_CH1ICFR_SPEC>`"]
pub type MCTM_CH1ICFR = crate::Reg<mctm_ch1icfr::MCTM_CH1ICFR_SPEC>;
#[doc = "MCTM_CH1ICFR"]
pub mod mctm_ch1icfr;
#[doc = "MCTM_CH2ICFR (rw) register accessor: an alias for `Reg<MCTM_CH2ICFR_SPEC>`"]
pub type MCTM_CH2ICFR = crate::Reg<mctm_ch2icfr::MCTM_CH2ICFR_SPEC>;
#[doc = "MCTM_CH2ICFR"]
pub mod mctm_ch2icfr;
#[doc = "MCTM_CH3ICFR (rw) register accessor: an alias for `Reg<MCTM_CH3ICFR_SPEC>`"]
pub type MCTM_CH3ICFR = crate::Reg<mctm_ch3icfr::MCTM_CH3ICFR_SPEC>;
#[doc = "MCTM_CH3ICFR"]
pub mod mctm_ch3icfr;
#[doc = "MCTM_CH0OCFR (rw) register accessor: an alias for `Reg<MCTM_CH0OCFR_SPEC>`"]
pub type MCTM_CH0OCFR = crate::Reg<mctm_ch0ocfr::MCTM_CH0OCFR_SPEC>;
#[doc = "MCTM_CH0OCFR"]
pub mod mctm_ch0ocfr;
#[doc = "MCTM_CH1OCFR (rw) register accessor: an alias for `Reg<MCTM_CH1OCFR_SPEC>`"]
pub type MCTM_CH1OCFR = crate::Reg<mctm_ch1ocfr::MCTM_CH1OCFR_SPEC>;
#[doc = "MCTM_CH1OCFR"]
pub mod mctm_ch1ocfr;
#[doc = "MCTM_CH2OCFR (rw) register accessor: an alias for `Reg<MCTM_CH2OCFR_SPEC>`"]
pub type MCTM_CH2OCFR = crate::Reg<mctm_ch2ocfr::MCTM_CH2OCFR_SPEC>;
#[doc = "MCTM_CH2OCFR"]
pub mod mctm_ch2ocfr;
#[doc = "MCTM_CH3OCFR (rw) register accessor: an alias for `Reg<MCTM_CH3OCFR_SPEC>`"]
pub type MCTM_CH3OCFR = crate::Reg<mctm_ch3ocfr::MCTM_CH3OCFR_SPEC>;
#[doc = "MCTM_CH3OCFR"]
pub mod mctm_ch3ocfr;
#[doc = "MCTM_CHCTR (rw) register accessor: an alias for `Reg<MCTM_CHCTR_SPEC>`"]
pub type MCTM_CHCTR = crate::Reg<mctm_chctr::MCTM_CHCTR_SPEC>;
#[doc = "MCTM_CHCTR"]
pub mod mctm_chctr;
#[doc = "MCTM_CHPOLR (rw) register accessor: an alias for `Reg<MCTM_CHPOLR_SPEC>`"]
pub type MCTM_CHPOLR = crate::Reg<mctm_chpolr::MCTM_CHPOLR_SPEC>;
#[doc = "MCTM_CHPOLR"]
pub mod mctm_chpolr;
#[doc = "MCTM_CHBRKCFR (rw) register accessor: an alias for `Reg<MCTM_CHBRKCFR_SPEC>`"]
pub type MCTM_CHBRKCFR = crate::Reg<mctm_chbrkcfr::MCTM_CHBRKCFR_SPEC>;
#[doc = "MCTM_CHBRKCFR"]
pub mod mctm_chbrkcfr;
#[doc = "MCTM_CHBRKCTR (rw) register accessor: an alias for `Reg<MCTM_CHBRKCTR_SPEC>`"]
pub type MCTM_CHBRKCTR = crate::Reg<mctm_chbrkctr::MCTM_CHBRKCTR_SPEC>;
#[doc = "MCTM_CHBRKCTR"]
pub mod mctm_chbrkctr;
#[doc = "MCTM_DICTR (rw) register accessor: an alias for `Reg<MCTM_DICTR_SPEC>`"]
pub type MCTM_DICTR = crate::Reg<mctm_dictr::MCTM_DICTR_SPEC>;
#[doc = "MCTM_DICTR"]
pub mod mctm_dictr;
#[doc = "MCTM_EVGR (rw) register accessor: an alias for `Reg<MCTM_EVGR_SPEC>`"]
pub type MCTM_EVGR = crate::Reg<mctm_evgr::MCTM_EVGR_SPEC>;
#[doc = "MCTM_EVGR"]
pub mod mctm_evgr;
#[doc = "MCTM_INTSR (rw) register accessor: an alias for `Reg<MCTM_INTSR_SPEC>`"]
pub type MCTM_INTSR = crate::Reg<mctm_intsr::MCTM_INTSR_SPEC>;
#[doc = "MCTM_INTSR"]
pub mod mctm_intsr;
#[doc = "MCTM_CNTR (rw) register accessor: an alias for `Reg<MCTM_CNTR_SPEC>`"]
pub type MCTM_CNTR = crate::Reg<mctm_cntr::MCTM_CNTR_SPEC>;
#[doc = "MCTM_CNTR"]
pub mod mctm_cntr;
#[doc = "MCTM_PSCR (rw) register accessor: an alias for `Reg<MCTM_PSCR_SPEC>`"]
pub type MCTM_PSCR = crate::Reg<mctm_pscr::MCTM_PSCR_SPEC>;
#[doc = "MCTM_PSCR"]
pub mod mctm_pscr;
#[doc = "MCTM_CRR (rw) register accessor: an alias for `Reg<MCTM_CRR_SPEC>`"]
pub type MCTM_CRR = crate::Reg<mctm_crr::MCTM_CRR_SPEC>;
#[doc = "MCTM_CRR"]
pub mod mctm_crr;
#[doc = "MCTM_REPR (rw) register accessor: an alias for `Reg<MCTM_REPR_SPEC>`"]
pub type MCTM_REPR = crate::Reg<mctm_repr::MCTM_REPR_SPEC>;
#[doc = "MCTM_REPR"]
pub mod mctm_repr;
#[doc = "MCTM_CH0CCR (rw) register accessor: an alias for `Reg<MCTM_CH0CCR_SPEC>`"]
pub type MCTM_CH0CCR = crate::Reg<mctm_ch0ccr::MCTM_CH0CCR_SPEC>;
#[doc = "MCTM_CH0CCR"]
pub mod mctm_ch0ccr;
#[doc = "MCTM_CH1CCR (rw) register accessor: an alias for `Reg<MCTM_CH1CCR_SPEC>`"]
pub type MCTM_CH1CCR = crate::Reg<mctm_ch1ccr::MCTM_CH1CCR_SPEC>;
#[doc = "MCTM_CH1CCR"]
pub mod mctm_ch1ccr;
#[doc = "MCTM_CH2CCR (rw) register accessor: an alias for `Reg<MCTM_CH2CCR_SPEC>`"]
pub type MCTM_CH2CCR = crate::Reg<mctm_ch2ccr::MCTM_CH2CCR_SPEC>;
#[doc = "MCTM_CH2CCR"]
pub mod mctm_ch2ccr;
#[doc = "MCTM_CH3CCR (rw) register accessor: an alias for `Reg<MCTM_CH3CCR_SPEC>`"]
pub type MCTM_CH3CCR = crate::Reg<mctm_ch3ccr::MCTM_CH3CCR_SPEC>;
#[doc = "MCTM_CH3CCR"]
pub mod mctm_ch3ccr;
