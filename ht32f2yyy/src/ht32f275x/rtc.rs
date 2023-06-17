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
#[doc = "RTC_CNT (rw) register accessor: an alias for `Reg<RTC_CNT_SPEC>`"]
pub type RTC_CNT = crate::Reg<rtc_cnt::RTC_CNT_SPEC>;
#[doc = "RTC_CNT"]
pub mod rtc_cnt;
#[doc = "RTC_CMP (rw) register accessor: an alias for `Reg<RTC_CMP_SPEC>`"]
pub type RTC_CMP = crate::Reg<rtc_cmp::RTC_CMP_SPEC>;
#[doc = "RTC_CMP"]
pub mod rtc_cmp;
#[doc = "RTC_CR (rw) register accessor: an alias for `Reg<RTC_CR_SPEC>`"]
pub type RTC_CR = crate::Reg<rtc_cr::RTC_CR_SPEC>;
#[doc = "RTC_CR"]
pub mod rtc_cr;
#[doc = "RTC_SR (rw) register accessor: an alias for `Reg<RTC_SR_SPEC>`"]
pub type RTC_SR = crate::Reg<rtc_sr::RTC_SR_SPEC>;
#[doc = "RTC_SR"]
pub mod rtc_sr;
#[doc = "RTC_IWEN (rw) register accessor: an alias for `Reg<RTC_IWEN_SPEC>`"]
pub type RTC_IWEN = crate::Reg<rtc_iwen::RTC_IWEN_SPEC>;
#[doc = "RTC_IWEN"]
pub mod rtc_iwen;
