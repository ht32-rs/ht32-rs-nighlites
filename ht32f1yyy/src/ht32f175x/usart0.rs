#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_usart0: [u8; 0x04],
    #[doc = "0x04 - USART0_IER"]
    pub usart0_ier: USART0_IER,
    #[doc = "0x08 - USART0_IIR"]
    pub usart0_iir: USART0_IIR,
    #[doc = "0x0c - USART0_FCR"]
    pub usart0_fcr: USART0_FCR,
    #[doc = "0x10 - USART0_LCR"]
    pub usart0_lcr: USART0_LCR,
    #[doc = "0x14 - USART0_MODCR"]
    pub usart0_modcr: USART0_MODCR,
    #[doc = "0x18 - USART0_LSR"]
    pub usart0_lsr: USART0_LSR,
    #[doc = "0x1c - USART0_MODSR"]
    pub usart0_modsr: USART0_MODSR,
    #[doc = "0x20 - USART0_TPR"]
    pub usart0_tpr: USART0_TPR,
    #[doc = "0x24 - USART0_MDR"]
    pub usart0_mdr: USART0_MDR,
    #[doc = "0x28 - USART0_IrDACR"]
    pub usart0_ir_dacr: USART0_IR_DACR,
    #[doc = "0x2c - USART0_RS485CR"]
    pub usart0_rs485cr: USART0_RS485CR,
    #[doc = "0x30 - USART0_SYNCR"]
    pub usart0_syncr: USART0_SYNCR,
    #[doc = "0x34 - USART0_FSR"]
    pub usart0_fsr: USART0_FSR,
    #[doc = "0x38 - USART0_DLR"]
    pub usart0_dlr: USART0_DLR,
    _reserved15: [u8; 0x04],
    #[doc = "0x40 - USART0_DEGTSTR"]
    pub usart0_degtstr: USART0_DEGTSTR,
}
impl RegisterBlock {
    #[doc = "0x00 - USART0_TBR"]
    #[inline(always)]
    pub const fn usart0_usart0_tbr(&self) -> &USART0_USART0_TBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
    #[doc = "0x00 - USART0_RBR"]
    #[inline(always)]
    pub const fn usart0_rbr(&self) -> &USART0_RBR {
        unsafe { &*(self as *const Self).cast::<u8>().add(0usize).cast() }
    }
}
#[doc = "USART0_RBR (rw) register accessor: an alias for `Reg<USART0_RBR_SPEC>`"]
pub type USART0_RBR = crate::Reg<usart0_rbr::USART0_RBR_SPEC>;
#[doc = "USART0_RBR"]
pub mod usart0_rbr;
#[doc = "USART0_USART0_TBR (rw) register accessor: an alias for `Reg<USART0_USART0_TBR_SPEC>`"]
pub type USART0_USART0_TBR = crate::Reg<usart0_usart0_tbr::USART0_USART0_TBR_SPEC>;
#[doc = "USART0_TBR"]
pub mod usart0_usart0_tbr;
#[doc = "USART0_IER (rw) register accessor: an alias for `Reg<USART0_IER_SPEC>`"]
pub type USART0_IER = crate::Reg<usart0_ier::USART0_IER_SPEC>;
#[doc = "USART0_IER"]
pub mod usart0_ier;
#[doc = "USART0_IIR (rw) register accessor: an alias for `Reg<USART0_IIR_SPEC>`"]
pub type USART0_IIR = crate::Reg<usart0_iir::USART0_IIR_SPEC>;
#[doc = "USART0_IIR"]
pub mod usart0_iir;
#[doc = "USART0_FCR (rw) register accessor: an alias for `Reg<USART0_FCR_SPEC>`"]
pub type USART0_FCR = crate::Reg<usart0_fcr::USART0_FCR_SPEC>;
#[doc = "USART0_FCR"]
pub mod usart0_fcr;
#[doc = "USART0_LCR (rw) register accessor: an alias for `Reg<USART0_LCR_SPEC>`"]
pub type USART0_LCR = crate::Reg<usart0_lcr::USART0_LCR_SPEC>;
#[doc = "USART0_LCR"]
pub mod usart0_lcr;
#[doc = "USART0_MODCR (rw) register accessor: an alias for `Reg<USART0_MODCR_SPEC>`"]
pub type USART0_MODCR = crate::Reg<usart0_modcr::USART0_MODCR_SPEC>;
#[doc = "USART0_MODCR"]
pub mod usart0_modcr;
#[doc = "USART0_LSR (rw) register accessor: an alias for `Reg<USART0_LSR_SPEC>`"]
pub type USART0_LSR = crate::Reg<usart0_lsr::USART0_LSR_SPEC>;
#[doc = "USART0_LSR"]
pub mod usart0_lsr;
#[doc = "USART0_MODSR (rw) register accessor: an alias for `Reg<USART0_MODSR_SPEC>`"]
pub type USART0_MODSR = crate::Reg<usart0_modsr::USART0_MODSR_SPEC>;
#[doc = "USART0_MODSR"]
pub mod usart0_modsr;
#[doc = "USART0_TPR (rw) register accessor: an alias for `Reg<USART0_TPR_SPEC>`"]
pub type USART0_TPR = crate::Reg<usart0_tpr::USART0_TPR_SPEC>;
#[doc = "USART0_TPR"]
pub mod usart0_tpr;
#[doc = "USART0_MDR (rw) register accessor: an alias for `Reg<USART0_MDR_SPEC>`"]
pub type USART0_MDR = crate::Reg<usart0_mdr::USART0_MDR_SPEC>;
#[doc = "USART0_MDR"]
pub mod usart0_mdr;
#[doc = "USART0_IrDACR (rw) register accessor: an alias for `Reg<USART0_IR_DACR_SPEC>`"]
pub type USART0_IR_DACR = crate::Reg<usart0_ir_dacr::USART0_IR_DACR_SPEC>;
#[doc = "USART0_IrDACR"]
pub mod usart0_ir_dacr;
#[doc = "USART0_RS485CR (rw) register accessor: an alias for `Reg<USART0_RS485CR_SPEC>`"]
pub type USART0_RS485CR = crate::Reg<usart0_rs485cr::USART0_RS485CR_SPEC>;
#[doc = "USART0_RS485CR"]
pub mod usart0_rs485cr;
#[doc = "USART0_SYNCR (rw) register accessor: an alias for `Reg<USART0_SYNCR_SPEC>`"]
pub type USART0_SYNCR = crate::Reg<usart0_syncr::USART0_SYNCR_SPEC>;
#[doc = "USART0_SYNCR"]
pub mod usart0_syncr;
#[doc = "USART0_FSR (rw) register accessor: an alias for `Reg<USART0_FSR_SPEC>`"]
pub type USART0_FSR = crate::Reg<usart0_fsr::USART0_FSR_SPEC>;
#[doc = "USART0_FSR"]
pub mod usart0_fsr;
#[doc = "USART0_DLR (rw) register accessor: an alias for `Reg<USART0_DLR_SPEC>`"]
pub type USART0_DLR = crate::Reg<usart0_dlr::USART0_DLR_SPEC>;
#[doc = "USART0_DLR"]
pub mod usart0_dlr;
#[doc = "USART0_DEGTSTR (rw) register accessor: an alias for `Reg<USART0_DEGTSTR_SPEC>`"]
pub type USART0_DEGTSTR = crate::Reg<usart0_degtstr::USART0_DEGTSTR_SPEC>;
#[doc = "USART0_DEGTSTR"]
pub mod usart0_degtstr;
