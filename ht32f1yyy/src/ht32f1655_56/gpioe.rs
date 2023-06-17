#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOE_DIRCR"]
    pub dircr: DIRCR,
    #[doc = "0x04 - GPIOE_INER"]
    pub iner: INER,
    #[doc = "0x08 - GPIOE_PUR"]
    pub pur: PUR,
    #[doc = "0x0c - GPIOE_PDR"]
    pub pdr: PDR,
    #[doc = "0x10 - GPIOE_ODR"]
    pub odr: ODR,
    #[doc = "0x14 - GPIOE_DRVR"]
    pub drvr: DRVR,
    #[doc = "0x18 - GPIOE_LOCKR"]
    pub lockr: LOCKR,
    #[doc = "0x1c - GPIOE_DINR"]
    pub dinr: DINR,
    #[doc = "0x20 - GPIOE_DOUTR"]
    pub doutr: DOUTR,
    #[doc = "0x24 - GPIOE_SRR"]
    pub srr: SRR,
    #[doc = "0x28 - GPIOE_RR"]
    pub rr: RR,
}
#[doc = "DIRCR (rw) register accessor: an alias for `Reg<DIRCR_SPEC>`"]
pub type DIRCR = crate::Reg<dircr::DIRCR_SPEC>;
#[doc = "GPIOE_DIRCR"]
pub mod dircr;
#[doc = "INER (rw) register accessor: an alias for `Reg<INER_SPEC>`"]
pub type INER = crate::Reg<iner::INER_SPEC>;
#[doc = "GPIOE_INER"]
pub mod iner;
#[doc = "PUR (rw) register accessor: an alias for `Reg<PUR_SPEC>`"]
pub type PUR = crate::Reg<pur::PUR_SPEC>;
#[doc = "GPIOE_PUR"]
pub mod pur;
#[doc = "PDR (rw) register accessor: an alias for `Reg<PDR_SPEC>`"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "GPIOE_PDR"]
pub mod pdr;
#[doc = "ODR (rw) register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "GPIOE_ODR"]
pub mod odr;
#[doc = "DRVR (rw) register accessor: an alias for `Reg<DRVR_SPEC>`"]
pub type DRVR = crate::Reg<drvr::DRVR_SPEC>;
#[doc = "GPIOE_DRVR"]
pub mod drvr;
#[doc = "LOCKR (rw) register accessor: an alias for `Reg<LOCKR_SPEC>`"]
pub type LOCKR = crate::Reg<lockr::LOCKR_SPEC>;
#[doc = "GPIOE_LOCKR"]
pub mod lockr;
#[doc = "DINR (rw) register accessor: an alias for `Reg<DINR_SPEC>`"]
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
#[doc = "GPIOE_DINR"]
pub mod dinr;
#[doc = "DOUTR (rw) register accessor: an alias for `Reg<DOUTR_SPEC>`"]
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
#[doc = "GPIOE_DOUTR"]
pub mod doutr;
#[doc = "SRR (rw) register accessor: an alias for `Reg<SRR_SPEC>`"]
pub type SRR = crate::Reg<srr::SRR_SPEC>;
#[doc = "GPIOE_SRR"]
pub mod srr;
#[doc = "RR (rw) register accessor: an alias for `Reg<RR_SPEC>`"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "GPIOE_RR"]
pub mod rr;
