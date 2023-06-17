#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOE_DIRCR"]
    pub gpioe_dircr: GPIOE_DIRCR,
    #[doc = "0x04 - GPIOE_INER"]
    pub gpioe_iner: GPIOE_INER,
    #[doc = "0x08 - GPIOE_PUR"]
    pub gpioe_pur: GPIOE_PUR,
    #[doc = "0x0c - GPIOE_PDR"]
    pub gpioe_pdr: GPIOE_PDR,
    #[doc = "0x10 - GPIOE_ODR"]
    pub gpioe_odr: GPIOE_ODR,
    #[doc = "0x14 - GPIOE_DRVR"]
    pub gpioe_drvr: GPIOE_DRVR,
    #[doc = "0x18 - GPIOE_LOCKR"]
    pub gpioe_lockr: GPIOE_LOCKR,
    #[doc = "0x1c - GPIOE_DINR"]
    pub gpioe_dinr: GPIOE_DINR,
    #[doc = "0x20 - GPIOE_DOUTR"]
    pub gpioe_doutr: GPIOE_DOUTR,
    #[doc = "0x24 - GPIOE_SRR"]
    pub gpioe_srr: GPIOE_SRR,
    #[doc = "0x28 - GPIOE_RR"]
    pub gpioe_rr: GPIOE_RR,
}
#[doc = "GPIOE_DIRCR (rw) register accessor: an alias for `Reg<GPIOE_DIRCR_SPEC>`"]
pub type GPIOE_DIRCR = crate::Reg<gpioe_dircr::GPIOE_DIRCR_SPEC>;
#[doc = "GPIOE_DIRCR"]
pub mod gpioe_dircr;
#[doc = "GPIOE_INER (rw) register accessor: an alias for `Reg<GPIOE_INER_SPEC>`"]
pub type GPIOE_INER = crate::Reg<gpioe_iner::GPIOE_INER_SPEC>;
#[doc = "GPIOE_INER"]
pub mod gpioe_iner;
#[doc = "GPIOE_PUR (rw) register accessor: an alias for `Reg<GPIOE_PUR_SPEC>`"]
pub type GPIOE_PUR = crate::Reg<gpioe_pur::GPIOE_PUR_SPEC>;
#[doc = "GPIOE_PUR"]
pub mod gpioe_pur;
#[doc = "GPIOE_PDR (rw) register accessor: an alias for `Reg<GPIOE_PDR_SPEC>`"]
pub type GPIOE_PDR = crate::Reg<gpioe_pdr::GPIOE_PDR_SPEC>;
#[doc = "GPIOE_PDR"]
pub mod gpioe_pdr;
#[doc = "GPIOE_ODR (rw) register accessor: an alias for `Reg<GPIOE_ODR_SPEC>`"]
pub type GPIOE_ODR = crate::Reg<gpioe_odr::GPIOE_ODR_SPEC>;
#[doc = "GPIOE_ODR"]
pub mod gpioe_odr;
#[doc = "GPIOE_DRVR (rw) register accessor: an alias for `Reg<GPIOE_DRVR_SPEC>`"]
pub type GPIOE_DRVR = crate::Reg<gpioe_drvr::GPIOE_DRVR_SPEC>;
#[doc = "GPIOE_DRVR"]
pub mod gpioe_drvr;
#[doc = "GPIOE_LOCKR (rw) register accessor: an alias for `Reg<GPIOE_LOCKR_SPEC>`"]
pub type GPIOE_LOCKR = crate::Reg<gpioe_lockr::GPIOE_LOCKR_SPEC>;
#[doc = "GPIOE_LOCKR"]
pub mod gpioe_lockr;
#[doc = "GPIOE_DINR (rw) register accessor: an alias for `Reg<GPIOE_DINR_SPEC>`"]
pub type GPIOE_DINR = crate::Reg<gpioe_dinr::GPIOE_DINR_SPEC>;
#[doc = "GPIOE_DINR"]
pub mod gpioe_dinr;
#[doc = "GPIOE_DOUTR (rw) register accessor: an alias for `Reg<GPIOE_DOUTR_SPEC>`"]
pub type GPIOE_DOUTR = crate::Reg<gpioe_doutr::GPIOE_DOUTR_SPEC>;
#[doc = "GPIOE_DOUTR"]
pub mod gpioe_doutr;
#[doc = "GPIOE_SRR (rw) register accessor: an alias for `Reg<GPIOE_SRR_SPEC>`"]
pub type GPIOE_SRR = crate::Reg<gpioe_srr::GPIOE_SRR_SPEC>;
#[doc = "GPIOE_SRR"]
pub mod gpioe_srr;
#[doc = "GPIOE_RR (rw) register accessor: an alias for `Reg<GPIOE_RR_SPEC>`"]
pub type GPIOE_RR = crate::Reg<gpioe_rr::GPIOE_RR_SPEC>;
#[doc = "GPIOE_RR"]
pub mod gpioe_rr;
