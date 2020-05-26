#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC_CNT"]
    pub rtc_cnt: RTC_CNT,
    #[doc = "0x04 - RTC_CMP"]
    pub rtc_cmp: RTC_CMP,
    #[doc = "0x08 - RTC_CR"]
    pub rtc_cr: RTC_CR,
    #[doc = "0x0c - RTC_SR"]
    pub rtc_sr: RTC_SR,
    #[doc = "0x10 - RTC_IWEN"]
    pub rtc_iwen: RTC_IWEN,
}
#[doc = "RTC_CNT\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cnt](rtc_cnt) module"]
pub type RTC_CNT = crate::Reg<u32, _RTC_CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CNT;
#[doc = "`read()` method returns [rtc_cnt::R](rtc_cnt::R) reader structure"]
impl crate::Readable for RTC_CNT {}
#[doc = "`write(|w| ..)` method takes [rtc_cnt::W](rtc_cnt::W) writer structure"]
impl crate::Writable for RTC_CNT {}
#[doc = "RTC_CNT"]
pub mod rtc_cnt;
#[doc = "RTC_CMP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cmp](rtc_cmp) module"]
pub type RTC_CMP = crate::Reg<u32, _RTC_CMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CMP;
#[doc = "`read()` method returns [rtc_cmp::R](rtc_cmp::R) reader structure"]
impl crate::Readable for RTC_CMP {}
#[doc = "`write(|w| ..)` method takes [rtc_cmp::W](rtc_cmp::W) writer structure"]
impl crate::Writable for RTC_CMP {}
#[doc = "RTC_CMP"]
pub mod rtc_cmp;
#[doc = "RTC_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cr](rtc_cr) module"]
pub type RTC_CR = crate::Reg<u32, _RTC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_CR;
#[doc = "`read()` method returns [rtc_cr::R](rtc_cr::R) reader structure"]
impl crate::Readable for RTC_CR {}
#[doc = "`write(|w| ..)` method takes [rtc_cr::W](rtc_cr::W) writer structure"]
impl crate::Writable for RTC_CR {}
#[doc = "RTC_CR"]
pub mod rtc_cr;
#[doc = "RTC_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_sr](rtc_sr) module"]
pub type RTC_SR = crate::Reg<u32, _RTC_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_SR;
#[doc = "`read()` method returns [rtc_sr::R](rtc_sr::R) reader structure"]
impl crate::Readable for RTC_SR {}
#[doc = "`write(|w| ..)` method takes [rtc_sr::W](rtc_sr::W) writer structure"]
impl crate::Writable for RTC_SR {}
#[doc = "RTC_SR"]
pub mod rtc_sr;
#[doc = "RTC_IWEN\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_iwen](rtc_iwen) module"]
pub type RTC_IWEN = crate::Reg<u32, _RTC_IWEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTC_IWEN;
#[doc = "`read()` method returns [rtc_iwen::R](rtc_iwen::R) reader structure"]
impl crate::Readable for RTC_IWEN {}
#[doc = "`write(|w| ..)` method takes [rtc_iwen::W](rtc_iwen::W) writer structure"]
impl crate::Writable for RTC_IWEN {}
#[doc = "RTC_IWEN"]
pub mod rtc_iwen;
