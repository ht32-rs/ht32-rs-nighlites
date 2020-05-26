#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_usart1: [u8; 4usize],
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
    pub usart1_ir_dacr: USART1_IRDACR,
    #[doc = "0x2c - USART1_RS485CR"]
    pub usart1_rs485cr: USART1_RS485CR,
    #[doc = "0x30 - USART1_SYNCR"]
    pub usart1_syncr: USART1_SYNCR,
    #[doc = "0x34 - USART1_FSR"]
    pub usart1_fsr: USART1_FSR,
    #[doc = "0x38 - USART1_DLR"]
    pub usart1_dlr: USART1_DLR,
    _reserved15: [u8; 4usize],
    #[doc = "0x40 - USART1_DEGTSTR"]
    pub usart1_degtstr: USART1_DEGTSTR,
}
impl RegisterBlock {
    #[doc = "0x00 - USART1_TBR"]
    #[inline(always)]
    pub fn usart1_tbr(&self) -> &USART1_TBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART1_TBR) }
    }
    #[doc = "0x00 - USART1_TBR"]
    #[inline(always)]
    pub fn usart1_tbr_mut(&self) -> &mut USART1_TBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut USART1_TBR) }
    }
    #[doc = "0x00 - USART1_RBR"]
    #[inline(always)]
    pub fn usart1_rbr(&self) -> &USART1_RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART1_RBR) }
    }
    #[doc = "0x00 - USART1_RBR"]
    #[inline(always)]
    pub fn usart1_rbr_mut(&self) -> &mut USART1_RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut USART1_RBR) }
    }
}
#[doc = "USART1_RBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_rbr](usart1_rbr) module"]
pub type USART1_RBR = crate::Reg<u32, _USART1_RBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_RBR;
#[doc = "`read()` method returns [usart1_rbr::R](usart1_rbr::R) reader structure"]
impl crate::Readable for USART1_RBR {}
#[doc = "`write(|w| ..)` method takes [usart1_rbr::W](usart1_rbr::W) writer structure"]
impl crate::Writable for USART1_RBR {}
#[doc = "USART1_RBR"]
pub mod usart1_rbr;
#[doc = "USART1_TBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_tbr](usart1_tbr) module"]
pub type USART1_TBR = crate::Reg<u32, _USART1_TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_TBR;
#[doc = "`read()` method returns [usart1_tbr::R](usart1_tbr::R) reader structure"]
impl crate::Readable for USART1_TBR {}
#[doc = "`write(|w| ..)` method takes [usart1_tbr::W](usart1_tbr::W) writer structure"]
impl crate::Writable for USART1_TBR {}
#[doc = "USART1_TBR"]
pub mod usart1_tbr;
#[doc = "USART1_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_ier](usart1_ier) module"]
pub type USART1_IER = crate::Reg<u32, _USART1_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_IER;
#[doc = "`read()` method returns [usart1_ier::R](usart1_ier::R) reader structure"]
impl crate::Readable for USART1_IER {}
#[doc = "`write(|w| ..)` method takes [usart1_ier::W](usart1_ier::W) writer structure"]
impl crate::Writable for USART1_IER {}
#[doc = "USART1_IER"]
pub mod usart1_ier;
#[doc = "USART1_IIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_iir](usart1_iir) module"]
pub type USART1_IIR = crate::Reg<u32, _USART1_IIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_IIR;
#[doc = "`read()` method returns [usart1_iir::R](usart1_iir::R) reader structure"]
impl crate::Readable for USART1_IIR {}
#[doc = "`write(|w| ..)` method takes [usart1_iir::W](usart1_iir::W) writer structure"]
impl crate::Writable for USART1_IIR {}
#[doc = "USART1_IIR"]
pub mod usart1_iir;
#[doc = "USART1_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_fcr](usart1_fcr) module"]
pub type USART1_FCR = crate::Reg<u32, _USART1_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_FCR;
#[doc = "`read()` method returns [usart1_fcr::R](usart1_fcr::R) reader structure"]
impl crate::Readable for USART1_FCR {}
#[doc = "`write(|w| ..)` method takes [usart1_fcr::W](usart1_fcr::W) writer structure"]
impl crate::Writable for USART1_FCR {}
#[doc = "USART1_FCR"]
pub mod usart1_fcr;
#[doc = "USART1_LCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_lcr](usart1_lcr) module"]
pub type USART1_LCR = crate::Reg<u32, _USART1_LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_LCR;
#[doc = "`read()` method returns [usart1_lcr::R](usart1_lcr::R) reader structure"]
impl crate::Readable for USART1_LCR {}
#[doc = "`write(|w| ..)` method takes [usart1_lcr::W](usart1_lcr::W) writer structure"]
impl crate::Writable for USART1_LCR {}
#[doc = "USART1_LCR"]
pub mod usart1_lcr;
#[doc = "USART1_MODCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_modcr](usart1_modcr) module"]
pub type USART1_MODCR = crate::Reg<u32, _USART1_MODCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_MODCR;
#[doc = "`read()` method returns [usart1_modcr::R](usart1_modcr::R) reader structure"]
impl crate::Readable for USART1_MODCR {}
#[doc = "`write(|w| ..)` method takes [usart1_modcr::W](usart1_modcr::W) writer structure"]
impl crate::Writable for USART1_MODCR {}
#[doc = "USART1_MODCR"]
pub mod usart1_modcr;
#[doc = "USART1_LSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_lsr](usart1_lsr) module"]
pub type USART1_LSR = crate::Reg<u32, _USART1_LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_LSR;
#[doc = "`read()` method returns [usart1_lsr::R](usart1_lsr::R) reader structure"]
impl crate::Readable for USART1_LSR {}
#[doc = "`write(|w| ..)` method takes [usart1_lsr::W](usart1_lsr::W) writer structure"]
impl crate::Writable for USART1_LSR {}
#[doc = "USART1_LSR"]
pub mod usart1_lsr;
#[doc = "USART1_MODSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_modsr](usart1_modsr) module"]
pub type USART1_MODSR = crate::Reg<u32, _USART1_MODSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_MODSR;
#[doc = "`read()` method returns [usart1_modsr::R](usart1_modsr::R) reader structure"]
impl crate::Readable for USART1_MODSR {}
#[doc = "`write(|w| ..)` method takes [usart1_modsr::W](usart1_modsr::W) writer structure"]
impl crate::Writable for USART1_MODSR {}
#[doc = "USART1_MODSR"]
pub mod usart1_modsr;
#[doc = "USART1_TPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_tpr](usart1_tpr) module"]
pub type USART1_TPR = crate::Reg<u32, _USART1_TPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_TPR;
#[doc = "`read()` method returns [usart1_tpr::R](usart1_tpr::R) reader structure"]
impl crate::Readable for USART1_TPR {}
#[doc = "`write(|w| ..)` method takes [usart1_tpr::W](usart1_tpr::W) writer structure"]
impl crate::Writable for USART1_TPR {}
#[doc = "USART1_TPR"]
pub mod usart1_tpr;
#[doc = "USART1_MDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_mdr](usart1_mdr) module"]
pub type USART1_MDR = crate::Reg<u32, _USART1_MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_MDR;
#[doc = "`read()` method returns [usart1_mdr::R](usart1_mdr::R) reader structure"]
impl crate::Readable for USART1_MDR {}
#[doc = "`write(|w| ..)` method takes [usart1_mdr::W](usart1_mdr::W) writer structure"]
impl crate::Writable for USART1_MDR {}
#[doc = "USART1_MDR"]
pub mod usart1_mdr;
#[doc = "USART1_IrDACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_ir_dacr](usart1_ir_dacr) module"]
pub type USART1_IRDACR = crate::Reg<u32, _USART1_IRDACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_IRDACR;
#[doc = "`read()` method returns [usart1_ir_dacr::R](usart1_ir_dacr::R) reader structure"]
impl crate::Readable for USART1_IRDACR {}
#[doc = "`write(|w| ..)` method takes [usart1_ir_dacr::W](usart1_ir_dacr::W) writer structure"]
impl crate::Writable for USART1_IRDACR {}
#[doc = "USART1_IrDACR"]
pub mod usart1_ir_dacr;
#[doc = "USART1_RS485CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_rs485cr](usart1_rs485cr) module"]
pub type USART1_RS485CR = crate::Reg<u32, _USART1_RS485CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_RS485CR;
#[doc = "`read()` method returns [usart1_rs485cr::R](usart1_rs485cr::R) reader structure"]
impl crate::Readable for USART1_RS485CR {}
#[doc = "`write(|w| ..)` method takes [usart1_rs485cr::W](usart1_rs485cr::W) writer structure"]
impl crate::Writable for USART1_RS485CR {}
#[doc = "USART1_RS485CR"]
pub mod usart1_rs485cr;
#[doc = "USART1_SYNCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_syncr](usart1_syncr) module"]
pub type USART1_SYNCR = crate::Reg<u32, _USART1_SYNCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_SYNCR;
#[doc = "`read()` method returns [usart1_syncr::R](usart1_syncr::R) reader structure"]
impl crate::Readable for USART1_SYNCR {}
#[doc = "`write(|w| ..)` method takes [usart1_syncr::W](usart1_syncr::W) writer structure"]
impl crate::Writable for USART1_SYNCR {}
#[doc = "USART1_SYNCR"]
pub mod usart1_syncr;
#[doc = "USART1_FSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_fsr](usart1_fsr) module"]
pub type USART1_FSR = crate::Reg<u32, _USART1_FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_FSR;
#[doc = "`read()` method returns [usart1_fsr::R](usart1_fsr::R) reader structure"]
impl crate::Readable for USART1_FSR {}
#[doc = "`write(|w| ..)` method takes [usart1_fsr::W](usart1_fsr::W) writer structure"]
impl crate::Writable for USART1_FSR {}
#[doc = "USART1_FSR"]
pub mod usart1_fsr;
#[doc = "USART1_DLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_dlr](usart1_dlr) module"]
pub type USART1_DLR = crate::Reg<u32, _USART1_DLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_DLR;
#[doc = "`read()` method returns [usart1_dlr::R](usart1_dlr::R) reader structure"]
impl crate::Readable for USART1_DLR {}
#[doc = "`write(|w| ..)` method takes [usart1_dlr::W](usart1_dlr::W) writer structure"]
impl crate::Writable for USART1_DLR {}
#[doc = "USART1_DLR"]
pub mod usart1_dlr;
#[doc = "USART1_DEGTSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_degtstr](usart1_degtstr) module"]
pub type USART1_DEGTSTR = crate::Reg<u32, _USART1_DEGTSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART1_DEGTSTR;
#[doc = "`read()` method returns [usart1_degtstr::R](usart1_degtstr::R) reader structure"]
impl crate::Readable for USART1_DEGTSTR {}
#[doc = "`write(|w| ..)` method takes [usart1_degtstr::W](usart1_degtstr::W) writer structure"]
impl crate::Writable for USART1_DEGTSTR {}
#[doc = "USART1_DEGTSTR"]
pub mod usart1_degtstr;
