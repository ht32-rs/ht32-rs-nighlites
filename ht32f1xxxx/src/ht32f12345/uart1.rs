#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uart1: [u8; 4usize],
    #[doc = "0x04 - UART1_IER"]
    pub uart1_ier: UART1_IER,
    #[doc = "0x08 - UART1_IIR"]
    pub uart1_iir: UART1_IIR,
    #[doc = "0x0c - UART1_FCR"]
    pub uart1_fcr: UART1_FCR,
    #[doc = "0x10 - UART1_LCR"]
    pub uart1_lcr: UART1_LCR,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - UART1_LSR"]
    pub uart1_lsr: UART1_LSR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - UART1_TPR"]
    pub uart1_tpr: UART1_TPR,
    #[doc = "0x24 - UART1_MDR"]
    pub uart1_mdr: UART1_MDR,
    _reserved8: [u8; 12usize],
    #[doc = "0x34 - UART1_FSR"]
    pub uart1_fsr: UART1_FSR,
    #[doc = "0x38 - UART1_DLR"]
    pub uart1_dlr: UART1_DLR,
    _reserved10: [u8; 4usize],
    #[doc = "0x40 - UART1_DEGTSTR"]
    pub uart1_degtstr: UART1_DEGTSTR,
}
impl RegisterBlock {
    #[doc = "0x00 - UART1_TBR"]
    #[inline(always)]
    pub fn uart1_tbr(&self) -> &UART1_TBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UART1_TBR) }
    }
    #[doc = "0x00 - UART1_TBR"]
    #[inline(always)]
    pub fn uart1_tbr_mut(&self) -> &mut UART1_TBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UART1_TBR) }
    }
    #[doc = "0x00 - UART1_RBR"]
    #[inline(always)]
    pub fn uart1_rbr(&self) -> &UART1_RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UART1_RBR) }
    }
    #[doc = "0x00 - UART1_RBR"]
    #[inline(always)]
    pub fn uart1_rbr_mut(&self) -> &mut UART1_RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UART1_RBR) }
    }
}
#[doc = "UART1_RBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_rbr](uart1_rbr) module"]
pub type UART1_RBR = crate::Reg<u32, _UART1_RBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_RBR;
#[doc = "`read()` method returns [uart1_rbr::R](uart1_rbr::R) reader structure"]
impl crate::Readable for UART1_RBR {}
#[doc = "`write(|w| ..)` method takes [uart1_rbr::W](uart1_rbr::W) writer structure"]
impl crate::Writable for UART1_RBR {}
#[doc = "UART1_RBR"]
pub mod uart1_rbr;
#[doc = "UART1_TBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_tbr](uart1_tbr) module"]
pub type UART1_TBR = crate::Reg<u32, _UART1_TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_TBR;
#[doc = "`read()` method returns [uart1_tbr::R](uart1_tbr::R) reader structure"]
impl crate::Readable for UART1_TBR {}
#[doc = "`write(|w| ..)` method takes [uart1_tbr::W](uart1_tbr::W) writer structure"]
impl crate::Writable for UART1_TBR {}
#[doc = "UART1_TBR"]
pub mod uart1_tbr;
#[doc = "UART1_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_ier](uart1_ier) module"]
pub type UART1_IER = crate::Reg<u32, _UART1_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_IER;
#[doc = "`read()` method returns [uart1_ier::R](uart1_ier::R) reader structure"]
impl crate::Readable for UART1_IER {}
#[doc = "`write(|w| ..)` method takes [uart1_ier::W](uart1_ier::W) writer structure"]
impl crate::Writable for UART1_IER {}
#[doc = "UART1_IER"]
pub mod uart1_ier;
#[doc = "UART1_IIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_iir](uart1_iir) module"]
pub type UART1_IIR = crate::Reg<u32, _UART1_IIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_IIR;
#[doc = "`read()` method returns [uart1_iir::R](uart1_iir::R) reader structure"]
impl crate::Readable for UART1_IIR {}
#[doc = "`write(|w| ..)` method takes [uart1_iir::W](uart1_iir::W) writer structure"]
impl crate::Writable for UART1_IIR {}
#[doc = "UART1_IIR"]
pub mod uart1_iir;
#[doc = "UART1_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_fcr](uart1_fcr) module"]
pub type UART1_FCR = crate::Reg<u32, _UART1_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_FCR;
#[doc = "`read()` method returns [uart1_fcr::R](uart1_fcr::R) reader structure"]
impl crate::Readable for UART1_FCR {}
#[doc = "`write(|w| ..)` method takes [uart1_fcr::W](uart1_fcr::W) writer structure"]
impl crate::Writable for UART1_FCR {}
#[doc = "UART1_FCR"]
pub mod uart1_fcr;
#[doc = "UART1_LCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_lcr](uart1_lcr) module"]
pub type UART1_LCR = crate::Reg<u32, _UART1_LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_LCR;
#[doc = "`read()` method returns [uart1_lcr::R](uart1_lcr::R) reader structure"]
impl crate::Readable for UART1_LCR {}
#[doc = "`write(|w| ..)` method takes [uart1_lcr::W](uart1_lcr::W) writer structure"]
impl crate::Writable for UART1_LCR {}
#[doc = "UART1_LCR"]
pub mod uart1_lcr;
#[doc = "UART1_LSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_lsr](uart1_lsr) module"]
pub type UART1_LSR = crate::Reg<u32, _UART1_LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_LSR;
#[doc = "`read()` method returns [uart1_lsr::R](uart1_lsr::R) reader structure"]
impl crate::Readable for UART1_LSR {}
#[doc = "`write(|w| ..)` method takes [uart1_lsr::W](uart1_lsr::W) writer structure"]
impl crate::Writable for UART1_LSR {}
#[doc = "UART1_LSR"]
pub mod uart1_lsr;
#[doc = "UART1_TPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_tpr](uart1_tpr) module"]
pub type UART1_TPR = crate::Reg<u32, _UART1_TPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_TPR;
#[doc = "`read()` method returns [uart1_tpr::R](uart1_tpr::R) reader structure"]
impl crate::Readable for UART1_TPR {}
#[doc = "`write(|w| ..)` method takes [uart1_tpr::W](uart1_tpr::W) writer structure"]
impl crate::Writable for UART1_TPR {}
#[doc = "UART1_TPR"]
pub mod uart1_tpr;
#[doc = "UART1_MDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_mdr](uart1_mdr) module"]
pub type UART1_MDR = crate::Reg<u32, _UART1_MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_MDR;
#[doc = "`read()` method returns [uart1_mdr::R](uart1_mdr::R) reader structure"]
impl crate::Readable for UART1_MDR {}
#[doc = "`write(|w| ..)` method takes [uart1_mdr::W](uart1_mdr::W) writer structure"]
impl crate::Writable for UART1_MDR {}
#[doc = "UART1_MDR"]
pub mod uart1_mdr;
#[doc = "UART1_FSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_fsr](uart1_fsr) module"]
pub type UART1_FSR = crate::Reg<u32, _UART1_FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_FSR;
#[doc = "`read()` method returns [uart1_fsr::R](uart1_fsr::R) reader structure"]
impl crate::Readable for UART1_FSR {}
#[doc = "`write(|w| ..)` method takes [uart1_fsr::W](uart1_fsr::W) writer structure"]
impl crate::Writable for UART1_FSR {}
#[doc = "UART1_FSR"]
pub mod uart1_fsr;
#[doc = "UART1_DLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_dlr](uart1_dlr) module"]
pub type UART1_DLR = crate::Reg<u32, _UART1_DLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_DLR;
#[doc = "`read()` method returns [uart1_dlr::R](uart1_dlr::R) reader structure"]
impl crate::Readable for UART1_DLR {}
#[doc = "`write(|w| ..)` method takes [uart1_dlr::W](uart1_dlr::W) writer structure"]
impl crate::Writable for UART1_DLR {}
#[doc = "UART1_DLR"]
pub mod uart1_dlr;
#[doc = "UART1_DEGTSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_degtstr](uart1_degtstr) module"]
pub type UART1_DEGTSTR = crate::Reg<u32, _UART1_DEGTSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART1_DEGTSTR;
#[doc = "`read()` method returns [uart1_degtstr::R](uart1_degtstr::R) reader structure"]
impl crate::Readable for UART1_DEGTSTR {}
#[doc = "`write(|w| ..)` method takes [uart1_degtstr::W](uart1_degtstr::W) writer structure"]
impl crate::Writable for UART1_DEGTSTR {}
#[doc = "UART1_DEGTSTR"]
pub mod uart1_degtstr;
