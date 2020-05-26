#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EBI_CR"]
    pub ebi_cr: EBI_CR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - EBI_SR"]
    pub ebi_sr: EBI_SR,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - EBI_ATR"]
    pub ebi_atr: EBI_ATR,
    #[doc = "0x14 - EBI_RTR"]
    pub ebi_rtr: EBI_RTR,
    #[doc = "0x18 - EBI_WTR"]
    pub ebi_wtr: EBI_WTR,
    #[doc = "0x1c - EBI_PR"]
    pub ebi_pr: EBI_PR,
}
#[doc = "EBI_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_cr](ebi_cr) module"]
pub type EBI_CR = crate::Reg<u32, _EBI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_CR;
#[doc = "`read()` method returns [ebi_cr::R](ebi_cr::R) reader structure"]
impl crate::Readable for EBI_CR {}
#[doc = "`write(|w| ..)` method takes [ebi_cr::W](ebi_cr::W) writer structure"]
impl crate::Writable for EBI_CR {}
#[doc = "EBI_CR"]
pub mod ebi_cr;
#[doc = "EBI_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_sr](ebi_sr) module"]
pub type EBI_SR = crate::Reg<u32, _EBI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_SR;
#[doc = "`read()` method returns [ebi_sr::R](ebi_sr::R) reader structure"]
impl crate::Readable for EBI_SR {}
#[doc = "`write(|w| ..)` method takes [ebi_sr::W](ebi_sr::W) writer structure"]
impl crate::Writable for EBI_SR {}
#[doc = "EBI_SR"]
pub mod ebi_sr;
#[doc = "EBI_ATR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_atr](ebi_atr) module"]
pub type EBI_ATR = crate::Reg<u32, _EBI_ATR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_ATR;
#[doc = "`read()` method returns [ebi_atr::R](ebi_atr::R) reader structure"]
impl crate::Readable for EBI_ATR {}
#[doc = "`write(|w| ..)` method takes [ebi_atr::W](ebi_atr::W) writer structure"]
impl crate::Writable for EBI_ATR {}
#[doc = "EBI_ATR"]
pub mod ebi_atr;
#[doc = "EBI_RTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_rtr](ebi_rtr) module"]
pub type EBI_RTR = crate::Reg<u32, _EBI_RTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_RTR;
#[doc = "`read()` method returns [ebi_rtr::R](ebi_rtr::R) reader structure"]
impl crate::Readable for EBI_RTR {}
#[doc = "`write(|w| ..)` method takes [ebi_rtr::W](ebi_rtr::W) writer structure"]
impl crate::Writable for EBI_RTR {}
#[doc = "EBI_RTR"]
pub mod ebi_rtr;
#[doc = "EBI_WTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_wtr](ebi_wtr) module"]
pub type EBI_WTR = crate::Reg<u32, _EBI_WTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_WTR;
#[doc = "`read()` method returns [ebi_wtr::R](ebi_wtr::R) reader structure"]
impl crate::Readable for EBI_WTR {}
#[doc = "`write(|w| ..)` method takes [ebi_wtr::W](ebi_wtr::W) writer structure"]
impl crate::Writable for EBI_WTR {}
#[doc = "EBI_WTR"]
pub mod ebi_wtr;
#[doc = "EBI_PR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_pr](ebi_pr) module"]
pub type EBI_PR = crate::Reg<u32, _EBI_PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_PR;
#[doc = "`read()` method returns [ebi_pr::R](ebi_pr::R) reader structure"]
impl crate::Readable for EBI_PR {}
#[doc = "`write(|w| ..)` method takes [ebi_pr::W](ebi_pr::W) writer structure"]
impl crate::Writable for EBI_PR {}
#[doc = "EBI_PR"]
pub mod ebi_pr;
