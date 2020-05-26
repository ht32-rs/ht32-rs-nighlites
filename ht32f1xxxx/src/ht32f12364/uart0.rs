#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_uart0: [u8; 4usize],
    #[doc = "0x04 - UART0_IER"]
    pub uart0_ier: UART0_IER,
    #[doc = "0x08 - UART0_IIR"]
    pub uart0_iir: UART0_IIR,
    #[doc = "0x0c - UART0_FCR"]
    pub uart0_fcr: UART0_FCR,
    #[doc = "0x10 - UART0_LCR"]
    pub uart0_lcr: UART0_LCR,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - UART0_LSR"]
    pub uart0_lsr: UART0_LSR,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - UART0_TPR"]
    pub uart0_tpr: UART0_TPR,
    #[doc = "0x24 - UART0_MDR"]
    pub uart0_mdr: UART0_MDR,
    _reserved8: [u8; 12usize],
    #[doc = "0x34 - UART0_FSR"]
    pub uart0_fsr: UART0_FSR,
    #[doc = "0x38 - UART0_DLR"]
    pub uart0_dlr: UART0_DLR,
    _reserved10: [u8; 4usize],
    #[doc = "0x40 - UART0_DEGTSTR"]
    pub uart0_degtstr: UART0_DEGTSTR,
}
impl RegisterBlock {
    #[doc = "0x00 - UART0_TBR"]
    #[inline(always)]
    pub fn uart0_tbr(&self) -> &UART0_TBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UART0_TBR) }
    }
    #[doc = "0x00 - UART0_TBR"]
    #[inline(always)]
    pub fn uart0_tbr_mut(&self) -> &mut UART0_TBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UART0_TBR) }
    }
    #[doc = "0x00 - UART0_RBR"]
    #[inline(always)]
    pub fn uart0_rbr(&self) -> &UART0_RBR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const UART0_RBR) }
    }
    #[doc = "0x00 - UART0_RBR"]
    #[inline(always)]
    pub fn uart0_rbr_mut(&self) -> &mut UART0_RBR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut UART0_RBR) }
    }
}
#[doc = "UART0_RBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_rbr](uart0_rbr) module"]
pub type UART0_RBR = crate::Reg<u32, _UART0_RBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_RBR;
#[doc = "`read()` method returns [uart0_rbr::R](uart0_rbr::R) reader structure"]
impl crate::Readable for UART0_RBR {}
#[doc = "`write(|w| ..)` method takes [uart0_rbr::W](uart0_rbr::W) writer structure"]
impl crate::Writable for UART0_RBR {}
#[doc = "UART0_RBR"]
pub mod uart0_rbr;
#[doc = "UART0_TBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_tbr](uart0_tbr) module"]
pub type UART0_TBR = crate::Reg<u32, _UART0_TBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_TBR;
#[doc = "`read()` method returns [uart0_tbr::R](uart0_tbr::R) reader structure"]
impl crate::Readable for UART0_TBR {}
#[doc = "`write(|w| ..)` method takes [uart0_tbr::W](uart0_tbr::W) writer structure"]
impl crate::Writable for UART0_TBR {}
#[doc = "UART0_TBR"]
pub mod uart0_tbr;
#[doc = "UART0_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_ier](uart0_ier) module"]
pub type UART0_IER = crate::Reg<u32, _UART0_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_IER;
#[doc = "`read()` method returns [uart0_ier::R](uart0_ier::R) reader structure"]
impl crate::Readable for UART0_IER {}
#[doc = "`write(|w| ..)` method takes [uart0_ier::W](uart0_ier::W) writer structure"]
impl crate::Writable for UART0_IER {}
#[doc = "UART0_IER"]
pub mod uart0_ier;
#[doc = "UART0_IIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_iir](uart0_iir) module"]
pub type UART0_IIR = crate::Reg<u32, _UART0_IIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_IIR;
#[doc = "`read()` method returns [uart0_iir::R](uart0_iir::R) reader structure"]
impl crate::Readable for UART0_IIR {}
#[doc = "`write(|w| ..)` method takes [uart0_iir::W](uart0_iir::W) writer structure"]
impl crate::Writable for UART0_IIR {}
#[doc = "UART0_IIR"]
pub mod uart0_iir;
#[doc = "UART0_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_fcr](uart0_fcr) module"]
pub type UART0_FCR = crate::Reg<u32, _UART0_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_FCR;
#[doc = "`read()` method returns [uart0_fcr::R](uart0_fcr::R) reader structure"]
impl crate::Readable for UART0_FCR {}
#[doc = "`write(|w| ..)` method takes [uart0_fcr::W](uart0_fcr::W) writer structure"]
impl crate::Writable for UART0_FCR {}
#[doc = "UART0_FCR"]
pub mod uart0_fcr;
#[doc = "UART0_LCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_lcr](uart0_lcr) module"]
pub type UART0_LCR = crate::Reg<u32, _UART0_LCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_LCR;
#[doc = "`read()` method returns [uart0_lcr::R](uart0_lcr::R) reader structure"]
impl crate::Readable for UART0_LCR {}
#[doc = "`write(|w| ..)` method takes [uart0_lcr::W](uart0_lcr::W) writer structure"]
impl crate::Writable for UART0_LCR {}
#[doc = "UART0_LCR"]
pub mod uart0_lcr;
#[doc = "UART0_LSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_lsr](uart0_lsr) module"]
pub type UART0_LSR = crate::Reg<u32, _UART0_LSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_LSR;
#[doc = "`read()` method returns [uart0_lsr::R](uart0_lsr::R) reader structure"]
impl crate::Readable for UART0_LSR {}
#[doc = "`write(|w| ..)` method takes [uart0_lsr::W](uart0_lsr::W) writer structure"]
impl crate::Writable for UART0_LSR {}
#[doc = "UART0_LSR"]
pub mod uart0_lsr;
#[doc = "UART0_TPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_tpr](uart0_tpr) module"]
pub type UART0_TPR = crate::Reg<u32, _UART0_TPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_TPR;
#[doc = "`read()` method returns [uart0_tpr::R](uart0_tpr::R) reader structure"]
impl crate::Readable for UART0_TPR {}
#[doc = "`write(|w| ..)` method takes [uart0_tpr::W](uart0_tpr::W) writer structure"]
impl crate::Writable for UART0_TPR {}
#[doc = "UART0_TPR"]
pub mod uart0_tpr;
#[doc = "UART0_MDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_mdr](uart0_mdr) module"]
pub type UART0_MDR = crate::Reg<u32, _UART0_MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_MDR;
#[doc = "`read()` method returns [uart0_mdr::R](uart0_mdr::R) reader structure"]
impl crate::Readable for UART0_MDR {}
#[doc = "`write(|w| ..)` method takes [uart0_mdr::W](uart0_mdr::W) writer structure"]
impl crate::Writable for UART0_MDR {}
#[doc = "UART0_MDR"]
pub mod uart0_mdr;
#[doc = "UART0_FSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_fsr](uart0_fsr) module"]
pub type UART0_FSR = crate::Reg<u32, _UART0_FSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_FSR;
#[doc = "`read()` method returns [uart0_fsr::R](uart0_fsr::R) reader structure"]
impl crate::Readable for UART0_FSR {}
#[doc = "`write(|w| ..)` method takes [uart0_fsr::W](uart0_fsr::W) writer structure"]
impl crate::Writable for UART0_FSR {}
#[doc = "UART0_FSR"]
pub mod uart0_fsr;
#[doc = "UART0_DLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_dlr](uart0_dlr) module"]
pub type UART0_DLR = crate::Reg<u32, _UART0_DLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_DLR;
#[doc = "`read()` method returns [uart0_dlr::R](uart0_dlr::R) reader structure"]
impl crate::Readable for UART0_DLR {}
#[doc = "`write(|w| ..)` method takes [uart0_dlr::W](uart0_dlr::W) writer structure"]
impl crate::Writable for UART0_DLR {}
#[doc = "UART0_DLR"]
pub mod uart0_dlr;
#[doc = "UART0_DEGTSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_degtstr](uart0_degtstr) module"]
pub type UART0_DEGTSTR = crate::Reg<u32, _UART0_DEGTSTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UART0_DEGTSTR;
#[doc = "`read()` method returns [uart0_degtstr::R](uart0_degtstr::R) reader structure"]
impl crate::Readable for UART0_DEGTSTR {}
#[doc = "`write(|w| ..)` method takes [uart0_degtstr::W](uart0_degtstr::W) writer structure"]
impl crate::Writable for UART0_DEGTSTR {}
#[doc = "UART0_DEGTSTR"]
pub mod uart0_degtstr;
