#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOD_DIRCR"]
    pub dircr: DIRCR,
    #[doc = "0x04 - GPIOD_INER"]
    pub iner: INER,
    #[doc = "0x08 - GPIOD_PUR"]
    pub pur: PUR,
    #[doc = "0x0c - GPIOD_PDR"]
    pub pdr: PDR,
    #[doc = "0x10 - GPIOD_ODR"]
    pub odr: ODR,
    #[doc = "0x14 - GPIOD_DRVR"]
    pub drvr: DRVR,
    #[doc = "0x18 - GPIOD_LOCKR"]
    pub lockr: LOCKR,
    #[doc = "0x1c - GPIOD_DINR"]
    pub dinr: DINR,
    #[doc = "0x20 - GPIOD_DOUTR"]
    pub doutr: DOUTR,
    #[doc = "0x24 - GPIOD_SRR"]
    pub srr: SRR,
    #[doc = "0x28 - GPIOD_RR"]
    pub rr: RR,
}
#[doc = "DIRCR (rw) register accessor: an alias for `Reg<DIRCR_SPEC>`"]
pub type DIRCR = crate::Reg<dircr::DIRCR_SPEC>;
#[doc = "GPIOD_DIRCR"]
pub mod dircr;
#[doc = "INER (rw) register accessor: an alias for `Reg<INER_SPEC>`"]
pub type INER = crate::Reg<iner::INER_SPEC>;
#[doc = "GPIOD_INER"]
pub mod iner;
#[doc = "PUR (rw) register accessor: an alias for `Reg<PUR_SPEC>`"]
pub type PUR = crate::Reg<pur::PUR_SPEC>;
#[doc = "GPIOD_PUR"]
pub mod pur;
#[doc = "PDR (rw) register accessor: an alias for `Reg<PDR_SPEC>`"]
pub type PDR = crate::Reg<pdr::PDR_SPEC>;
#[doc = "GPIOD_PDR"]
pub mod pdr;
#[doc = "ODR (rw) register accessor: an alias for `Reg<ODR_SPEC>`"]
pub type ODR = crate::Reg<odr::ODR_SPEC>;
#[doc = "GPIOD_ODR"]
pub mod odr;
#[doc = "DRVR (rw) register accessor: an alias for `Reg<DRVR_SPEC>`"]
pub type DRVR = crate::Reg<drvr::DRVR_SPEC>;
#[doc = "GPIOD_DRVR"]
pub mod drvr;
#[doc = "LOCKR (rw) register accessor: an alias for `Reg<LOCKR_SPEC>`"]
pub type LOCKR = crate::Reg<lockr::LOCKR_SPEC>;
#[doc = "GPIOD_LOCKR"]
pub mod lockr;
#[doc = "DINR (rw) register accessor: an alias for `Reg<DINR_SPEC>`"]
pub type DINR = crate::Reg<dinr::DINR_SPEC>;
#[doc = "GPIOD_DINR"]
pub mod dinr;
#[doc = "DOUTR (rw) register accessor: an alias for `Reg<DOUTR_SPEC>`"]
pub type DOUTR = crate::Reg<doutr::DOUTR_SPEC>;
#[doc = "GPIOD_DOUTR"]
pub mod doutr;
#[doc = "SRR (rw) register accessor: an alias for `Reg<SRR_SPEC>`"]
pub type SRR = crate::Reg<srr::SRR_SPEC>;
#[doc = "GPIOD_SRR"]
pub mod srr;
#[doc = "RR (rw) register accessor: an alias for `Reg<RR_SPEC>`"]
pub type RR = crate::Reg<rr::RR_SPEC>;
#[doc = "GPIOD_RR"]
pub mod rr;
