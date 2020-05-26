#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - ADC_RST"]
    pub adc_rst: ADC_RST,
    #[doc = "0x08 - ADC_CONV"]
    pub adc_conv: ADC_CONV,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - ADC_LST0"]
    pub adc_lst0: ADC_LST0,
    #[doc = "0x14 - ADC_LST1"]
    pub adc_lst1: ADC_LST1,
    _reserved4: [u8; 88usize],
    #[doc = "0x70 - ADC_STR0"]
    pub adc_str0: ADC_STR0,
    #[doc = "0x74 - ADC_STR1"]
    pub adc_str1: ADC_STR1,
    #[doc = "0x78 - ADC_STR2"]
    pub adc_str2: ADC_STR2,
    #[doc = "0x7c - ADC_STR3"]
    pub adc_str3: ADC_STR3,
    #[doc = "0x80 - ADC_STR4"]
    pub adc_str4: ADC_STR4,
    #[doc = "0x84 - ADC_STR5"]
    pub adc_str5: ADC_STR5,
    #[doc = "0x88 - ADC_STR6"]
    pub adc_str6: ADC_STR6,
    #[doc = "0x8c - ADC_STR7"]
    pub adc_str7: ADC_STR7,
    _reserved12: [u8; 32usize],
    #[doc = "0xb0 - ADC_DR0"]
    pub adc_dr0: ADC_DR0,
    #[doc = "0xb4 - ADC_DR1"]
    pub adc_dr1: ADC_DR1,
    #[doc = "0xb8 - ADC_DR2"]
    pub adc_dr2: ADC_DR2,
    #[doc = "0xbc - ADC_DR3"]
    pub adc_dr3: ADC_DR3,
    #[doc = "0xc0 - ADC_DR4"]
    pub adc_dr4: ADC_DR4,
    #[doc = "0xc4 - ADC_DR5"]
    pub adc_dr5: ADC_DR5,
    #[doc = "0xc8 - ADC_DR6"]
    pub adc_dr6: ADC_DR6,
    #[doc = "0xcc - ADC_DR7"]
    pub adc_dr7: ADC_DR7,
    _reserved20: [u8; 48usize],
    #[doc = "0x100 - ADC_TCR"]
    pub adc_tcr: ADC_TCR,
    #[doc = "0x104 - ADC_TSR"]
    pub adc_tsr: ADC_TSR,
    _reserved22: [u8; 24usize],
    #[doc = "0x120 - ADC_WCR"]
    pub adc_wcr: ADC_WCR,
    #[doc = "0x124 - ADC_LTR"]
    pub adc_ltr: ADC_LTR,
    #[doc = "0x128 - ADC_UTR"]
    pub adc_utr: ADC_UTR,
    _reserved25: [u8; 4usize],
    #[doc = "0x130 - ADC_IMR"]
    pub adc_imr: ADC_IMR,
    #[doc = "0x134 - ADC_IRAW"]
    pub adc_iraw: ADC_IRAW,
    #[doc = "0x138 - ADC_IMASK"]
    pub adc_imask: ADC_IMASK,
    #[doc = "0x13c - ADC_ICLR"]
    pub adc_iclr: ADC_ICLR,
}
#[doc = "ADC_RST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_rst](adc_rst) module"]
pub type ADC_RST = crate::Reg<u32, _ADC_RST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_RST;
#[doc = "`read()` method returns [adc_rst::R](adc_rst::R) reader structure"]
impl crate::Readable for ADC_RST {}
#[doc = "`write(|w| ..)` method takes [adc_rst::W](adc_rst::W) writer structure"]
impl crate::Writable for ADC_RST {}
#[doc = "ADC_RST"]
pub mod adc_rst;
#[doc = "ADC_CONV\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_conv](adc_conv) module"]
pub type ADC_CONV = crate::Reg<u32, _ADC_CONV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CONV;
#[doc = "`read()` method returns [adc_conv::R](adc_conv::R) reader structure"]
impl crate::Readable for ADC_CONV {}
#[doc = "`write(|w| ..)` method takes [adc_conv::W](adc_conv::W) writer structure"]
impl crate::Writable for ADC_CONV {}
#[doc = "ADC_CONV"]
pub mod adc_conv;
#[doc = "ADC_LST0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_lst0](adc_lst0) module"]
pub type ADC_LST0 = crate::Reg<u32, _ADC_LST0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LST0;
#[doc = "`read()` method returns [adc_lst0::R](adc_lst0::R) reader structure"]
impl crate::Readable for ADC_LST0 {}
#[doc = "`write(|w| ..)` method takes [adc_lst0::W](adc_lst0::W) writer structure"]
impl crate::Writable for ADC_LST0 {}
#[doc = "ADC_LST0"]
pub mod adc_lst0;
#[doc = "ADC_LST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_lst1](adc_lst1) module"]
pub type ADC_LST1 = crate::Reg<u32, _ADC_LST1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LST1;
#[doc = "`read()` method returns [adc_lst1::R](adc_lst1::R) reader structure"]
impl crate::Readable for ADC_LST1 {}
#[doc = "`write(|w| ..)` method takes [adc_lst1::W](adc_lst1::W) writer structure"]
impl crate::Writable for ADC_LST1 {}
#[doc = "ADC_LST1"]
pub mod adc_lst1;
#[doc = "ADC_STR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str0](adc_str0) module"]
pub type ADC_STR0 = crate::Reg<u32, _ADC_STR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR0;
#[doc = "`read()` method returns [adc_str0::R](adc_str0::R) reader structure"]
impl crate::Readable for ADC_STR0 {}
#[doc = "`write(|w| ..)` method takes [adc_str0::W](adc_str0::W) writer structure"]
impl crate::Writable for ADC_STR0 {}
#[doc = "ADC_STR0"]
pub mod adc_str0;
#[doc = "ADC_STR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str1](adc_str1) module"]
pub type ADC_STR1 = crate::Reg<u32, _ADC_STR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR1;
#[doc = "`read()` method returns [adc_str1::R](adc_str1::R) reader structure"]
impl crate::Readable for ADC_STR1 {}
#[doc = "`write(|w| ..)` method takes [adc_str1::W](adc_str1::W) writer structure"]
impl crate::Writable for ADC_STR1 {}
#[doc = "ADC_STR1"]
pub mod adc_str1;
#[doc = "ADC_STR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str2](adc_str2) module"]
pub type ADC_STR2 = crate::Reg<u32, _ADC_STR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR2;
#[doc = "`read()` method returns [adc_str2::R](adc_str2::R) reader structure"]
impl crate::Readable for ADC_STR2 {}
#[doc = "`write(|w| ..)` method takes [adc_str2::W](adc_str2::W) writer structure"]
impl crate::Writable for ADC_STR2 {}
#[doc = "ADC_STR2"]
pub mod adc_str2;
#[doc = "ADC_STR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str3](adc_str3) module"]
pub type ADC_STR3 = crate::Reg<u32, _ADC_STR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR3;
#[doc = "`read()` method returns [adc_str3::R](adc_str3::R) reader structure"]
impl crate::Readable for ADC_STR3 {}
#[doc = "`write(|w| ..)` method takes [adc_str3::W](adc_str3::W) writer structure"]
impl crate::Writable for ADC_STR3 {}
#[doc = "ADC_STR3"]
pub mod adc_str3;
#[doc = "ADC_STR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str4](adc_str4) module"]
pub type ADC_STR4 = crate::Reg<u32, _ADC_STR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR4;
#[doc = "`read()` method returns [adc_str4::R](adc_str4::R) reader structure"]
impl crate::Readable for ADC_STR4 {}
#[doc = "`write(|w| ..)` method takes [adc_str4::W](adc_str4::W) writer structure"]
impl crate::Writable for ADC_STR4 {}
#[doc = "ADC_STR4"]
pub mod adc_str4;
#[doc = "ADC_STR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str5](adc_str5) module"]
pub type ADC_STR5 = crate::Reg<u32, _ADC_STR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR5;
#[doc = "`read()` method returns [adc_str5::R](adc_str5::R) reader structure"]
impl crate::Readable for ADC_STR5 {}
#[doc = "`write(|w| ..)` method takes [adc_str5::W](adc_str5::W) writer structure"]
impl crate::Writable for ADC_STR5 {}
#[doc = "ADC_STR5"]
pub mod adc_str5;
#[doc = "ADC_STR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str6](adc_str6) module"]
pub type ADC_STR6 = crate::Reg<u32, _ADC_STR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR6;
#[doc = "`read()` method returns [adc_str6::R](adc_str6::R) reader structure"]
impl crate::Readable for ADC_STR6 {}
#[doc = "`write(|w| ..)` method takes [adc_str6::W](adc_str6::W) writer structure"]
impl crate::Writable for ADC_STR6 {}
#[doc = "ADC_STR6"]
pub mod adc_str6;
#[doc = "ADC_STR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str7](adc_str7) module"]
pub type ADC_STR7 = crate::Reg<u32, _ADC_STR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR7;
#[doc = "`read()` method returns [adc_str7::R](adc_str7::R) reader structure"]
impl crate::Readable for ADC_STR7 {}
#[doc = "`write(|w| ..)` method takes [adc_str7::W](adc_str7::W) writer structure"]
impl crate::Writable for ADC_STR7 {}
#[doc = "ADC_STR7"]
pub mod adc_str7;
#[doc = "ADC_DR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr0](adc_dr0) module"]
pub type ADC_DR0 = crate::Reg<u32, _ADC_DR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR0;
#[doc = "`read()` method returns [adc_dr0::R](adc_dr0::R) reader structure"]
impl crate::Readable for ADC_DR0 {}
#[doc = "`write(|w| ..)` method takes [adc_dr0::W](adc_dr0::W) writer structure"]
impl crate::Writable for ADC_DR0 {}
#[doc = "ADC_DR0"]
pub mod adc_dr0;
#[doc = "ADC_DR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr1](adc_dr1) module"]
pub type ADC_DR1 = crate::Reg<u32, _ADC_DR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR1;
#[doc = "`read()` method returns [adc_dr1::R](adc_dr1::R) reader structure"]
impl crate::Readable for ADC_DR1 {}
#[doc = "`write(|w| ..)` method takes [adc_dr1::W](adc_dr1::W) writer structure"]
impl crate::Writable for ADC_DR1 {}
#[doc = "ADC_DR1"]
pub mod adc_dr1;
#[doc = "ADC_DR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr2](adc_dr2) module"]
pub type ADC_DR2 = crate::Reg<u32, _ADC_DR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR2;
#[doc = "`read()` method returns [adc_dr2::R](adc_dr2::R) reader structure"]
impl crate::Readable for ADC_DR2 {}
#[doc = "`write(|w| ..)` method takes [adc_dr2::W](adc_dr2::W) writer structure"]
impl crate::Writable for ADC_DR2 {}
#[doc = "ADC_DR2"]
pub mod adc_dr2;
#[doc = "ADC_DR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr3](adc_dr3) module"]
pub type ADC_DR3 = crate::Reg<u32, _ADC_DR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR3;
#[doc = "`read()` method returns [adc_dr3::R](adc_dr3::R) reader structure"]
impl crate::Readable for ADC_DR3 {}
#[doc = "`write(|w| ..)` method takes [adc_dr3::W](adc_dr3::W) writer structure"]
impl crate::Writable for ADC_DR3 {}
#[doc = "ADC_DR3"]
pub mod adc_dr3;
#[doc = "ADC_DR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr4](adc_dr4) module"]
pub type ADC_DR4 = crate::Reg<u32, _ADC_DR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR4;
#[doc = "`read()` method returns [adc_dr4::R](adc_dr4::R) reader structure"]
impl crate::Readable for ADC_DR4 {}
#[doc = "`write(|w| ..)` method takes [adc_dr4::W](adc_dr4::W) writer structure"]
impl crate::Writable for ADC_DR4 {}
#[doc = "ADC_DR4"]
pub mod adc_dr4;
#[doc = "ADC_DR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr5](adc_dr5) module"]
pub type ADC_DR5 = crate::Reg<u32, _ADC_DR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR5;
#[doc = "`read()` method returns [adc_dr5::R](adc_dr5::R) reader structure"]
impl crate::Readable for ADC_DR5 {}
#[doc = "`write(|w| ..)` method takes [adc_dr5::W](adc_dr5::W) writer structure"]
impl crate::Writable for ADC_DR5 {}
#[doc = "ADC_DR5"]
pub mod adc_dr5;
#[doc = "ADC_DR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr6](adc_dr6) module"]
pub type ADC_DR6 = crate::Reg<u32, _ADC_DR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR6;
#[doc = "`read()` method returns [adc_dr6::R](adc_dr6::R) reader structure"]
impl crate::Readable for ADC_DR6 {}
#[doc = "`write(|w| ..)` method takes [adc_dr6::W](adc_dr6::W) writer structure"]
impl crate::Writable for ADC_DR6 {}
#[doc = "ADC_DR6"]
pub mod adc_dr6;
#[doc = "ADC_DR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr7](adc_dr7) module"]
pub type ADC_DR7 = crate::Reg<u32, _ADC_DR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DR7;
#[doc = "`read()` method returns [adc_dr7::R](adc_dr7::R) reader structure"]
impl crate::Readable for ADC_DR7 {}
#[doc = "`write(|w| ..)` method takes [adc_dr7::W](adc_dr7::W) writer structure"]
impl crate::Writable for ADC_DR7 {}
#[doc = "ADC_DR7"]
pub mod adc_dr7;
#[doc = "ADC_TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_tcr](adc_tcr) module"]
pub type ADC_TCR = crate::Reg<u32, _ADC_TCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_TCR;
#[doc = "`read()` method returns [adc_tcr::R](adc_tcr::R) reader structure"]
impl crate::Readable for ADC_TCR {}
#[doc = "`write(|w| ..)` method takes [adc_tcr::W](adc_tcr::W) writer structure"]
impl crate::Writable for ADC_TCR {}
#[doc = "ADC_TCR"]
pub mod adc_tcr;
#[doc = "ADC_TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_tsr](adc_tsr) module"]
pub type ADC_TSR = crate::Reg<u32, _ADC_TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_TSR;
#[doc = "`read()` method returns [adc_tsr::R](adc_tsr::R) reader structure"]
impl crate::Readable for ADC_TSR {}
#[doc = "`write(|w| ..)` method takes [adc_tsr::W](adc_tsr::W) writer structure"]
impl crate::Writable for ADC_TSR {}
#[doc = "ADC_TSR"]
pub mod adc_tsr;
#[doc = "ADC_WCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_wcr](adc_wcr) module"]
pub type ADC_WCR = crate::Reg<u32, _ADC_WCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_WCR;
#[doc = "`read()` method returns [adc_wcr::R](adc_wcr::R) reader structure"]
impl crate::Readable for ADC_WCR {}
#[doc = "`write(|w| ..)` method takes [adc_wcr::W](adc_wcr::W) writer structure"]
impl crate::Writable for ADC_WCR {}
#[doc = "ADC_WCR"]
pub mod adc_wcr;
#[doc = "ADC_LTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ltr](adc_ltr) module"]
pub type ADC_LTR = crate::Reg<u32, _ADC_LTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_LTR;
#[doc = "`read()` method returns [adc_ltr::R](adc_ltr::R) reader structure"]
impl crate::Readable for ADC_LTR {}
#[doc = "`write(|w| ..)` method takes [adc_ltr::W](adc_ltr::W) writer structure"]
impl crate::Writable for ADC_LTR {}
#[doc = "ADC_LTR"]
pub mod adc_ltr;
#[doc = "ADC_UTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_utr](adc_utr) module"]
pub type ADC_UTR = crate::Reg<u32, _ADC_UTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_UTR;
#[doc = "`read()` method returns [adc_utr::R](adc_utr::R) reader structure"]
impl crate::Readable for ADC_UTR {}
#[doc = "`write(|w| ..)` method takes [adc_utr::W](adc_utr::W) writer structure"]
impl crate::Writable for ADC_UTR {}
#[doc = "ADC_UTR"]
pub mod adc_utr;
#[doc = "ADC_IMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_imr](adc_imr) module"]
pub type ADC_IMR = crate::Reg<u32, _ADC_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_IMR;
#[doc = "`read()` method returns [adc_imr::R](adc_imr::R) reader structure"]
impl crate::Readable for ADC_IMR {}
#[doc = "`write(|w| ..)` method takes [adc_imr::W](adc_imr::W) writer structure"]
impl crate::Writable for ADC_IMR {}
#[doc = "ADC_IMR"]
pub mod adc_imr;
#[doc = "ADC_IRAW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_iraw](adc_iraw) module"]
pub type ADC_IRAW = crate::Reg<u32, _ADC_IRAW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_IRAW;
#[doc = "`read()` method returns [adc_iraw::R](adc_iraw::R) reader structure"]
impl crate::Readable for ADC_IRAW {}
#[doc = "`write(|w| ..)` method takes [adc_iraw::W](adc_iraw::W) writer structure"]
impl crate::Writable for ADC_IRAW {}
#[doc = "ADC_IRAW"]
pub mod adc_iraw;
#[doc = "ADC_IMASK\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_imask](adc_imask) module"]
pub type ADC_IMASK = crate::Reg<u32, _ADC_IMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_IMASK;
#[doc = "`read()` method returns [adc_imask::R](adc_imask::R) reader structure"]
impl crate::Readable for ADC_IMASK {}
#[doc = "`write(|w| ..)` method takes [adc_imask::W](adc_imask::W) writer structure"]
impl crate::Writable for ADC_IMASK {}
#[doc = "ADC_IMASK"]
pub mod adc_imask;
#[doc = "ADC_ICLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_iclr](adc_iclr) module"]
pub type ADC_ICLR = crate::Reg<u32, _ADC_ICLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_ICLR;
#[doc = "`read()` method returns [adc_iclr::R](adc_iclr::R) reader structure"]
impl crate::Readable for ADC_ICLR {}
#[doc = "`write(|w| ..)` method takes [adc_iclr::W](adc_iclr::W) writer structure"]
impl crate::Writable for ADC_ICLR {}
#[doc = "ADC_ICLR"]
pub mod adc_iclr;
