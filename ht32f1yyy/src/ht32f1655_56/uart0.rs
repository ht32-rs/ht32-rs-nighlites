#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uart0: [u8; 0x04],
    #[doc = "0x04 - UART0_IER"]
    pub uart0_ier: UART0_IER,
    #[doc = "0x08 - UART0_IIR"]
    pub uart0_iir: UART0_IIR,
    #[doc = "0x0c - UART0_FCR"]
    pub uart0_fcr: UART0_FCR,
    #[doc = "0x10 - UART0_LCR"]
    pub uart0_lcr: UART0_LCR,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - UART0_LSR"]
    pub uart0_lsr: UART0_LSR,
    _reserved6: [u8; 0x04],
    #[doc = "0x20 - UART0_TPR"]
    pub uart0_tpr: UART0_TPR,
    #[doc = "0x24 - UART0_MDR"]
    pub uart0_mdr: UART0_MDR,
    _reserved8: [u8; 0x0c],
    #[doc = "0x34 - UART0_FSR"]
    pub uart0_fsr: UART0_FSR,
    #[doc = "0x38 - UART0_DLR"]
    pub uart0_dlr: UART0_DLR,
    _reserved10: [u8; 0x04],
    #[doc = "0x40 - UART0_DEGTSTR"]
    pub uart0_degtstr: UART0_DEGTSTR,
}
impl RegisterBlock {
    #[doc = "0x00 - UART0_TBR"]
    #[inline(always)]
    pub const fn uart0_uart0_tbr(&self) -> &UART0_UART0_TBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - UART0_RBR"]
    #[inline(always)]
    pub const fn uart0_rbr(&self) -> &UART0_RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "UART0_RBR (rw) register accessor: an alias for `Reg<UART0_RBR_SPEC>`"]
pub type UART0_RBR = crate::Reg<uart0_rbr::UART0_RBR_SPEC>;
#[doc = "UART0_RBR"]
pub mod uart0_rbr;
#[doc = "UART0_UART0_TBR (rw) register accessor: an alias for `Reg<UART0_UART0_TBR_SPEC>`"]
pub type UART0_UART0_TBR = crate::Reg<uart0_uart0_tbr::UART0_UART0_TBR_SPEC>;
#[doc = "UART0_TBR"]
pub mod uart0_uart0_tbr;
#[doc = "UART0_IER (rw) register accessor: an alias for `Reg<UART0_IER_SPEC>`"]
pub type UART0_IER = crate::Reg<uart0_ier::UART0_IER_SPEC>;
#[doc = "UART0_IER"]
pub mod uart0_ier;
#[doc = "UART0_IIR (rw) register accessor: an alias for `Reg<UART0_IIR_SPEC>`"]
pub type UART0_IIR = crate::Reg<uart0_iir::UART0_IIR_SPEC>;
#[doc = "UART0_IIR"]
pub mod uart0_iir;
#[doc = "UART0_FCR (rw) register accessor: an alias for `Reg<UART0_FCR_SPEC>`"]
pub type UART0_FCR = crate::Reg<uart0_fcr::UART0_FCR_SPEC>;
#[doc = "UART0_FCR"]
pub mod uart0_fcr;
#[doc = "UART0_LCR (rw) register accessor: an alias for `Reg<UART0_LCR_SPEC>`"]
pub type UART0_LCR = crate::Reg<uart0_lcr::UART0_LCR_SPEC>;
#[doc = "UART0_LCR"]
pub mod uart0_lcr;
#[doc = "UART0_LSR (rw) register accessor: an alias for `Reg<UART0_LSR_SPEC>`"]
pub type UART0_LSR = crate::Reg<uart0_lsr::UART0_LSR_SPEC>;
#[doc = "UART0_LSR"]
pub mod uart0_lsr;
#[doc = "UART0_TPR (rw) register accessor: an alias for `Reg<UART0_TPR_SPEC>`"]
pub type UART0_TPR = crate::Reg<uart0_tpr::UART0_TPR_SPEC>;
#[doc = "UART0_TPR"]
pub mod uart0_tpr;
#[doc = "UART0_MDR (rw) register accessor: an alias for `Reg<UART0_MDR_SPEC>`"]
pub type UART0_MDR = crate::Reg<uart0_mdr::UART0_MDR_SPEC>;
#[doc = "UART0_MDR"]
pub mod uart0_mdr;
#[doc = "UART0_FSR (rw) register accessor: an alias for `Reg<UART0_FSR_SPEC>`"]
pub type UART0_FSR = crate::Reg<uart0_fsr::UART0_FSR_SPEC>;
#[doc = "UART0_FSR"]
pub mod uart0_fsr;
#[doc = "UART0_DLR (rw) register accessor: an alias for `Reg<UART0_DLR_SPEC>`"]
pub type UART0_DLR = crate::Reg<uart0_dlr::UART0_DLR_SPEC>;
#[doc = "UART0_DLR"]
pub mod uart0_dlr;
#[doc = "UART0_DEGTSTR (rw) register accessor: an alias for `Reg<UART0_DEGTSTR_SPEC>`"]
pub type UART0_DEGTSTR = crate::Reg<uart0_degtstr::UART0_DEGTSTR_SPEC>;
#[doc = "UART0_DEGTSTR"]
pub mod uart0_degtstr;
