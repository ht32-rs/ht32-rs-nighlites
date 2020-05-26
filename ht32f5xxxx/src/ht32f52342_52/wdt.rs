#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WDT_CR"]
    pub wdt_cr: WDT_CR,
    #[doc = "0x04 - WDT_MR0"]
    pub wdt_mr0: WDT_MR0,
    #[doc = "0x08 - WDT_MR1"]
    pub wdt_mr1: WDT_MR1,
    #[doc = "0x0c - WDT_SR"]
    pub wdt_sr: WDT_SR,
    #[doc = "0x10 - WDT_PR"]
    pub wdt_pr: WDT_PR,
}
#[doc = "WDT_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_cr](wdt_cr) module"]
pub type WDT_CR = crate::Reg<u32, _WDT_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_CR;
#[doc = "`read()` method returns [wdt_cr::R](wdt_cr::R) reader structure"]
impl crate::Readable for WDT_CR {}
#[doc = "`write(|w| ..)` method takes [wdt_cr::W](wdt_cr::W) writer structure"]
impl crate::Writable for WDT_CR {}
#[doc = "WDT_CR"]
pub mod wdt_cr;
#[doc = "WDT_MR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_mr0](wdt_mr0) module"]
pub type WDT_MR0 = crate::Reg<u32, _WDT_MR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_MR0;
#[doc = "`read()` method returns [wdt_mr0::R](wdt_mr0::R) reader structure"]
impl crate::Readable for WDT_MR0 {}
#[doc = "`write(|w| ..)` method takes [wdt_mr0::W](wdt_mr0::W) writer structure"]
impl crate::Writable for WDT_MR0 {}
#[doc = "WDT_MR0"]
pub mod wdt_mr0;
#[doc = "WDT_MR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_mr1](wdt_mr1) module"]
pub type WDT_MR1 = crate::Reg<u32, _WDT_MR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_MR1;
#[doc = "`read()` method returns [wdt_mr1::R](wdt_mr1::R) reader structure"]
impl crate::Readable for WDT_MR1 {}
#[doc = "`write(|w| ..)` method takes [wdt_mr1::W](wdt_mr1::W) writer structure"]
impl crate::Writable for WDT_MR1 {}
#[doc = "WDT_MR1"]
pub mod wdt_mr1;
#[doc = "WDT_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_sr](wdt_sr) module"]
pub type WDT_SR = crate::Reg<u32, _WDT_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_SR;
#[doc = "`read()` method returns [wdt_sr::R](wdt_sr::R) reader structure"]
impl crate::Readable for WDT_SR {}
#[doc = "`write(|w| ..)` method takes [wdt_sr::W](wdt_sr::W) writer structure"]
impl crate::Writable for WDT_SR {}
#[doc = "WDT_SR"]
pub mod wdt_sr;
#[doc = "WDT_PR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdt_pr](wdt_pr) module"]
pub type WDT_PR = crate::Reg<u32, _WDT_PR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDT_PR;
#[doc = "`read()` method returns [wdt_pr::R](wdt_pr::R) reader structure"]
impl crate::Readable for WDT_PR {}
#[doc = "`write(|w| ..)` method takes [wdt_pr::W](wdt_pr::W) writer structure"]
impl crate::Writable for WDT_PR {}
#[doc = "WDT_PR"]
pub mod wdt_pr;
