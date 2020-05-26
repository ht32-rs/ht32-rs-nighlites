#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2S_CR"]
    pub i2s_cr: I2S_CR,
    #[doc = "0x04 - I2S_IER"]
    pub i2s_ier: I2S_IER,
    #[doc = "0x08 - I2S_CDR"]
    pub i2s_cdr: I2S_CDR,
    #[doc = "0x0c - I2S_TXDR"]
    pub i2s_txdr: I2S_TXDR,
    #[doc = "0x10 - I2S_RXDR"]
    pub i2s_rxdr: I2S_RXDR,
    #[doc = "0x14 - I2S_FCR"]
    pub i2s_fcr: I2S_FCR,
    #[doc = "0x18 - I2S_SR"]
    pub i2s_sr: I2S_SR,
    #[doc = "0x1c - I2S_RCNTR"]
    pub i2s_rcntr: I2S_RCNTR,
}
#[doc = "I2S_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_cr](i2s_cr) module"]
pub type I2S_CR = crate::Reg<u32, _I2S_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CR;
#[doc = "`read()` method returns [i2s_cr::R](i2s_cr::R) reader structure"]
impl crate::Readable for I2S_CR {}
#[doc = "`write(|w| ..)` method takes [i2s_cr::W](i2s_cr::W) writer structure"]
impl crate::Writable for I2S_CR {}
#[doc = "I2S_CR"]
pub mod i2s_cr;
#[doc = "I2S_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_ier](i2s_ier) module"]
pub type I2S_IER = crate::Reg<u32, _I2S_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_IER;
#[doc = "`read()` method returns [i2s_ier::R](i2s_ier::R) reader structure"]
impl crate::Readable for I2S_IER {}
#[doc = "`write(|w| ..)` method takes [i2s_ier::W](i2s_ier::W) writer structure"]
impl crate::Writable for I2S_IER {}
#[doc = "I2S_IER"]
pub mod i2s_ier;
#[doc = "I2S_CDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_cdr](i2s_cdr) module"]
pub type I2S_CDR = crate::Reg<u32, _I2S_CDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_CDR;
#[doc = "`read()` method returns [i2s_cdr::R](i2s_cdr::R) reader structure"]
impl crate::Readable for I2S_CDR {}
#[doc = "`write(|w| ..)` method takes [i2s_cdr::W](i2s_cdr::W) writer structure"]
impl crate::Writable for I2S_CDR {}
#[doc = "I2S_CDR"]
pub mod i2s_cdr;
#[doc = "I2S_TXDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_txdr](i2s_txdr) module"]
pub type I2S_TXDR = crate::Reg<u32, _I2S_TXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_TXDR;
#[doc = "`read()` method returns [i2s_txdr::R](i2s_txdr::R) reader structure"]
impl crate::Readable for I2S_TXDR {}
#[doc = "`write(|w| ..)` method takes [i2s_txdr::W](i2s_txdr::W) writer structure"]
impl crate::Writable for I2S_TXDR {}
#[doc = "I2S_TXDR"]
pub mod i2s_txdr;
#[doc = "I2S_RXDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rxdr](i2s_rxdr) module"]
pub type I2S_RXDR = crate::Reg<u32, _I2S_RXDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RXDR;
#[doc = "`read()` method returns [i2s_rxdr::R](i2s_rxdr::R) reader structure"]
impl crate::Readable for I2S_RXDR {}
#[doc = "`write(|w| ..)` method takes [i2s_rxdr::W](i2s_rxdr::W) writer structure"]
impl crate::Writable for I2S_RXDR {}
#[doc = "I2S_RXDR"]
pub mod i2s_rxdr;
#[doc = "I2S_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_fcr](i2s_fcr) module"]
pub type I2S_FCR = crate::Reg<u32, _I2S_FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_FCR;
#[doc = "`read()` method returns [i2s_fcr::R](i2s_fcr::R) reader structure"]
impl crate::Readable for I2S_FCR {}
#[doc = "`write(|w| ..)` method takes [i2s_fcr::W](i2s_fcr::W) writer structure"]
impl crate::Writable for I2S_FCR {}
#[doc = "I2S_FCR"]
pub mod i2s_fcr;
#[doc = "I2S_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_sr](i2s_sr) module"]
pub type I2S_SR = crate::Reg<u32, _I2S_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_SR;
#[doc = "`read()` method returns [i2s_sr::R](i2s_sr::R) reader structure"]
impl crate::Readable for I2S_SR {}
#[doc = "`write(|w| ..)` method takes [i2s_sr::W](i2s_sr::W) writer structure"]
impl crate::Writable for I2S_SR {}
#[doc = "I2S_SR"]
pub mod i2s_sr;
#[doc = "I2S_RCNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_rcntr](i2s_rcntr) module"]
pub type I2S_RCNTR = crate::Reg<u32, _I2S_RCNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _I2S_RCNTR;
#[doc = "`read()` method returns [i2s_rcntr::R](i2s_rcntr::R) reader structure"]
impl crate::Readable for I2S_RCNTR {}
#[doc = "`write(|w| ..)` method takes [i2s_rcntr::W](i2s_rcntr::W) writer structure"]
impl crate::Writable for I2S_RCNTR {}
#[doc = "I2S_RCNTR"]
pub mod i2s_rcntr;
