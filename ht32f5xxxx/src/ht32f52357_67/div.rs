#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CR"]
    pub cr: CR,
    #[doc = "0x04 - DDR"]
    pub ddr: DDR,
    #[doc = "0x08 - DSR"]
    pub dsr: DSR,
    #[doc = "0x0c - QTR"]
    pub qtr: QTR,
    #[doc = "0x10 - RMR"]
    pub rmr: RMR,
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "CR"]
pub mod cr;
#[doc = "DDR (rw) register accessor: an alias for `Reg<DDR_SPEC>`"]
pub type DDR = crate::Reg<ddr::DDR_SPEC>;
#[doc = "DDR"]
pub mod ddr;
#[doc = "DSR (rw) register accessor: an alias for `Reg<DSR_SPEC>`"]
pub type DSR = crate::Reg<dsr::DSR_SPEC>;
#[doc = "DSR"]
pub mod dsr;
#[doc = "QTR (rw) register accessor: an alias for `Reg<QTR_SPEC>`"]
pub type QTR = crate::Reg<qtr::QTR_SPEC>;
#[doc = "QTR"]
pub mod qtr;
#[doc = "RMR (rw) register accessor: an alias for `Reg<RMR_SPEC>`"]
pub type RMR = crate::Reg<rmr::RMR_SPEC>;
#[doc = "RMR"]
pub mod rmr;
