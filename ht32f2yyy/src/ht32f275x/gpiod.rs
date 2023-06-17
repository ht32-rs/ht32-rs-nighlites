#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOD_DIRCR"]
    pub gpiod_dircr: GPIOD_DIRCR,
    #[doc = "0x04 - GPIOD_INER"]
    pub gpiod_iner: GPIOD_INER,
    #[doc = "0x08 - GPIOD_PUR"]
    pub gpiod_pur: GPIOD_PUR,
    #[doc = "0x0c - GPIOD_PDR"]
    pub gpiod_pdr: GPIOD_PDR,
    #[doc = "0x10 - GPIOD_ODR"]
    pub gpiod_odr: GPIOD_ODR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - GPIOD_LOCKR"]
    pub gpiod_lockr: GPIOD_LOCKR,
    #[doc = "0x1c - GPIOD_DINR"]
    pub gpiod_dinr: GPIOD_DINR,
    #[doc = "0x20 - GPIOD_DOUTR"]
    pub gpiod_doutr: GPIOD_DOUTR,
    #[doc = "0x24 - GPIOD_SRR"]
    pub gpiod_srr: GPIOD_SRR,
    #[doc = "0x28 - GPIOD_RR"]
    pub gpiod_rr: GPIOD_RR,
}
#[doc = "GPIOD_DIRCR (rw) register accessor: an alias for `Reg<GPIOD_DIRCR_SPEC>`"]
pub type GPIOD_DIRCR = crate::Reg<gpiod_dircr::GPIOD_DIRCR_SPEC>;
#[doc = "GPIOD_DIRCR"]
pub mod gpiod_dircr;
#[doc = "GPIOD_INER (rw) register accessor: an alias for `Reg<GPIOD_INER_SPEC>`"]
pub type GPIOD_INER = crate::Reg<gpiod_iner::GPIOD_INER_SPEC>;
#[doc = "GPIOD_INER"]
pub mod gpiod_iner;
#[doc = "GPIOD_PUR (rw) register accessor: an alias for `Reg<GPIOD_PUR_SPEC>`"]
pub type GPIOD_PUR = crate::Reg<gpiod_pur::GPIOD_PUR_SPEC>;
#[doc = "GPIOD_PUR"]
pub mod gpiod_pur;
#[doc = "GPIOD_PDR (rw) register accessor: an alias for `Reg<GPIOD_PDR_SPEC>`"]
pub type GPIOD_PDR = crate::Reg<gpiod_pdr::GPIOD_PDR_SPEC>;
#[doc = "GPIOD_PDR"]
pub mod gpiod_pdr;
#[doc = "GPIOD_ODR (rw) register accessor: an alias for `Reg<GPIOD_ODR_SPEC>`"]
pub type GPIOD_ODR = crate::Reg<gpiod_odr::GPIOD_ODR_SPEC>;
#[doc = "GPIOD_ODR"]
pub mod gpiod_odr;
#[doc = "GPIOD_LOCKR (rw) register accessor: an alias for `Reg<GPIOD_LOCKR_SPEC>`"]
pub type GPIOD_LOCKR = crate::Reg<gpiod_lockr::GPIOD_LOCKR_SPEC>;
#[doc = "GPIOD_LOCKR"]
pub mod gpiod_lockr;
#[doc = "GPIOD_DINR (rw) register accessor: an alias for `Reg<GPIOD_DINR_SPEC>`"]
pub type GPIOD_DINR = crate::Reg<gpiod_dinr::GPIOD_DINR_SPEC>;
#[doc = "GPIOD_DINR"]
pub mod gpiod_dinr;
#[doc = "GPIOD_DOUTR (rw) register accessor: an alias for `Reg<GPIOD_DOUTR_SPEC>`"]
pub type GPIOD_DOUTR = crate::Reg<gpiod_doutr::GPIOD_DOUTR_SPEC>;
#[doc = "GPIOD_DOUTR"]
pub mod gpiod_doutr;
#[doc = "GPIOD_SRR (rw) register accessor: an alias for `Reg<GPIOD_SRR_SPEC>`"]
pub type GPIOD_SRR = crate::Reg<gpiod_srr::GPIOD_SRR_SPEC>;
#[doc = "GPIOD_SRR"]
pub mod gpiod_srr;
#[doc = "GPIOD_RR (rw) register accessor: an alias for `Reg<GPIOD_RR_SPEC>`"]
pub type GPIOD_RR = crate::Reg<gpiod_rr::GPIOD_RR_SPEC>;
#[doc = "GPIOD_RR"]
pub mod gpiod_rr;
