#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_usart1: [u8; 0x04],
    #[doc = "0x04 - USART1_IER"]
    pub usart1_ier: USART1_IER,
    #[doc = "0x08 - USART1_IIR"]
    pub usart1_iir: USART1_IIR,
    #[doc = "0x0c - USART1_FCR"]
    pub usart1_fcr: USART1_FCR,
    #[doc = "0x10 - USART1_LCR"]
    pub usart1_lcr: USART1_LCR,
    #[doc = "0x14 - USART1_MODCR"]
    pub usart1_modcr: USART1_MODCR,
    #[doc = "0x18 - USART1_LSR"]
    pub usart1_lsr: USART1_LSR,
    #[doc = "0x1c - USART1_MODSR"]
    pub usart1_modsr: USART1_MODSR,
    #[doc = "0x20 - USART1_TPR"]
    pub usart1_tpr: USART1_TPR,
    #[doc = "0x24 - USART1_MDR"]
    pub usart1_mdr: USART1_MDR,
    #[doc = "0x28 - USART1_IrDACR"]
    pub usart1_ir_dacr: USART1_IR_DACR,
    #[doc = "0x2c - USART1_RS485CR"]
    pub usart1_rs485cr: USART1_RS485CR,
    #[doc = "0x30 - USART1_SYNCR"]
    pub usart1_syncr: USART1_SYNCR,
    #[doc = "0x34 - USART1_FSR"]
    pub usart1_fsr: USART1_FSR,
    #[doc = "0x38 - USART1_DLR"]
    pub usart1_dlr: USART1_DLR,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - USART1_DEGTSTR"]
    pub usart1_degtstr: USART1_DEGTSTR,
}
impl RegisterBlock {
    #[doc = "0x00 - USART1_TBR"]
    #[inline(always)]
    pub const fn usart1_usart1_tbr(&self) -> &USART1_USART1_TBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - USART1_RBR"]
    #[inline(always)]
    pub const fn usart1_rbr(&self) -> &USART1_RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "USART1_RBR (rw) register accessor: an alias for `Reg<USART1_RBR_SPEC>`"]
pub type USART1_RBR = crate::Reg<usart1_rbr::USART1_RBR_SPEC>;
#[doc = "USART1_RBR"]
pub mod usart1_rbr;
#[doc = "USART1_USART1_TBR (rw) register accessor: an alias for `Reg<USART1_USART1_TBR_SPEC>`"]
pub type USART1_USART1_TBR = crate::Reg<usart1_usart1_tbr::USART1_USART1_TBR_SPEC>;
#[doc = "USART1_TBR"]
pub mod usart1_usart1_tbr;
#[doc = "USART1_IER (rw) register accessor: an alias for `Reg<USART1_IER_SPEC>`"]
pub type USART1_IER = crate::Reg<usart1_ier::USART1_IER_SPEC>;
#[doc = "USART1_IER"]
pub mod usart1_ier;
#[doc = "USART1_IIR (rw) register accessor: an alias for `Reg<USART1_IIR_SPEC>`"]
pub type USART1_IIR = crate::Reg<usart1_iir::USART1_IIR_SPEC>;
#[doc = "USART1_IIR"]
pub mod usart1_iir;
#[doc = "USART1_FCR (rw) register accessor: an alias for `Reg<USART1_FCR_SPEC>`"]
pub type USART1_FCR = crate::Reg<usart1_fcr::USART1_FCR_SPEC>;
#[doc = "USART1_FCR"]
pub mod usart1_fcr;
#[doc = "USART1_LCR (rw) register accessor: an alias for `Reg<USART1_LCR_SPEC>`"]
pub type USART1_LCR = crate::Reg<usart1_lcr::USART1_LCR_SPEC>;
#[doc = "USART1_LCR"]
pub mod usart1_lcr;
#[doc = "USART1_MODCR (rw) register accessor: an alias for `Reg<USART1_MODCR_SPEC>`"]
pub type USART1_MODCR = crate::Reg<usart1_modcr::USART1_MODCR_SPEC>;
#[doc = "USART1_MODCR"]
pub mod usart1_modcr;
#[doc = "USART1_LSR (rw) register accessor: an alias for `Reg<USART1_LSR_SPEC>`"]
pub type USART1_LSR = crate::Reg<usart1_lsr::USART1_LSR_SPEC>;
#[doc = "USART1_LSR"]
pub mod usart1_lsr;
#[doc = "USART1_MODSR (rw) register accessor: an alias for `Reg<USART1_MODSR_SPEC>`"]
pub type USART1_MODSR = crate::Reg<usart1_modsr::USART1_MODSR_SPEC>;
#[doc = "USART1_MODSR"]
pub mod usart1_modsr;
#[doc = "USART1_TPR (rw) register accessor: an alias for `Reg<USART1_TPR_SPEC>`"]
pub type USART1_TPR = crate::Reg<usart1_tpr::USART1_TPR_SPEC>;
#[doc = "USART1_TPR"]
pub mod usart1_tpr;
#[doc = "USART1_MDR (rw) register accessor: an alias for `Reg<USART1_MDR_SPEC>`"]
pub type USART1_MDR = crate::Reg<usart1_mdr::USART1_MDR_SPEC>;
#[doc = "USART1_MDR"]
pub mod usart1_mdr;
#[doc = "USART1_IrDACR (rw) register accessor: an alias for `Reg<USART1_IR_DACR_SPEC>`"]
pub type USART1_IR_DACR = crate::Reg<usart1_ir_dacr::USART1_IR_DACR_SPEC>;
#[doc = "USART1_IrDACR"]
pub mod usart1_ir_dacr;
#[doc = "USART1_RS485CR (rw) register accessor: an alias for `Reg<USART1_RS485CR_SPEC>`"]
pub type USART1_RS485CR = crate::Reg<usart1_rs485cr::USART1_RS485CR_SPEC>;
#[doc = "USART1_RS485CR"]
pub mod usart1_rs485cr;
#[doc = "USART1_SYNCR (rw) register accessor: an alias for `Reg<USART1_SYNCR_SPEC>`"]
pub type USART1_SYNCR = crate::Reg<usart1_syncr::USART1_SYNCR_SPEC>;
#[doc = "USART1_SYNCR"]
pub mod usart1_syncr;
#[doc = "USART1_FSR (rw) register accessor: an alias for `Reg<USART1_FSR_SPEC>`"]
pub type USART1_FSR = crate::Reg<usart1_fsr::USART1_FSR_SPEC>;
#[doc = "USART1_FSR"]
pub mod usart1_fsr;
#[doc = "USART1_DLR (rw) register accessor: an alias for `Reg<USART1_DLR_SPEC>`"]
pub type USART1_DLR = crate::Reg<usart1_dlr::USART1_DLR_SPEC>;
#[doc = "USART1_DLR"]
pub mod usart1_dlr;
#[doc = "USART1_DEGTSTR (rw) register accessor: an alias for `Reg<USART1_DEGTSTR_SPEC>`"]
pub type USART1_DEGTSTR = crate::Reg<usart1_degtstr::USART1_DEGTSTR_SPEC>;
#[doc = "USART1_DEGTSTR"]
pub mod usart1_degtstr;
