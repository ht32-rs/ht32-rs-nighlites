#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DIV_CR"]
    pub div_cr: DIV_CR,
    #[doc = "0x04 - DIV_DDR"]
    pub div_ddr: DIV_DDR,
    #[doc = "0x08 - DIV_DSR"]
    pub div_dsr: DIV_DSR,
    #[doc = "0x0c - DIV_QTR"]
    pub div_qtr: DIV_QTR,
    #[doc = "0x10 - DIV_RMR"]
    pub div_rmr: DIV_RMR,
}
#[doc = "DIV_CR (rw) register accessor: an alias for `Reg<DIV_CR_SPEC>`"]
pub type DIV_CR = crate::Reg<div_cr::DIV_CR_SPEC>;
#[doc = "DIV_CR"]
pub mod div_cr;
#[doc = "DIV_DDR (rw) register accessor: an alias for `Reg<DIV_DDR_SPEC>`"]
pub type DIV_DDR = crate::Reg<div_ddr::DIV_DDR_SPEC>;
#[doc = "DIV_DDR"]
pub mod div_ddr;
#[doc = "DIV_DSR (rw) register accessor: an alias for `Reg<DIV_DSR_SPEC>`"]
pub type DIV_DSR = crate::Reg<div_dsr::DIV_DSR_SPEC>;
#[doc = "DIV_DSR"]
pub mod div_dsr;
#[doc = "DIV_QTR (rw) register accessor: an alias for `Reg<DIV_QTR_SPEC>`"]
pub type DIV_QTR = crate::Reg<div_qtr::DIV_QTR_SPEC>;
#[doc = "DIV_QTR"]
pub mod div_qtr;
#[doc = "DIV_RMR (rw) register accessor: an alias for `Reg<DIV_RMR_SPEC>`"]
pub type DIV_RMR = crate::Reg<div_rmr::DIV_RMR_SPEC>;
#[doc = "DIV_RMR"]
pub mod div_rmr;
