#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - BFTM_CR"]
    pub bftm_cr: BFTM_CR,
    #[doc = "0x04 - BFTM_SR"]
    pub bftm_sr: BFTM_SR,
    #[doc = "0x08 - BFTM_CNTR"]
    pub bftm_cntr: BFTM_CNTR,
    #[doc = "0x0c - BFTM_CMPR"]
    pub bftm_cmpr: BFTM_CMPR,
}
#[doc = "BFTM_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftm_cr](bftm_cr) module"]
pub type BFTM_CR = crate::Reg<u32, _BFTM_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFTM_CR;
#[doc = "`read()` method returns [bftm_cr::R](bftm_cr::R) reader structure"]
impl crate::Readable for BFTM_CR {}
#[doc = "`write(|w| ..)` method takes [bftm_cr::W](bftm_cr::W) writer structure"]
impl crate::Writable for BFTM_CR {}
#[doc = "BFTM_CR"]
pub mod bftm_cr;
#[doc = "BFTM_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftm_sr](bftm_sr) module"]
pub type BFTM_SR = crate::Reg<u32, _BFTM_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFTM_SR;
#[doc = "`read()` method returns [bftm_sr::R](bftm_sr::R) reader structure"]
impl crate::Readable for BFTM_SR {}
#[doc = "`write(|w| ..)` method takes [bftm_sr::W](bftm_sr::W) writer structure"]
impl crate::Writable for BFTM_SR {}
#[doc = "BFTM_SR"]
pub mod bftm_sr;
#[doc = "BFTM_CNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftm_cntr](bftm_cntr) module"]
pub type BFTM_CNTR = crate::Reg<u32, _BFTM_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFTM_CNTR;
#[doc = "`read()` method returns [bftm_cntr::R](bftm_cntr::R) reader structure"]
impl crate::Readable for BFTM_CNTR {}
#[doc = "`write(|w| ..)` method takes [bftm_cntr::W](bftm_cntr::W) writer structure"]
impl crate::Writable for BFTM_CNTR {}
#[doc = "BFTM_CNTR"]
pub mod bftm_cntr;
#[doc = "BFTM_CMPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftm_cmpr](bftm_cmpr) module"]
pub type BFTM_CMPR = crate::Reg<u32, _BFTM_CMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFTM_CMPR;
#[doc = "`read()` method returns [bftm_cmpr::R](bftm_cmpr::R) reader structure"]
impl crate::Readable for BFTM_CMPR {}
#[doc = "`write(|w| ..)` method takes [bftm_cmpr::W](bftm_cmpr::W) writer structure"]
impl crate::Writable for BFTM_CMPR {}
#[doc = "BFTM_CMPR"]
pub mod bftm_cmpr;
