#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOC_DIRCR"]
    pub gpioc_dircr: GPIOC_DIRCR,
    #[doc = "0x04 - GPIOC_INER"]
    pub gpioc_iner: GPIOC_INER,
    #[doc = "0x08 - GPIOC_PUR"]
    pub gpioc_pur: GPIOC_PUR,
    #[doc = "0x0c - GPIOC_PDR"]
    pub gpioc_pdr: GPIOC_PDR,
    #[doc = "0x10 - GPIOC_ODR"]
    pub gpioc_odr: GPIOC_ODR,
    #[doc = "0x14 - GPIOC_DRVR"]
    pub gpioc_drvr: GPIOC_DRVR,
    #[doc = "0x18 - GPIOC_LOCKR"]
    pub gpioc_lockr: GPIOC_LOCKR,
    #[doc = "0x1c - GPIOC_DINR"]
    pub gpioc_dinr: GPIOC_DINR,
    #[doc = "0x20 - GPIOC_DOUTR"]
    pub gpioc_doutr: GPIOC_DOUTR,
    #[doc = "0x24 - GPIOC_SRR"]
    pub gpioc_srr: GPIOC_SRR,
    #[doc = "0x28 - GPIOC_RR"]
    pub gpioc_rr: GPIOC_RR,
}
#[doc = "GPIOC_DIRCR (rw) register accessor: an alias for `Reg<GPIOC_DIRCR_SPEC>`"]
pub type GPIOC_DIRCR = crate::Reg<gpioc_dircr::GPIOC_DIRCR_SPEC>;
#[doc = "GPIOC_DIRCR"]
pub mod gpioc_dircr;
#[doc = "GPIOC_INER (rw) register accessor: an alias for `Reg<GPIOC_INER_SPEC>`"]
pub type GPIOC_INER = crate::Reg<gpioc_iner::GPIOC_INER_SPEC>;
#[doc = "GPIOC_INER"]
pub mod gpioc_iner;
#[doc = "GPIOC_PUR (rw) register accessor: an alias for `Reg<GPIOC_PUR_SPEC>`"]
pub type GPIOC_PUR = crate::Reg<gpioc_pur::GPIOC_PUR_SPEC>;
#[doc = "GPIOC_PUR"]
pub mod gpioc_pur;
#[doc = "GPIOC_PDR (rw) register accessor: an alias for `Reg<GPIOC_PDR_SPEC>`"]
pub type GPIOC_PDR = crate::Reg<gpioc_pdr::GPIOC_PDR_SPEC>;
#[doc = "GPIOC_PDR"]
pub mod gpioc_pdr;
#[doc = "GPIOC_ODR (rw) register accessor: an alias for `Reg<GPIOC_ODR_SPEC>`"]
pub type GPIOC_ODR = crate::Reg<gpioc_odr::GPIOC_ODR_SPEC>;
#[doc = "GPIOC_ODR"]
pub mod gpioc_odr;
#[doc = "GPIOC_DRVR (rw) register accessor: an alias for `Reg<GPIOC_DRVR_SPEC>`"]
pub type GPIOC_DRVR = crate::Reg<gpioc_drvr::GPIOC_DRVR_SPEC>;
#[doc = "GPIOC_DRVR"]
pub mod gpioc_drvr;
#[doc = "GPIOC_LOCKR (rw) register accessor: an alias for `Reg<GPIOC_LOCKR_SPEC>`"]
pub type GPIOC_LOCKR = crate::Reg<gpioc_lockr::GPIOC_LOCKR_SPEC>;
#[doc = "GPIOC_LOCKR"]
pub mod gpioc_lockr;
#[doc = "GPIOC_DINR (rw) register accessor: an alias for `Reg<GPIOC_DINR_SPEC>`"]
pub type GPIOC_DINR = crate::Reg<gpioc_dinr::GPIOC_DINR_SPEC>;
#[doc = "GPIOC_DINR"]
pub mod gpioc_dinr;
#[doc = "GPIOC_DOUTR (rw) register accessor: an alias for `Reg<GPIOC_DOUTR_SPEC>`"]
pub type GPIOC_DOUTR = crate::Reg<gpioc_doutr::GPIOC_DOUTR_SPEC>;
#[doc = "GPIOC_DOUTR"]
pub mod gpioc_doutr;
#[doc = "GPIOC_SRR (rw) register accessor: an alias for `Reg<GPIOC_SRR_SPEC>`"]
pub type GPIOC_SRR = crate::Reg<gpioc_srr::GPIOC_SRR_SPEC>;
#[doc = "GPIOC_SRR"]
pub mod gpioc_srr;
#[doc = "GPIOC_RR (rw) register accessor: an alias for `Reg<GPIOC_RR_SPEC>`"]
pub type GPIOC_RR = crate::Reg<gpioc_rr::GPIOC_RR_SPEC>;
#[doc = "GPIOC_RR"]
pub mod gpioc_rr;
