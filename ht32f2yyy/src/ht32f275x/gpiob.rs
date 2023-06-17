#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOB_DIRCR"]
    pub gpiob_dircr: GPIOB_DIRCR,
    #[doc = "0x04 - GPIOB_INER"]
    pub gpiob_iner: GPIOB_INER,
    #[doc = "0x08 - GPIOB_PUR"]
    pub gpiob_pur: GPIOB_PUR,
    #[doc = "0x0c - GPIOB_PDR"]
    pub gpiob_pdr: GPIOB_PDR,
    #[doc = "0x10 - GPIOB_ODR"]
    pub gpiob_odr: GPIOB_ODR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - GPIOB_LOCKR"]
    pub gpiob_lockr: GPIOB_LOCKR,
    #[doc = "0x1c - GPIOB_DINR"]
    pub gpiob_dinr: GPIOB_DINR,
    #[doc = "0x20 - GPIOB_DOUTR"]
    pub gpiob_doutr: GPIOB_DOUTR,
    #[doc = "0x24 - GPIOB_SRR"]
    pub gpiob_srr: GPIOB_SRR,
    #[doc = "0x28 - GPIOB_RR"]
    pub gpiob_rr: GPIOB_RR,
}
#[doc = "GPIOB_DIRCR (rw) register accessor: an alias for `Reg<GPIOB_DIRCR_SPEC>`"]
pub type GPIOB_DIRCR = crate::Reg<gpiob_dircr::GPIOB_DIRCR_SPEC>;
#[doc = "GPIOB_DIRCR"]
pub mod gpiob_dircr;
#[doc = "GPIOB_INER (rw) register accessor: an alias for `Reg<GPIOB_INER_SPEC>`"]
pub type GPIOB_INER = crate::Reg<gpiob_iner::GPIOB_INER_SPEC>;
#[doc = "GPIOB_INER"]
pub mod gpiob_iner;
#[doc = "GPIOB_PUR (rw) register accessor: an alias for `Reg<GPIOB_PUR_SPEC>`"]
pub type GPIOB_PUR = crate::Reg<gpiob_pur::GPIOB_PUR_SPEC>;
#[doc = "GPIOB_PUR"]
pub mod gpiob_pur;
#[doc = "GPIOB_PDR (rw) register accessor: an alias for `Reg<GPIOB_PDR_SPEC>`"]
pub type GPIOB_PDR = crate::Reg<gpiob_pdr::GPIOB_PDR_SPEC>;
#[doc = "GPIOB_PDR"]
pub mod gpiob_pdr;
#[doc = "GPIOB_ODR (rw) register accessor: an alias for `Reg<GPIOB_ODR_SPEC>`"]
pub type GPIOB_ODR = crate::Reg<gpiob_odr::GPIOB_ODR_SPEC>;
#[doc = "GPIOB_ODR"]
pub mod gpiob_odr;
#[doc = "GPIOB_LOCKR (rw) register accessor: an alias for `Reg<GPIOB_LOCKR_SPEC>`"]
pub type GPIOB_LOCKR = crate::Reg<gpiob_lockr::GPIOB_LOCKR_SPEC>;
#[doc = "GPIOB_LOCKR"]
pub mod gpiob_lockr;
#[doc = "GPIOB_DINR (rw) register accessor: an alias for `Reg<GPIOB_DINR_SPEC>`"]
pub type GPIOB_DINR = crate::Reg<gpiob_dinr::GPIOB_DINR_SPEC>;
#[doc = "GPIOB_DINR"]
pub mod gpiob_dinr;
#[doc = "GPIOB_DOUTR (rw) register accessor: an alias for `Reg<GPIOB_DOUTR_SPEC>`"]
pub type GPIOB_DOUTR = crate::Reg<gpiob_doutr::GPIOB_DOUTR_SPEC>;
#[doc = "GPIOB_DOUTR"]
pub mod gpiob_doutr;
#[doc = "GPIOB_SRR (rw) register accessor: an alias for `Reg<GPIOB_SRR_SPEC>`"]
pub type GPIOB_SRR = crate::Reg<gpiob_srr::GPIOB_SRR_SPEC>;
#[doc = "GPIOB_SRR"]
pub mod gpiob_srr;
#[doc = "GPIOB_RR (rw) register accessor: an alias for `Reg<GPIOB_RR_SPEC>`"]
pub type GPIOB_RR = crate::Reg<gpiob_rr::GPIOB_RR_SPEC>;
#[doc = "GPIOB_RR"]
pub mod gpiob_rr;
