#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PEDIRCR"]
    pub pedircr: PEDIRCR,
    #[doc = "0x04 - PEINER"]
    pub peiner: PEINER,
    #[doc = "0x08 - PEPUR"]
    pub pepur: PEPUR,
    #[doc = "0x0c - PEPDR"]
    pub pepdr: PEPDR,
    #[doc = "0x10 - PEODR"]
    pub peodr: PEODR,
    #[doc = "0x14 - PEDRVR"]
    pub pedrvr: PEDRVR,
    #[doc = "0x18 - PELOCKR"]
    pub pelockr: PELOCKR,
    #[doc = "0x1c - PEDINR"]
    pub pedinr: PEDINR,
    #[doc = "0x20 - PEDOUTR"]
    pub pedoutr: PEDOUTR,
    #[doc = "0x24 - PESRR"]
    pub pesrr: PESRR,
    #[doc = "0x28 - PERR"]
    pub perr: PERR,
}
#[doc = "PEDIRCR (rw) register accessor: an alias for `Reg<PEDIRCR_SPEC>`"]
pub type PEDIRCR = crate::Reg<pedircr::PEDIRCR_SPEC>;
#[doc = "PEDIRCR"]
pub mod pedircr;
#[doc = "PEINER (rw) register accessor: an alias for `Reg<PEINER_SPEC>`"]
pub type PEINER = crate::Reg<peiner::PEINER_SPEC>;
#[doc = "PEINER"]
pub mod peiner;
#[doc = "PEPUR (rw) register accessor: an alias for `Reg<PEPUR_SPEC>`"]
pub type PEPUR = crate::Reg<pepur::PEPUR_SPEC>;
#[doc = "PEPUR"]
pub mod pepur;
#[doc = "PEPDR (rw) register accessor: an alias for `Reg<PEPDR_SPEC>`"]
pub type PEPDR = crate::Reg<pepdr::PEPDR_SPEC>;
#[doc = "PEPDR"]
pub mod pepdr;
#[doc = "PEODR (rw) register accessor: an alias for `Reg<PEODR_SPEC>`"]
pub type PEODR = crate::Reg<peodr::PEODR_SPEC>;
#[doc = "PEODR"]
pub mod peodr;
#[doc = "PEDRVR (rw) register accessor: an alias for `Reg<PEDRVR_SPEC>`"]
pub type PEDRVR = crate::Reg<pedrvr::PEDRVR_SPEC>;
#[doc = "PEDRVR"]
pub mod pedrvr;
#[doc = "PELOCKR (rw) register accessor: an alias for `Reg<PELOCKR_SPEC>`"]
pub type PELOCKR = crate::Reg<pelockr::PELOCKR_SPEC>;
#[doc = "PELOCKR"]
pub mod pelockr;
#[doc = "PEDINR (rw) register accessor: an alias for `Reg<PEDINR_SPEC>`"]
pub type PEDINR = crate::Reg<pedinr::PEDINR_SPEC>;
#[doc = "PEDINR"]
pub mod pedinr;
#[doc = "PEDOUTR (rw) register accessor: an alias for `Reg<PEDOUTR_SPEC>`"]
pub type PEDOUTR = crate::Reg<pedoutr::PEDOUTR_SPEC>;
#[doc = "PEDOUTR"]
pub mod pedoutr;
#[doc = "PESRR (rw) register accessor: an alias for `Reg<PESRR_SPEC>`"]
pub type PESRR = crate::Reg<pesrr::PESRR_SPEC>;
#[doc = "PESRR"]
pub mod pesrr;
#[doc = "PERR (rw) register accessor: an alias for `Reg<PERR_SPEC>`"]
pub type PERR = crate::Reg<perr::PERR_SPEC>;
#[doc = "PERR"]
pub mod perr;
