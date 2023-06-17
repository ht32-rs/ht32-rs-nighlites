#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PBDIRCR"]
    pub pbdircr: PBDIRCR,
    #[doc = "0x04 - PBINER"]
    pub pbiner: PBINER,
    #[doc = "0x08 - PBPUR"]
    pub pbpur: PBPUR,
    #[doc = "0x0c - PBPDR"]
    pub pbpdr: PBPDR,
    #[doc = "0x10 - PBODR"]
    pub pbodr: PBODR,
    #[doc = "0x14 - PBDRVR"]
    pub pbdrvr: PBDRVR,
    #[doc = "0x18 - PBLOCKR"]
    pub pblockr: PBLOCKR,
    #[doc = "0x1c - PBDINR"]
    pub pbdinr: PBDINR,
    #[doc = "0x20 - PBDOUTR"]
    pub pbdoutr: PBDOUTR,
    #[doc = "0x24 - PBSRR"]
    pub pbsrr: PBSRR,
    #[doc = "0x28 - PBRR"]
    pub pbrr: PBRR,
    #[doc = "0x2c - PBSCER"]
    pub pbscer: PBSCER,
}
#[doc = "PBDIRCR (rw) register accessor: an alias for `Reg<PBDIRCR_SPEC>`"]
pub type PBDIRCR = crate::Reg<pbdircr::PBDIRCR_SPEC>;
#[doc = "PBDIRCR"]
pub mod pbdircr;
#[doc = "PBINER (rw) register accessor: an alias for `Reg<PBINER_SPEC>`"]
pub type PBINER = crate::Reg<pbiner::PBINER_SPEC>;
#[doc = "PBINER"]
pub mod pbiner;
#[doc = "PBPUR (rw) register accessor: an alias for `Reg<PBPUR_SPEC>`"]
pub type PBPUR = crate::Reg<pbpur::PBPUR_SPEC>;
#[doc = "PBPUR"]
pub mod pbpur;
#[doc = "PBPDR (rw) register accessor: an alias for `Reg<PBPDR_SPEC>`"]
pub type PBPDR = crate::Reg<pbpdr::PBPDR_SPEC>;
#[doc = "PBPDR"]
pub mod pbpdr;
#[doc = "PBODR (rw) register accessor: an alias for `Reg<PBODR_SPEC>`"]
pub type PBODR = crate::Reg<pbodr::PBODR_SPEC>;
#[doc = "PBODR"]
pub mod pbodr;
#[doc = "PBDRVR (rw) register accessor: an alias for `Reg<PBDRVR_SPEC>`"]
pub type PBDRVR = crate::Reg<pbdrvr::PBDRVR_SPEC>;
#[doc = "PBDRVR"]
pub mod pbdrvr;
#[doc = "PBLOCKR (rw) register accessor: an alias for `Reg<PBLOCKR_SPEC>`"]
pub type PBLOCKR = crate::Reg<pblockr::PBLOCKR_SPEC>;
#[doc = "PBLOCKR"]
pub mod pblockr;
#[doc = "PBDINR (rw) register accessor: an alias for `Reg<PBDINR_SPEC>`"]
pub type PBDINR = crate::Reg<pbdinr::PBDINR_SPEC>;
#[doc = "PBDINR"]
pub mod pbdinr;
#[doc = "PBDOUTR (rw) register accessor: an alias for `Reg<PBDOUTR_SPEC>`"]
pub type PBDOUTR = crate::Reg<pbdoutr::PBDOUTR_SPEC>;
#[doc = "PBDOUTR"]
pub mod pbdoutr;
#[doc = "PBSRR (rw) register accessor: an alias for `Reg<PBSRR_SPEC>`"]
pub type PBSRR = crate::Reg<pbsrr::PBSRR_SPEC>;
#[doc = "PBSRR"]
pub mod pbsrr;
#[doc = "PBRR (rw) register accessor: an alias for `Reg<PBRR_SPEC>`"]
pub type PBRR = crate::Reg<pbrr::PBRR_SPEC>;
#[doc = "PBRR"]
pub mod pbrr;
#[doc = "PBSCER (rw) register accessor: an alias for `Reg<PBSCER_SPEC>`"]
pub type PBSCER = crate::Reg<pbscer::PBSCER_SPEC>;
#[doc = "PBSCER"]
pub mod pbscer;
