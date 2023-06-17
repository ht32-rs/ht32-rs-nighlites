#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDDIRCR"]
    pub pddircr: PDDIRCR,
    #[doc = "0x04 - PDINER"]
    pub pdiner: PDINER,
    #[doc = "0x08 - PDPUR"]
    pub pdpur: PDPUR,
    #[doc = "0x0c - PDPDR"]
    pub pdpdr: PDPDR,
    #[doc = "0x10 - PDODR"]
    pub pdodr: PDODR,
    #[doc = "0x14 - PDDRVR"]
    pub pddrvr: PDDRVR,
    #[doc = "0x18 - PDLOCKR"]
    pub pdlockr: PDLOCKR,
    #[doc = "0x1c - PDDINR"]
    pub pddinr: PDDINR,
    #[doc = "0x20 - PDDOUTR"]
    pub pddoutr: PDDOUTR,
    #[doc = "0x24 - PDSRR"]
    pub pdsrr: PDSRR,
    #[doc = "0x28 - PDRR"]
    pub pdrr: PDRR,
}
#[doc = "PDDIRCR (rw) register accessor: an alias for `Reg<PDDIRCR_SPEC>`"]
pub type PDDIRCR = crate::Reg<pddircr::PDDIRCR_SPEC>;
#[doc = "PDDIRCR"]
pub mod pddircr;
#[doc = "PDINER (rw) register accessor: an alias for `Reg<PDINER_SPEC>`"]
pub type PDINER = crate::Reg<pdiner::PDINER_SPEC>;
#[doc = "PDINER"]
pub mod pdiner;
#[doc = "PDPUR (rw) register accessor: an alias for `Reg<PDPUR_SPEC>`"]
pub type PDPUR = crate::Reg<pdpur::PDPUR_SPEC>;
#[doc = "PDPUR"]
pub mod pdpur;
#[doc = "PDPDR (rw) register accessor: an alias for `Reg<PDPDR_SPEC>`"]
pub type PDPDR = crate::Reg<pdpdr::PDPDR_SPEC>;
#[doc = "PDPDR"]
pub mod pdpdr;
#[doc = "PDODR (rw) register accessor: an alias for `Reg<PDODR_SPEC>`"]
pub type PDODR = crate::Reg<pdodr::PDODR_SPEC>;
#[doc = "PDODR"]
pub mod pdodr;
#[doc = "PDDRVR (rw) register accessor: an alias for `Reg<PDDRVR_SPEC>`"]
pub type PDDRVR = crate::Reg<pddrvr::PDDRVR_SPEC>;
#[doc = "PDDRVR"]
pub mod pddrvr;
#[doc = "PDLOCKR (rw) register accessor: an alias for `Reg<PDLOCKR_SPEC>`"]
pub type PDLOCKR = crate::Reg<pdlockr::PDLOCKR_SPEC>;
#[doc = "PDLOCKR"]
pub mod pdlockr;
#[doc = "PDDINR (rw) register accessor: an alias for `Reg<PDDINR_SPEC>`"]
pub type PDDINR = crate::Reg<pddinr::PDDINR_SPEC>;
#[doc = "PDDINR"]
pub mod pddinr;
#[doc = "PDDOUTR (rw) register accessor: an alias for `Reg<PDDOUTR_SPEC>`"]
pub type PDDOUTR = crate::Reg<pddoutr::PDDOUTR_SPEC>;
#[doc = "PDDOUTR"]
pub mod pddoutr;
#[doc = "PDSRR (rw) register accessor: an alias for `Reg<PDSRR_SPEC>`"]
pub type PDSRR = crate::Reg<pdsrr::PDSRR_SPEC>;
#[doc = "PDSRR"]
pub mod pdsrr;
#[doc = "PDRR (rw) register accessor: an alias for `Reg<PDRR_SPEC>`"]
pub type PDRR = crate::Reg<pdrr::PDRR_SPEC>;
#[doc = "PDRR"]
pub mod pdrr;
