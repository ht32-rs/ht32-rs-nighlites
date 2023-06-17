#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PFDIRCR"]
    pub pfdircr: PFDIRCR,
    #[doc = "0x04 - PFINER"]
    pub pfiner: PFINER,
    #[doc = "0x08 - PFPUR"]
    pub pfpur: PFPUR,
    #[doc = "0x0c - PFPDR"]
    pub pfpdr: PFPDR,
    #[doc = "0x10 - PFODR"]
    pub pfodr: PFODR,
    #[doc = "0x14 - PFDRVR"]
    pub pfdrvr: PFDRVR,
    #[doc = "0x18 - PFLOCKR"]
    pub pflockr: PFLOCKR,
    #[doc = "0x1c - PFDINR"]
    pub pfdinr: PFDINR,
    #[doc = "0x20 - PFDOUTR"]
    pub pfdoutr: PFDOUTR,
    #[doc = "0x24 - PFSRR"]
    pub pfsrr: PFSRR,
    #[doc = "0x28 - PFRR"]
    pub pfrr: PFRR,
}
#[doc = "PFDIRCR (rw) register accessor: an alias for `Reg<PFDIRCR_SPEC>`"]
pub type PFDIRCR = crate::Reg<pfdircr::PFDIRCR_SPEC>;
#[doc = "PFDIRCR"]
pub mod pfdircr;
#[doc = "PFINER (rw) register accessor: an alias for `Reg<PFINER_SPEC>`"]
pub type PFINER = crate::Reg<pfiner::PFINER_SPEC>;
#[doc = "PFINER"]
pub mod pfiner;
#[doc = "PFPUR (rw) register accessor: an alias for `Reg<PFPUR_SPEC>`"]
pub type PFPUR = crate::Reg<pfpur::PFPUR_SPEC>;
#[doc = "PFPUR"]
pub mod pfpur;
#[doc = "PFPDR (rw) register accessor: an alias for `Reg<PFPDR_SPEC>`"]
pub type PFPDR = crate::Reg<pfpdr::PFPDR_SPEC>;
#[doc = "PFPDR"]
pub mod pfpdr;
#[doc = "PFODR (rw) register accessor: an alias for `Reg<PFODR_SPEC>`"]
pub type PFODR = crate::Reg<pfodr::PFODR_SPEC>;
#[doc = "PFODR"]
pub mod pfodr;
#[doc = "PFDRVR (rw) register accessor: an alias for `Reg<PFDRVR_SPEC>`"]
pub type PFDRVR = crate::Reg<pfdrvr::PFDRVR_SPEC>;
#[doc = "PFDRVR"]
pub mod pfdrvr;
#[doc = "PFLOCKR (rw) register accessor: an alias for `Reg<PFLOCKR_SPEC>`"]
pub type PFLOCKR = crate::Reg<pflockr::PFLOCKR_SPEC>;
#[doc = "PFLOCKR"]
pub mod pflockr;
#[doc = "PFDINR (rw) register accessor: an alias for `Reg<PFDINR_SPEC>`"]
pub type PFDINR = crate::Reg<pfdinr::PFDINR_SPEC>;
#[doc = "PFDINR"]
pub mod pfdinr;
#[doc = "PFDOUTR (rw) register accessor: an alias for `Reg<PFDOUTR_SPEC>`"]
pub type PFDOUTR = crate::Reg<pfdoutr::PFDOUTR_SPEC>;
#[doc = "PFDOUTR"]
pub mod pfdoutr;
#[doc = "PFSRR (rw) register accessor: an alias for `Reg<PFSRR_SPEC>`"]
pub type PFSRR = crate::Reg<pfsrr::PFSRR_SPEC>;
#[doc = "PFSRR"]
pub mod pfsrr;
#[doc = "PFRR (rw) register accessor: an alias for `Reg<PFRR_SPEC>`"]
pub type PFRR = crate::Reg<pfrr::PFRR_SPEC>;
#[doc = "PFRR"]
pub mod pfrr;
