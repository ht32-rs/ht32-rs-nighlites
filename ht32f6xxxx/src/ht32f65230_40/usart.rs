#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USRDR"]
    pub usrdr: USRDR,
    #[doc = "0x04 - USRCR"]
    pub usrcr: USRCR,
    #[doc = "0x08 - USRFCR"]
    pub usrfcr: USRFCR,
    #[doc = "0x0c - USRIER"]
    pub usrier: USRIER,
    #[doc = "0x10 - USRSIFR"]
    pub usrsifr: USRSIFR,
    #[doc = "0x14 - USRTPR"]
    pub usrtpr: USRTPR,
    #[doc = "0x18 - IrDACR"]
    pub ir_dacr: IR_DACR,
    #[doc = "0x1c - RS485CR"]
    pub rs485cr: RS485CR,
    #[doc = "0x20 - SYNCR"]
    pub syncr: SYNCR,
    #[doc = "0x24 - USRDLR"]
    pub usrdlr: USRDLR,
    #[doc = "0x28 - USRTSTR"]
    pub usrtstr: USRTSTR,
}
#[doc = "USRDR (rw) register accessor: an alias for `Reg<USRDR_SPEC>`"]
pub type USRDR = crate::Reg<usrdr::USRDR_SPEC>;
#[doc = "USRDR"]
pub mod usrdr;
#[doc = "USRCR (rw) register accessor: an alias for `Reg<USRCR_SPEC>`"]
pub type USRCR = crate::Reg<usrcr::USRCR_SPEC>;
#[doc = "USRCR"]
pub mod usrcr;
#[doc = "USRFCR (rw) register accessor: an alias for `Reg<USRFCR_SPEC>`"]
pub type USRFCR = crate::Reg<usrfcr::USRFCR_SPEC>;
#[doc = "USRFCR"]
pub mod usrfcr;
#[doc = "USRIER (rw) register accessor: an alias for `Reg<USRIER_SPEC>`"]
pub type USRIER = crate::Reg<usrier::USRIER_SPEC>;
#[doc = "USRIER"]
pub mod usrier;
#[doc = "USRSIFR (rw) register accessor: an alias for `Reg<USRSIFR_SPEC>`"]
pub type USRSIFR = crate::Reg<usrsifr::USRSIFR_SPEC>;
#[doc = "USRSIFR"]
pub mod usrsifr;
#[doc = "USRTPR (rw) register accessor: an alias for `Reg<USRTPR_SPEC>`"]
pub type USRTPR = crate::Reg<usrtpr::USRTPR_SPEC>;
#[doc = "USRTPR"]
pub mod usrtpr;
#[doc = "IrDACR (rw) register accessor: an alias for `Reg<IR_DACR_SPEC>`"]
pub type IR_DACR = crate::Reg<ir_dacr::IR_DACR_SPEC>;
#[doc = "IrDACR"]
pub mod ir_dacr;
#[doc = "RS485CR (rw) register accessor: an alias for `Reg<RS485CR_SPEC>`"]
pub type RS485CR = crate::Reg<rs485cr::RS485CR_SPEC>;
#[doc = "RS485CR"]
pub mod rs485cr;
#[doc = "SYNCR (rw) register accessor: an alias for `Reg<SYNCR_SPEC>`"]
pub type SYNCR = crate::Reg<syncr::SYNCR_SPEC>;
#[doc = "SYNCR"]
pub mod syncr;
#[doc = "USRDLR (rw) register accessor: an alias for `Reg<USRDLR_SPEC>`"]
pub type USRDLR = crate::Reg<usrdlr::USRDLR_SPEC>;
#[doc = "USRDLR"]
pub mod usrdlr;
#[doc = "USRTSTR (rw) register accessor: an alias for `Reg<USRTSTR_SPEC>`"]
pub type USRTSTR = crate::Reg<usrtstr::USRTSTR_SPEC>;
#[doc = "USRTSTR"]
pub mod usrtstr;
