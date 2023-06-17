#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PCDIRCR"]
    pub pcdircr: PCDIRCR,
    #[doc = "0x04 - PCINER"]
    pub pciner: PCINER,
    #[doc = "0x08 - PCPUR"]
    pub pcpur: PCPUR,
    #[doc = "0x0c - PCPDR"]
    pub pcpdr: PCPDR,
    #[doc = "0x10 - PCODR"]
    pub pcodr: PCODR,
    #[doc = "0x14 - PCDRVR"]
    pub pcdrvr: PCDRVR,
    #[doc = "0x18 - PCLOCKR"]
    pub pclockr: PCLOCKR,
    #[doc = "0x1c - PCDINR"]
    pub pcdinr: PCDINR,
    #[doc = "0x20 - PCDOUTR"]
    pub pcdoutr: PCDOUTR,
    #[doc = "0x24 - PCSRR"]
    pub pcsrr: PCSRR,
    #[doc = "0x28 - PCRR"]
    pub pcrr: PCRR,
    #[doc = "0x2c - PCSCER"]
    pub pcscer: PCSCER,
}
#[doc = "PCDIRCR (rw) register accessor: an alias for `Reg<PCDIRCR_SPEC>`"]
pub type PCDIRCR = crate::Reg<pcdircr::PCDIRCR_SPEC>;
#[doc = "PCDIRCR"]
pub mod pcdircr;
#[doc = "PCINER (rw) register accessor: an alias for `Reg<PCINER_SPEC>`"]
pub type PCINER = crate::Reg<pciner::PCINER_SPEC>;
#[doc = "PCINER"]
pub mod pciner;
#[doc = "PCPUR (rw) register accessor: an alias for `Reg<PCPUR_SPEC>`"]
pub type PCPUR = crate::Reg<pcpur::PCPUR_SPEC>;
#[doc = "PCPUR"]
pub mod pcpur;
#[doc = "PCPDR (rw) register accessor: an alias for `Reg<PCPDR_SPEC>`"]
pub type PCPDR = crate::Reg<pcpdr::PCPDR_SPEC>;
#[doc = "PCPDR"]
pub mod pcpdr;
#[doc = "PCODR (rw) register accessor: an alias for `Reg<PCODR_SPEC>`"]
pub type PCODR = crate::Reg<pcodr::PCODR_SPEC>;
#[doc = "PCODR"]
pub mod pcodr;
#[doc = "PCDRVR (rw) register accessor: an alias for `Reg<PCDRVR_SPEC>`"]
pub type PCDRVR = crate::Reg<pcdrvr::PCDRVR_SPEC>;
#[doc = "PCDRVR"]
pub mod pcdrvr;
#[doc = "PCLOCKR (rw) register accessor: an alias for `Reg<PCLOCKR_SPEC>`"]
pub type PCLOCKR = crate::Reg<pclockr::PCLOCKR_SPEC>;
#[doc = "PCLOCKR"]
pub mod pclockr;
#[doc = "PCDINR (rw) register accessor: an alias for `Reg<PCDINR_SPEC>`"]
pub type PCDINR = crate::Reg<pcdinr::PCDINR_SPEC>;
#[doc = "PCDINR"]
pub mod pcdinr;
#[doc = "PCDOUTR (rw) register accessor: an alias for `Reg<PCDOUTR_SPEC>`"]
pub type PCDOUTR = crate::Reg<pcdoutr::PCDOUTR_SPEC>;
#[doc = "PCDOUTR"]
pub mod pcdoutr;
#[doc = "PCSRR (rw) register accessor: an alias for `Reg<PCSRR_SPEC>`"]
pub type PCSRR = crate::Reg<pcsrr::PCSRR_SPEC>;
#[doc = "PCSRR"]
pub mod pcsrr;
#[doc = "PCRR (rw) register accessor: an alias for `Reg<PCRR_SPEC>`"]
pub type PCRR = crate::Reg<pcrr::PCRR_SPEC>;
#[doc = "PCRR"]
pub mod pcrr;
#[doc = "PCSCER (rw) register accessor: an alias for `Reg<PCSCER_SPEC>`"]
pub type PCSCER = crate::Reg<pcscer::PCSCER_SPEC>;
#[doc = "PCSCER"]
pub mod pcscer;
