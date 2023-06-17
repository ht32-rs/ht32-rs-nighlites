#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PADIRCR"]
    pub padircr: PADIRCR,
    #[doc = "0x04 - PAINER"]
    pub painer: PAINER,
    #[doc = "0x08 - PAPUR"]
    pub papur: PAPUR,
    #[doc = "0x0c - PAPDR"]
    pub papdr: PAPDR,
    #[doc = "0x10 - PAODR"]
    pub paodr: PAODR,
    #[doc = "0x14 - PADRVR"]
    pub padrvr: PADRVR,
    #[doc = "0x18 - PALOCKR"]
    pub palockr: PALOCKR,
    #[doc = "0x1c - PADINR"]
    pub padinr: PADINR,
    #[doc = "0x20 - PADOUTR"]
    pub padoutr: PADOUTR,
    #[doc = "0x24 - PASRR"]
    pub pasrr: PASRR,
    #[doc = "0x28 - PARR"]
    pub parr: PARR,
}
#[doc = "PADIRCR (rw) register accessor: an alias for `Reg<PADIRCR_SPEC>`"]
pub type PADIRCR = crate::Reg<padircr::PADIRCR_SPEC>;
#[doc = "PADIRCR"]
pub mod padircr;
#[doc = "PAINER (rw) register accessor: an alias for `Reg<PAINER_SPEC>`"]
pub type PAINER = crate::Reg<painer::PAINER_SPEC>;
#[doc = "PAINER"]
pub mod painer;
#[doc = "PAPUR (rw) register accessor: an alias for `Reg<PAPUR_SPEC>`"]
pub type PAPUR = crate::Reg<papur::PAPUR_SPEC>;
#[doc = "PAPUR"]
pub mod papur;
#[doc = "PAPDR (rw) register accessor: an alias for `Reg<PAPDR_SPEC>`"]
pub type PAPDR = crate::Reg<papdr::PAPDR_SPEC>;
#[doc = "PAPDR"]
pub mod papdr;
#[doc = "PAODR (rw) register accessor: an alias for `Reg<PAODR_SPEC>`"]
pub type PAODR = crate::Reg<paodr::PAODR_SPEC>;
#[doc = "PAODR"]
pub mod paodr;
#[doc = "PADRVR (rw) register accessor: an alias for `Reg<PADRVR_SPEC>`"]
pub type PADRVR = crate::Reg<padrvr::PADRVR_SPEC>;
#[doc = "PADRVR"]
pub mod padrvr;
#[doc = "PALOCKR (rw) register accessor: an alias for `Reg<PALOCKR_SPEC>`"]
pub type PALOCKR = crate::Reg<palockr::PALOCKR_SPEC>;
#[doc = "PALOCKR"]
pub mod palockr;
#[doc = "PADINR (rw) register accessor: an alias for `Reg<PADINR_SPEC>`"]
pub type PADINR = crate::Reg<padinr::PADINR_SPEC>;
#[doc = "PADINR"]
pub mod padinr;
#[doc = "PADOUTR (rw) register accessor: an alias for `Reg<PADOUTR_SPEC>`"]
pub type PADOUTR = crate::Reg<padoutr::PADOUTR_SPEC>;
#[doc = "PADOUTR"]
pub mod padoutr;
#[doc = "PASRR (rw) register accessor: an alias for `Reg<PASRR_SPEC>`"]
pub type PASRR = crate::Reg<pasrr::PASRR_SPEC>;
#[doc = "PASRR"]
pub mod pasrr;
#[doc = "PARR (rw) register accessor: an alias for `Reg<PARR_SPEC>`"]
pub type PARR = crate::Reg<parr::PARR_SPEC>;
#[doc = "PARR"]
pub mod parr;
