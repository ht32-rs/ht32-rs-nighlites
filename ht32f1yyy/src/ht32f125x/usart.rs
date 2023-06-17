#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_usart: [u8; 0x04],
    #[doc = "0x04 - USART_IER"]
    pub usart_ier: USART_IER,
    #[doc = "0x08 - USART_IIR"]
    pub usart_iir: USART_IIR,
    #[doc = "0x0c - USART_FCR"]
    pub usart_fcr: USART_FCR,
    #[doc = "0x10 - USART_LCR"]
    pub usart_lcr: USART_LCR,
    #[doc = "0x14 - USART_MODCR"]
    pub usart_modcr: USART_MODCR,
    #[doc = "0x18 - USART_LSR"]
    pub usart_lsr: USART_LSR,
    #[doc = "0x1c - USART_MODSR"]
    pub usart_modsr: USART_MODSR,
    #[doc = "0x20 - USART_TPR"]
    pub usart_tpr: USART_TPR,
    #[doc = "0x24 - USART_MDR"]
    pub usart_mdr: USART_MDR,
    #[doc = "0x28 - USART_IrDACR"]
    pub usart_ir_dacr: USART_IR_DACR,
    #[doc = "0x2c - USART_RS485CR"]
    pub usart_rs485cr: USART_RS485CR,
    #[doc = "0x30 - USART_SYNCR"]
    pub usart_syncr: USART_SYNCR,
    #[doc = "0x34 - USART_DEGTSTR"]
    pub usart_degtstr: USART_DEGTSTR,
    #[doc = "0x38 - USART_DLR"]
    pub usart_dlr: USART_DLR,
}
impl RegisterBlock {
    #[doc = "0x00 - USART_TBR"]
    #[inline(always)]
    pub const fn usart_usart_tbr(&self) -> &USART_USART_TBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - USART_RBR"]
    #[inline(always)]
    pub const fn usart_rbr(&self) -> &USART_RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "USART_RBR (rw) register accessor: an alias for `Reg<USART_RBR_SPEC>`"]
pub type USART_RBR = crate::Reg<usart_rbr::USART_RBR_SPEC>;
#[doc = "USART_RBR"]
pub mod usart_rbr;
#[doc = "USART_USART_TBR (rw) register accessor: an alias for `Reg<USART_USART_TBR_SPEC>`"]
pub type USART_USART_TBR = crate::Reg<usart_usart_tbr::USART_USART_TBR_SPEC>;
#[doc = "USART_TBR"]
pub mod usart_usart_tbr;
#[doc = "USART_IER (rw) register accessor: an alias for `Reg<USART_IER_SPEC>`"]
pub type USART_IER = crate::Reg<usart_ier::USART_IER_SPEC>;
#[doc = "USART_IER"]
pub mod usart_ier;
#[doc = "USART_IIR (rw) register accessor: an alias for `Reg<USART_IIR_SPEC>`"]
pub type USART_IIR = crate::Reg<usart_iir::USART_IIR_SPEC>;
#[doc = "USART_IIR"]
pub mod usart_iir;
#[doc = "USART_FCR (rw) register accessor: an alias for `Reg<USART_FCR_SPEC>`"]
pub type USART_FCR = crate::Reg<usart_fcr::USART_FCR_SPEC>;
#[doc = "USART_FCR"]
pub mod usart_fcr;
#[doc = "USART_LCR (rw) register accessor: an alias for `Reg<USART_LCR_SPEC>`"]
pub type USART_LCR = crate::Reg<usart_lcr::USART_LCR_SPEC>;
#[doc = "USART_LCR"]
pub mod usart_lcr;
#[doc = "USART_MODCR (rw) register accessor: an alias for `Reg<USART_MODCR_SPEC>`"]
pub type USART_MODCR = crate::Reg<usart_modcr::USART_MODCR_SPEC>;
#[doc = "USART_MODCR"]
pub mod usart_modcr;
#[doc = "USART_LSR (rw) register accessor: an alias for `Reg<USART_LSR_SPEC>`"]
pub type USART_LSR = crate::Reg<usart_lsr::USART_LSR_SPEC>;
#[doc = "USART_LSR"]
pub mod usart_lsr;
#[doc = "USART_MODSR (rw) register accessor: an alias for `Reg<USART_MODSR_SPEC>`"]
pub type USART_MODSR = crate::Reg<usart_modsr::USART_MODSR_SPEC>;
#[doc = "USART_MODSR"]
pub mod usart_modsr;
#[doc = "USART_TPR (rw) register accessor: an alias for `Reg<USART_TPR_SPEC>`"]
pub type USART_TPR = crate::Reg<usart_tpr::USART_TPR_SPEC>;
#[doc = "USART_TPR"]
pub mod usart_tpr;
#[doc = "USART_MDR (rw) register accessor: an alias for `Reg<USART_MDR_SPEC>`"]
pub type USART_MDR = crate::Reg<usart_mdr::USART_MDR_SPEC>;
#[doc = "USART_MDR"]
pub mod usart_mdr;
#[doc = "USART_IrDACR (rw) register accessor: an alias for `Reg<USART_IR_DACR_SPEC>`"]
pub type USART_IR_DACR = crate::Reg<usart_ir_dacr::USART_IR_DACR_SPEC>;
#[doc = "USART_IrDACR"]
pub mod usart_ir_dacr;
#[doc = "USART_RS485CR (rw) register accessor: an alias for `Reg<USART_RS485CR_SPEC>`"]
pub type USART_RS485CR = crate::Reg<usart_rs485cr::USART_RS485CR_SPEC>;
#[doc = "USART_RS485CR"]
pub mod usart_rs485cr;
#[doc = "USART_SYNCR (rw) register accessor: an alias for `Reg<USART_SYNCR_SPEC>`"]
pub type USART_SYNCR = crate::Reg<usart_syncr::USART_SYNCR_SPEC>;
#[doc = "USART_SYNCR"]
pub mod usart_syncr;
#[doc = "USART_DEGTSTR (rw) register accessor: an alias for `Reg<USART_DEGTSTR_SPEC>`"]
pub type USART_DEGTSTR = crate::Reg<usart_degtstr::USART_DEGTSTR_SPEC>;
#[doc = "USART_DEGTSTR"]
pub mod usart_degtstr;
#[doc = "USART_DLR (rw) register accessor: an alias for `Reg<USART_DLR_SPEC>`"]
pub type USART_DLR = crate::Reg<usart_dlr::USART_DLR_SPEC>;
#[doc = "USART_DLR"]
pub mod usart_dlr;
