#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOA_DIRCR"]
    pub gpioa_dircr: GPIOA_DIRCR,
    #[doc = "0x04 - GPIOA_INER"]
    pub gpioa_iner: GPIOA_INER,
    #[doc = "0x08 - GPIOA_PUR"]
    pub gpioa_pur: GPIOA_PUR,
    #[doc = "0x0c - GPIOA_PDR"]
    pub gpioa_pdr: GPIOA_PDR,
    #[doc = "0x10 - GPIOA_ODR"]
    pub gpioa_odr: GPIOA_ODR,
    #[doc = "0x14 - GPIOA_DRVR"]
    pub gpioa_drvr: GPIOA_DRVR,
    #[doc = "0x18 - GPIOA_LOCKR"]
    pub gpioa_lockr: GPIOA_LOCKR,
    #[doc = "0x1c - GPIOA_DINR"]
    pub gpioa_dinr: GPIOA_DINR,
    #[doc = "0x20 - GPIOA_DOUTR"]
    pub gpioa_doutr: GPIOA_DOUTR,
    #[doc = "0x24 - GPIOA_SRR"]
    pub gpioa_srr: GPIOA_SRR,
    #[doc = "0x28 - GPIOA_RR"]
    pub gpioa_rr: GPIOA_RR,
}
#[doc = "GPIOA_DIRCR (rw) register accessor: an alias for `Reg<GPIOA_DIRCR_SPEC>`"]
pub type GPIOA_DIRCR = crate::Reg<gpioa_dircr::GPIOA_DIRCR_SPEC>;
#[doc = "GPIOA_DIRCR"]
pub mod gpioa_dircr;
#[doc = "GPIOA_INER (rw) register accessor: an alias for `Reg<GPIOA_INER_SPEC>`"]
pub type GPIOA_INER = crate::Reg<gpioa_iner::GPIOA_INER_SPEC>;
#[doc = "GPIOA_INER"]
pub mod gpioa_iner;
#[doc = "GPIOA_PUR (rw) register accessor: an alias for `Reg<GPIOA_PUR_SPEC>`"]
pub type GPIOA_PUR = crate::Reg<gpioa_pur::GPIOA_PUR_SPEC>;
#[doc = "GPIOA_PUR"]
pub mod gpioa_pur;
#[doc = "GPIOA_PDR (rw) register accessor: an alias for `Reg<GPIOA_PDR_SPEC>`"]
pub type GPIOA_PDR = crate::Reg<gpioa_pdr::GPIOA_PDR_SPEC>;
#[doc = "GPIOA_PDR"]
pub mod gpioa_pdr;
#[doc = "GPIOA_ODR (rw) register accessor: an alias for `Reg<GPIOA_ODR_SPEC>`"]
pub type GPIOA_ODR = crate::Reg<gpioa_odr::GPIOA_ODR_SPEC>;
#[doc = "GPIOA_ODR"]
pub mod gpioa_odr;
#[doc = "GPIOA_DRVR (rw) register accessor: an alias for `Reg<GPIOA_DRVR_SPEC>`"]
pub type GPIOA_DRVR = crate::Reg<gpioa_drvr::GPIOA_DRVR_SPEC>;
#[doc = "GPIOA_DRVR"]
pub mod gpioa_drvr;
#[doc = "GPIOA_LOCKR (rw) register accessor: an alias for `Reg<GPIOA_LOCKR_SPEC>`"]
pub type GPIOA_LOCKR = crate::Reg<gpioa_lockr::GPIOA_LOCKR_SPEC>;
#[doc = "GPIOA_LOCKR"]
pub mod gpioa_lockr;
#[doc = "GPIOA_DINR (rw) register accessor: an alias for `Reg<GPIOA_DINR_SPEC>`"]
pub type GPIOA_DINR = crate::Reg<gpioa_dinr::GPIOA_DINR_SPEC>;
#[doc = "GPIOA_DINR"]
pub mod gpioa_dinr;
#[doc = "GPIOA_DOUTR (rw) register accessor: an alias for `Reg<GPIOA_DOUTR_SPEC>`"]
pub type GPIOA_DOUTR = crate::Reg<gpioa_doutr::GPIOA_DOUTR_SPEC>;
#[doc = "GPIOA_DOUTR"]
pub mod gpioa_doutr;
#[doc = "GPIOA_SRR (rw) register accessor: an alias for `Reg<GPIOA_SRR_SPEC>`"]
pub type GPIOA_SRR = crate::Reg<gpioa_srr::GPIOA_SRR_SPEC>;
#[doc = "GPIOA_SRR"]
pub mod gpioa_srr;
#[doc = "GPIOA_RR (rw) register accessor: an alias for `Reg<GPIOA_RR_SPEC>`"]
pub type GPIOA_RR = crate::Reg<gpioa_rr::GPIOA_RR_SPEC>;
#[doc = "GPIOA_RR"]
pub mod gpioa_rr;
