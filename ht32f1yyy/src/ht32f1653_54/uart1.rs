#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uart1: [u8; 0x04],
    #[doc = "0x04 - UART1_IER"]
    pub uart1_ier: UART1_IER,
    #[doc = "0x08 - UART1_IIR"]
    pub uart1_iir: UART1_IIR,
    #[doc = "0x0c - UART1_FCR"]
    pub uart1_fcr: UART1_FCR,
    #[doc = "0x10 - UART1_LCR"]
    pub uart1_lcr: UART1_LCR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - UART1_LSR"]
    pub uart1_lsr: UART1_LSR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - UART1_TPR"]
    pub uart1_tpr: UART1_TPR,
    #[doc = "0x24 - UART1_MDR"]
    pub uart1_mdr: UART1_MDR,
    _reserved8: [u8; 0x0c],
    #[doc = "0x34 - UART1_FSR"]
    pub uart1_fsr: UART1_FSR,
    #[doc = "0x38 - UART1_DLR"]
    pub uart1_dlr: UART1_DLR,
    _reserved10: [u8; 0x04],
    #[doc = "0x40 - UART1_DEGTSTR"]
    pub uart1_degtstr: UART1_DEGTSTR,
}
impl RegisterBlock {
    #[doc = "0x00 - UART1_TBR"]
    #[inline(always)]
    pub const fn uart1_uart1_tbr(&self) -> &UART1_UART1_TBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - UART1_RBR"]
    #[inline(always)]
    pub const fn uart1_rbr(&self) -> &UART1_RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "UART1_RBR (rw) register accessor: an alias for `Reg<UART1_RBR_SPEC>`"]
pub type UART1_RBR = crate::Reg<uart1_rbr::UART1_RBR_SPEC>;
#[doc = "UART1_RBR"]
pub mod uart1_rbr;
#[doc = "UART1_UART1_TBR (rw) register accessor: an alias for `Reg<UART1_UART1_TBR_SPEC>`"]
pub type UART1_UART1_TBR = crate::Reg<uart1_uart1_tbr::UART1_UART1_TBR_SPEC>;
#[doc = "UART1_TBR"]
pub mod uart1_uart1_tbr;
#[doc = "UART1_IER (rw) register accessor: an alias for `Reg<UART1_IER_SPEC>`"]
pub type UART1_IER = crate::Reg<uart1_ier::UART1_IER_SPEC>;
#[doc = "UART1_IER"]
pub mod uart1_ier;
#[doc = "UART1_IIR (rw) register accessor: an alias for `Reg<UART1_IIR_SPEC>`"]
pub type UART1_IIR = crate::Reg<uart1_iir::UART1_IIR_SPEC>;
#[doc = "UART1_IIR"]
pub mod uart1_iir;
#[doc = "UART1_FCR (rw) register accessor: an alias for `Reg<UART1_FCR_SPEC>`"]
pub type UART1_FCR = crate::Reg<uart1_fcr::UART1_FCR_SPEC>;
#[doc = "UART1_FCR"]
pub mod uart1_fcr;
#[doc = "UART1_LCR (rw) register accessor: an alias for `Reg<UART1_LCR_SPEC>`"]
pub type UART1_LCR = crate::Reg<uart1_lcr::UART1_LCR_SPEC>;
#[doc = "UART1_LCR"]
pub mod uart1_lcr;
#[doc = "UART1_LSR (rw) register accessor: an alias for `Reg<UART1_LSR_SPEC>`"]
pub type UART1_LSR = crate::Reg<uart1_lsr::UART1_LSR_SPEC>;
#[doc = "UART1_LSR"]
pub mod uart1_lsr;
#[doc = "UART1_TPR (rw) register accessor: an alias for `Reg<UART1_TPR_SPEC>`"]
pub type UART1_TPR = crate::Reg<uart1_tpr::UART1_TPR_SPEC>;
#[doc = "UART1_TPR"]
pub mod uart1_tpr;
#[doc = "UART1_MDR (rw) register accessor: an alias for `Reg<UART1_MDR_SPEC>`"]
pub type UART1_MDR = crate::Reg<uart1_mdr::UART1_MDR_SPEC>;
#[doc = "UART1_MDR"]
pub mod uart1_mdr;
#[doc = "UART1_FSR (rw) register accessor: an alias for `Reg<UART1_FSR_SPEC>`"]
pub type UART1_FSR = crate::Reg<uart1_fsr::UART1_FSR_SPEC>;
#[doc = "UART1_FSR"]
pub mod uart1_fsr;
#[doc = "UART1_DLR (rw) register accessor: an alias for `Reg<UART1_DLR_SPEC>`"]
pub type UART1_DLR = crate::Reg<uart1_dlr::UART1_DLR_SPEC>;
#[doc = "UART1_DLR"]
pub mod uart1_dlr;
#[doc = "UART1_DEGTSTR (rw) register accessor: an alias for `Reg<UART1_DEGTSTR_SPEC>`"]
pub type UART1_DEGTSTR = crate::Reg<uart1_degtstr::UART1_DEGTSTR_SPEC>;
#[doc = "UART1_DEGTSTR"]
pub mod uart1_degtstr;
