#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOC_DIRCR"]
    pub dircr: DIRCR,
    #[doc = "0x04 - GPIOC_INER"]
    pub iner: INER,
    #[doc = "0x08 - GPIOC_PUR"]
    pub pur: PUR,
    #[doc = "0x0c - GPIOC_PDR"]
    pub pdr: PDR,
    #[doc = "0x10 - GPIOC_ODR"]
    pub odr: ODR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - GPIOC_LOCKR"]
    pub lockr: LOCKR,
    #[doc = "0x1c - GPIOC_DINR"]
    pub dinr: DINR,
    #[doc = "0x20 - GPIOC_DOUTR"]
    pub doutr: DOUTR,
    #[doc = "0x24 - GPIOC_SRR"]
    pub srr: SRR,
    #[doc = "0x28 - GPIOC_RR"]
    pub rr: RR,
}
#[doc = "DIRCR (rw) register accessor: an alias for `Reg<DIRCR_SPEC>`"]
pub type DIRCR = crate::Reg<dircr::DIRCR_SPEC>;
#[doc = "GPIOC_DIRCR"]
pub mod dircr;
#[doc = "INER (rw) register accessor: an alias for `Reg<INER_SPEC>`"]
pub type INER = crate::Reg<iner::INER_SPEC>;
#[doc = "GPIOC_INER"]
pub mod iner;
#[doc = "PUR (rw) register accessor: an alias for `Reg<PUR_SPEC>`"]
pub type PUR = crate::Reg<pur::PUR_SPEC>;
#[doc = "GPIOC_PUR"]
pub mod pur;
#[doc = "PDR (rw) register accessor: an alias for `Reg<PDR_SPEC>`"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "GPIOC_PDR"]
pub mod pdr;
#[doc = "ODR (rw) register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "GPIOC_ODR"]
pub mod odr;
#[doc = "LOCKR (rw) register accessor: an alias for `Reg<LOCKR_SPEC>`"]
pub type LOCKR = crate::Reg<lockr::LOCKR_SPEC>;
#[doc = "GPIOC_LOCKR"]
pub mod lockr;
#[doc = "DINR (rw) register accessor: an alias for `Reg<DINR_SPEC>`"]
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
#[doc = "GPIOC_DINR"]
pub mod dinr;
#[doc = "DOUTR (rw) register accessor: an alias for `Reg<DOUTR_SPEC>`"]
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
#[doc = "GPIOC_DOUTR"]
pub mod doutr;
#[doc = "SRR (rw) register accessor: an alias for `Reg<SRR_SPEC>`"]
pub type SRR = crate::Reg<srr::SRR_SPEC>;
#[doc = "GPIOC_SRR"]
pub mod srr;
#[doc = "RR (rw) register accessor: an alias for `Reg<RR_SPEC>`"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "GPIOC_RR"]
pub mod rr;
