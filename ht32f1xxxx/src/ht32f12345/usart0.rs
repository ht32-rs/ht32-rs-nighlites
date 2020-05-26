#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_usart0: [u8; 4usize],
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
    pub usart0_ir_dacr: USART0_IRDACR,
    #[doc = "0x2c - USART0_RS485CR"]
    pub usart0_rs485cr: USART0_RS485CR,
    #[doc = "0x30 - USART0_SYNCR"]
    pub usart0_syncr: USART0_SYNCR,
    #[doc = "0x34 - USART0_FSR"]
    pub usart0_fsr: USART0_FSR,
    #[doc = "0x38 - USART0_DLR"]
    pub usart0_dlr: USART0_DLR,
    _reserved15: [u8; 4usize],
    #[doc = "0x40 - USART0_DEGTSTR"]
    pub usart0_degtstr: USART0_DEGTSTR,
}
impl RegisterBlock {
    #[doc = "0x00 - USART0_TBR"]
    #[inline(always)]
    pub fn usart0_tbr(&self) -> &USART0_TBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART0_TBR) }
    }
    #[doc = "0x00 - USART0_TBR"]
    #[inline(always)]
    pub fn usart0_tbr_mut(&self) -> &mut USART0_TBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut USART0_TBR) }
    }
    #[doc = "0x00 - USART0_RBR"]
    #[inline(always)]
    pub fn usart0_rbr(&self) -> &USART0_RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART0_RBR) }
    }
    #[doc = "0x00 - USART0_RBR"]
    #[inline(always)]
    pub fn usart0_rbr_mut(&self) -> &mut USART0_RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut USART0_RBR) }
    }
}
#[doc = "USART0_RBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_rbr](usart0_rbr) module"]
pub type USART0_RBR = crate::Reg<u32, _USART0_RBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_RBR;
#[doc = "`read()` method returns [usart0_rbr::R](usart0_rbr::R) reader structure"]
impl crate::Readable for USART0_RBR {}
#[doc = "`write(|w| ..)` method takes [usart0_rbr::W](usart0_rbr::W) writer structure"]
impl crate::Writable for USART0_RBR {}
#[doc = "USART0_RBR"]
pub mod usart0_rbr;
#[doc = "USART0_TBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_tbr](usart0_tbr) module"]
pub type USART0_TBR = crate::Reg<u32, _USART0_TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_TBR;
#[doc = "`read()` method returns [usart0_tbr::R](usart0_tbr::R) reader structure"]
impl crate::Readable for USART0_TBR {}
#[doc = "`write(|w| ..)` method takes [usart0_tbr::W](usart0_tbr::W) writer structure"]
impl crate::Writable for USART0_TBR {}
#[doc = "USART0_TBR"]
pub mod usart0_tbr;
#[doc = "USART0_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_ier](usart0_ier) module"]
pub type USART0_IER = crate::Reg<u32, _USART0_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_IER;
#[doc = "`read()` method returns [usart0_ier::R](usart0_ier::R) reader structure"]
impl crate::Readable for USART0_IER {}
#[doc = "`write(|w| ..)` method takes [usart0_ier::W](usart0_ier::W) writer structure"]
impl crate::Writable for USART0_IER {}
#[doc = "USART0_IER"]
pub mod usart0_ier;
#[doc = "USART0_IIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_iir](usart0_iir) module"]
pub type USART0_IIR = crate::Reg<u32, _USART0_IIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_IIR;
#[doc = "`read()` method returns [usart0_iir::R](usart0_iir::R) reader structure"]
impl crate::Readable for USART0_IIR {}
#[doc = "`write(|w| ..)` method takes [usart0_iir::W](usart0_iir::W) writer structure"]
impl crate::Writable for USART0_IIR {}
#[doc = "USART0_IIR"]
pub mod usart0_iir;
#[doc = "USART0_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_fcr](usart0_fcr) module"]
pub type USART0_FCR = crate::Reg<u32, _USART0_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_FCR;
#[doc = "`read()` method returns [usart0_fcr::R](usart0_fcr::R) reader structure"]
impl crate::Readable for USART0_FCR {}
#[doc = "`write(|w| ..)` method takes [usart0_fcr::W](usart0_fcr::W) writer structure"]
impl crate::Writable for USART0_FCR {}
#[doc = "USART0_FCR"]
pub mod usart0_fcr;
#[doc = "USART0_LCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_lcr](usart0_lcr) module"]
pub type USART0_LCR = crate::Reg<u32, _USART0_LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_LCR;
#[doc = "`read()` method returns [usart0_lcr::R](usart0_lcr::R) reader structure"]
impl crate::Readable for USART0_LCR {}
#[doc = "`write(|w| ..)` method takes [usart0_lcr::W](usart0_lcr::W) writer structure"]
impl crate::Writable for USART0_LCR {}
#[doc = "USART0_LCR"]
pub mod usart0_lcr;
#[doc = "USART0_MODCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_modcr](usart0_modcr) module"]
pub type USART0_MODCR = crate::Reg<u32, _USART0_MODCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_MODCR;
#[doc = "`read()` method returns [usart0_modcr::R](usart0_modcr::R) reader structure"]
impl crate::Readable for USART0_MODCR {}
#[doc = "`write(|w| ..)` method takes [usart0_modcr::W](usart0_modcr::W) writer structure"]
impl crate::Writable for USART0_MODCR {}
#[doc = "USART0_MODCR"]
pub mod usart0_modcr;
#[doc = "USART0_LSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_lsr](usart0_lsr) module"]
pub type USART0_LSR = crate::Reg<u32, _USART0_LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_LSR;
#[doc = "`read()` method returns [usart0_lsr::R](usart0_lsr::R) reader structure"]
impl crate::Readable for USART0_LSR {}
#[doc = "`write(|w| ..)` method takes [usart0_lsr::W](usart0_lsr::W) writer structure"]
impl crate::Writable for USART0_LSR {}
#[doc = "USART0_LSR"]
pub mod usart0_lsr;
#[doc = "USART0_MODSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_modsr](usart0_modsr) module"]
pub type USART0_MODSR = crate::Reg<u32, _USART0_MODSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_MODSR;
#[doc = "`read()` method returns [usart0_modsr::R](usart0_modsr::R) reader structure"]
impl crate::Readable for USART0_MODSR {}
#[doc = "`write(|w| ..)` method takes [usart0_modsr::W](usart0_modsr::W) writer structure"]
impl crate::Writable for USART0_MODSR {}
#[doc = "USART0_MODSR"]
pub mod usart0_modsr;
#[doc = "USART0_TPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_tpr](usart0_tpr) module"]
pub type USART0_TPR = crate::Reg<u32, _USART0_TPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_TPR;
#[doc = "`read()` method returns [usart0_tpr::R](usart0_tpr::R) reader structure"]
impl crate::Readable for USART0_TPR {}
#[doc = "`write(|w| ..)` method takes [usart0_tpr::W](usart0_tpr::W) writer structure"]
impl crate::Writable for USART0_TPR {}
#[doc = "USART0_TPR"]
pub mod usart0_tpr;
#[doc = "USART0_MDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_mdr](usart0_mdr) module"]
pub type USART0_MDR = crate::Reg<u32, _USART0_MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_MDR;
#[doc = "`read()` method returns [usart0_mdr::R](usart0_mdr::R) reader structure"]
impl crate::Readable for USART0_MDR {}
#[doc = "`write(|w| ..)` method takes [usart0_mdr::W](usart0_mdr::W) writer structure"]
impl crate::Writable for USART0_MDR {}
#[doc = "USART0_MDR"]
pub mod usart0_mdr;
#[doc = "USART0_IrDACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_ir_dacr](usart0_ir_dacr) module"]
pub type USART0_IRDACR = crate::Reg<u32, _USART0_IRDACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_IRDACR;
#[doc = "`read()` method returns [usart0_ir_dacr::R](usart0_ir_dacr::R) reader structure"]
impl crate::Readable for USART0_IRDACR {}
#[doc = "`write(|w| ..)` method takes [usart0_ir_dacr::W](usart0_ir_dacr::W) writer structure"]
impl crate::Writable for USART0_IRDACR {}
#[doc = "USART0_IrDACR"]
pub mod usart0_ir_dacr;
#[doc = "USART0_RS485CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_rs485cr](usart0_rs485cr) module"]
pub type USART0_RS485CR = crate::Reg<u32, _USART0_RS485CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_RS485CR;
#[doc = "`read()` method returns [usart0_rs485cr::R](usart0_rs485cr::R) reader structure"]
impl crate::Readable for USART0_RS485CR {}
#[doc = "`write(|w| ..)` method takes [usart0_rs485cr::W](usart0_rs485cr::W) writer structure"]
impl crate::Writable for USART0_RS485CR {}
#[doc = "USART0_RS485CR"]
pub mod usart0_rs485cr;
#[doc = "USART0_SYNCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_syncr](usart0_syncr) module"]
pub type USART0_SYNCR = crate::Reg<u32, _USART0_SYNCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_SYNCR;
#[doc = "`read()` method returns [usart0_syncr::R](usart0_syncr::R) reader structure"]
impl crate::Readable for USART0_SYNCR {}
#[doc = "`write(|w| ..)` method takes [usart0_syncr::W](usart0_syncr::W) writer structure"]
impl crate::Writable for USART0_SYNCR {}
#[doc = "USART0_SYNCR"]
pub mod usart0_syncr;
#[doc = "USART0_FSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_fsr](usart0_fsr) module"]
pub type USART0_FSR = crate::Reg<u32, _USART0_FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_FSR;
#[doc = "`read()` method returns [usart0_fsr::R](usart0_fsr::R) reader structure"]
impl crate::Readable for USART0_FSR {}
#[doc = "`write(|w| ..)` method takes [usart0_fsr::W](usart0_fsr::W) writer structure"]
impl crate::Writable for USART0_FSR {}
#[doc = "USART0_FSR"]
pub mod usart0_fsr;
#[doc = "USART0_DLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_dlr](usart0_dlr) module"]
pub type USART0_DLR = crate::Reg<u32, _USART0_DLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_DLR;
#[doc = "`read()` method returns [usart0_dlr::R](usart0_dlr::R) reader structure"]
impl crate::Readable for USART0_DLR {}
#[doc = "`write(|w| ..)` method takes [usart0_dlr::W](usart0_dlr::W) writer structure"]
impl crate::Writable for USART0_DLR {}
#[doc = "USART0_DLR"]
pub mod usart0_dlr;
#[doc = "USART0_DEGTSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_degtstr](usart0_degtstr) module"]
pub type USART0_DEGTSTR = crate::Reg<u32, _USART0_DEGTSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART0_DEGTSTR;
#[doc = "`read()` method returns [usart0_degtstr::R](usart0_degtstr::R) reader structure"]
impl crate::Readable for USART0_DEGTSTR {}
#[doc = "`write(|w| ..)` method takes [usart0_degtstr::W](usart0_degtstr::W) writer structure"]
impl crate::Writable for USART0_DEGTSTR {}
#[doc = "USART0_DEGTSTR"]
pub mod usart0_degtstr;
