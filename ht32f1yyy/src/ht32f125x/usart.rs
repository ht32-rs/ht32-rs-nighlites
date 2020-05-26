#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_usart: [u8; 4usize],
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
    pub usart_ir_dacr: USART_IRDACR,
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
    pub fn usart_tbr(&self) -> &USART_TBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART_TBR) }
    }
    #[doc = "0x00 - USART_TBR"]
    #[inline(always)]
    pub fn usart_tbr_mut(&self) -> &mut USART_TBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut USART_TBR) }
    }
    #[doc = "0x00 - USART_RBR"]
    #[inline(always)]
    pub fn usart_rbr(&self) -> &USART_RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const USART_RBR) }
    }
    #[doc = "0x00 - USART_RBR"]
    #[inline(always)]
    pub fn usart_rbr_mut(&self) -> &mut USART_RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut USART_RBR) }
    }
}
#[doc = "USART_RBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_rbr](usart_rbr) module"]
pub type USART_RBR = crate::Reg<u32, _USART_RBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_RBR;
#[doc = "`read()` method returns [usart_rbr::R](usart_rbr::R) reader structure"]
impl crate::Readable for USART_RBR {}
#[doc = "`write(|w| ..)` method takes [usart_rbr::W](usart_rbr::W) writer structure"]
impl crate::Writable for USART_RBR {}
#[doc = "USART_RBR"]
pub mod usart_rbr;
#[doc = "USART_TBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_tbr](usart_tbr) module"]
pub type USART_TBR = crate::Reg<u32, _USART_TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_TBR;
#[doc = "`read()` method returns [usart_tbr::R](usart_tbr::R) reader structure"]
impl crate::Readable for USART_TBR {}
#[doc = "`write(|w| ..)` method takes [usart_tbr::W](usart_tbr::W) writer structure"]
impl crate::Writable for USART_TBR {}
#[doc = "USART_TBR"]
pub mod usart_tbr;
#[doc = "USART_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_ier](usart_ier) module"]
pub type USART_IER = crate::Reg<u32, _USART_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_IER;
#[doc = "`read()` method returns [usart_ier::R](usart_ier::R) reader structure"]
impl crate::Readable for USART_IER {}
#[doc = "`write(|w| ..)` method takes [usart_ier::W](usart_ier::W) writer structure"]
impl crate::Writable for USART_IER {}
#[doc = "USART_IER"]
pub mod usart_ier;
#[doc = "USART_IIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_iir](usart_iir) module"]
pub type USART_IIR = crate::Reg<u32, _USART_IIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_IIR;
#[doc = "`read()` method returns [usart_iir::R](usart_iir::R) reader structure"]
impl crate::Readable for USART_IIR {}
#[doc = "`write(|w| ..)` method takes [usart_iir::W](usart_iir::W) writer structure"]
impl crate::Writable for USART_IIR {}
#[doc = "USART_IIR"]
pub mod usart_iir;
#[doc = "USART_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_fcr](usart_fcr) module"]
pub type USART_FCR = crate::Reg<u32, _USART_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_FCR;
#[doc = "`read()` method returns [usart_fcr::R](usart_fcr::R) reader structure"]
impl crate::Readable for USART_FCR {}
#[doc = "`write(|w| ..)` method takes [usart_fcr::W](usart_fcr::W) writer structure"]
impl crate::Writable for USART_FCR {}
#[doc = "USART_FCR"]
pub mod usart_fcr;
#[doc = "USART_LCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_lcr](usart_lcr) module"]
pub type USART_LCR = crate::Reg<u32, _USART_LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_LCR;
#[doc = "`read()` method returns [usart_lcr::R](usart_lcr::R) reader structure"]
impl crate::Readable for USART_LCR {}
#[doc = "`write(|w| ..)` method takes [usart_lcr::W](usart_lcr::W) writer structure"]
impl crate::Writable for USART_LCR {}
#[doc = "USART_LCR"]
pub mod usart_lcr;
#[doc = "USART_MODCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_modcr](usart_modcr) module"]
pub type USART_MODCR = crate::Reg<u32, _USART_MODCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_MODCR;
#[doc = "`read()` method returns [usart_modcr::R](usart_modcr::R) reader structure"]
impl crate::Readable for USART_MODCR {}
#[doc = "`write(|w| ..)` method takes [usart_modcr::W](usart_modcr::W) writer structure"]
impl crate::Writable for USART_MODCR {}
#[doc = "USART_MODCR"]
pub mod usart_modcr;
#[doc = "USART_LSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_lsr](usart_lsr) module"]
pub type USART_LSR = crate::Reg<u32, _USART_LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_LSR;
#[doc = "`read()` method returns [usart_lsr::R](usart_lsr::R) reader structure"]
impl crate::Readable for USART_LSR {}
#[doc = "`write(|w| ..)` method takes [usart_lsr::W](usart_lsr::W) writer structure"]
impl crate::Writable for USART_LSR {}
#[doc = "USART_LSR"]
pub mod usart_lsr;
#[doc = "USART_MODSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_modsr](usart_modsr) module"]
pub type USART_MODSR = crate::Reg<u32, _USART_MODSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_MODSR;
#[doc = "`read()` method returns [usart_modsr::R](usart_modsr::R) reader structure"]
impl crate::Readable for USART_MODSR {}
#[doc = "`write(|w| ..)` method takes [usart_modsr::W](usart_modsr::W) writer structure"]
impl crate::Writable for USART_MODSR {}
#[doc = "USART_MODSR"]
pub mod usart_modsr;
#[doc = "USART_TPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_tpr](usart_tpr) module"]
pub type USART_TPR = crate::Reg<u32, _USART_TPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_TPR;
#[doc = "`read()` method returns [usart_tpr::R](usart_tpr::R) reader structure"]
impl crate::Readable for USART_TPR {}
#[doc = "`write(|w| ..)` method takes [usart_tpr::W](usart_tpr::W) writer structure"]
impl crate::Writable for USART_TPR {}
#[doc = "USART_TPR"]
pub mod usart_tpr;
#[doc = "USART_MDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_mdr](usart_mdr) module"]
pub type USART_MDR = crate::Reg<u32, _USART_MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_MDR;
#[doc = "`read()` method returns [usart_mdr::R](usart_mdr::R) reader structure"]
impl crate::Readable for USART_MDR {}
#[doc = "`write(|w| ..)` method takes [usart_mdr::W](usart_mdr::W) writer structure"]
impl crate::Writable for USART_MDR {}
#[doc = "USART_MDR"]
pub mod usart_mdr;
#[doc = "USART_IrDACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_ir_dacr](usart_ir_dacr) module"]
pub type USART_IRDACR = crate::Reg<u32, _USART_IRDACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_IRDACR;
#[doc = "`read()` method returns [usart_ir_dacr::R](usart_ir_dacr::R) reader structure"]
impl crate::Readable for USART_IRDACR {}
#[doc = "`write(|w| ..)` method takes [usart_ir_dacr::W](usart_ir_dacr::W) writer structure"]
impl crate::Writable for USART_IRDACR {}
#[doc = "USART_IrDACR"]
pub mod usart_ir_dacr;
#[doc = "USART_RS485CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_rs485cr](usart_rs485cr) module"]
pub type USART_RS485CR = crate::Reg<u32, _USART_RS485CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_RS485CR;
#[doc = "`read()` method returns [usart_rs485cr::R](usart_rs485cr::R) reader structure"]
impl crate::Readable for USART_RS485CR {}
#[doc = "`write(|w| ..)` method takes [usart_rs485cr::W](usart_rs485cr::W) writer structure"]
impl crate::Writable for USART_RS485CR {}
#[doc = "USART_RS485CR"]
pub mod usart_rs485cr;
#[doc = "USART_SYNCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_syncr](usart_syncr) module"]
pub type USART_SYNCR = crate::Reg<u32, _USART_SYNCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_SYNCR;
#[doc = "`read()` method returns [usart_syncr::R](usart_syncr::R) reader structure"]
impl crate::Readable for USART_SYNCR {}
#[doc = "`write(|w| ..)` method takes [usart_syncr::W](usart_syncr::W) writer structure"]
impl crate::Writable for USART_SYNCR {}
#[doc = "USART_SYNCR"]
pub mod usart_syncr;
#[doc = "USART_DEGTSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_degtstr](usart_degtstr) module"]
pub type USART_DEGTSTR = crate::Reg<u32, _USART_DEGTSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_DEGTSTR;
#[doc = "`read()` method returns [usart_degtstr::R](usart_degtstr::R) reader structure"]
impl crate::Readable for USART_DEGTSTR {}
#[doc = "`write(|w| ..)` method takes [usart_degtstr::W](usart_degtstr::W) writer structure"]
impl crate::Writable for USART_DEGTSTR {}
#[doc = "USART_DEGTSTR"]
pub mod usart_degtstr;
#[doc = "USART_DLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_dlr](usart_dlr) module"]
pub type USART_DLR = crate::Reg<u32, _USART_DLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USART_DLR;
#[doc = "`read()` method returns [usart_dlr::R](usart_dlr::R) reader structure"]
impl crate::Readable for USART_DLR {}
#[doc = "`write(|w| ..)` method takes [usart_dlr::W](usart_dlr::W) writer structure"]
impl crate::Writable for USART_DLR {}
#[doc = "USART_DLR"]
pub mod usart_dlr;
