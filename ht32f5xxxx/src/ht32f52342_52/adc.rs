#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC_CR"]
    pub adc_cr: ADC_CR,
    #[doc = "0x04 - ADC_LST0"]
    pub adc_lst0: ADC_LST0,
    #[doc = "0x08 - ADC_LST1"]
    pub adc_lst1: ADC_LST1,
    _reserved3: [u8; 20usize],
    #[doc = "0x20 - ADC_STR"]
    pub adc_str: ADC_STR,
    _reserved4: [u8; 12usize],
    #[doc = "0x30 - ADC_DR0"]
    pub adc_dr0: ADC_DR0,
    #[doc = "0x34 - ADC_DR1"]
    pub adc_dr1: ADC_DR1,
    #[doc = "0x38 - ADC_DR2"]
    pub adc_dr2: ADC_DR2,
    #[doc = "0x3c - ADC_DR3"]
    pub adc_dr3: ADC_DR3,
    #[doc = "0x40 - ADC_DR4"]
    pub adc_dr4: ADC_DR4,
    #[doc = "0x44 - ADC_DR5"]
    pub adc_dr5: ADC_DR5,
    #[doc = "0x48 - ADC_DR6"]
    pub adc_dr6: ADC_DR6,
    #[doc = "0x4c - ADC_DR7"]
    pub adc_dr7: ADC_DR7,
    _reserved12: [u8; 32usize],
    #[doc = "0x70 - ADC_TCR"]
    pub adc_tcr: ADC_TCR,
    #[doc = "0x74 - ADC_TSR"]
    pub adc_tsr: ADC_TSR,
    #[doc = "0x78 - ADC_WCR"]
    pub adc_wcr: ADC_WCR,
    #[doc = "0x7c - ADC_TR"]
    pub adc_tr: ADC_TR,
    #[doc = "0x80 - ADC_IMR"]
    pub adc_ier: ADC_IER,
    #[doc = "0x84 - ADC_IRAW"]
    pub adc_iraw: ADC_IRAW,
    #[doc = "0x88 - ADC_ISR"]
    pub adc_isr: ADC_ISR,
    #[doc = "0x8c - ADC_ICLR"]
    pub adc_iclr: ADC_ICLR,
    #[doc = "0x90 - ADC_DMAR"]
    pub adc_dmar: ADC_DMAR,
}
#[doc = "ADC_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_cr](adc_cr) module"]
pub type ADC_CR = crate::Reg<u32, _ADC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_CR;
#[doc = "`read()` method returns [adc_cr::R](adc_cr::R) reader structure"]
impl crate::Readable for ADC_CR {}
#[doc = "`write(|w| ..)` method takes [adc_cr::W](adc_cr::W) writer structure"]
impl crate::Writable for ADC_CR {}
#[doc = "ADC_CR"]
pub mod adc_cr;
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
#[doc = "ADC_STR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str](adc_str) module"]
pub type ADC_STR = crate::Reg<u32, _ADC_STR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_STR;
#[doc = "`read()` method returns [adc_str::R](adc_str::R) reader structure"]
impl crate::Readable for ADC_STR {}
#[doc = "`write(|w| ..)` method takes [adc_str::W](adc_str::W) writer structure"]
impl crate::Writable for ADC_STR {}
#[doc = "ADC_STR"]
pub mod adc_str;
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
#[doc = "ADC_TR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_tr](adc_tr) module"]
pub type ADC_TR = crate::Reg<u32, _ADC_TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_TR;
#[doc = "`read()` method returns [adc_tr::R](adc_tr::R) reader structure"]
impl crate::Readable for ADC_TR {}
#[doc = "`write(|w| ..)` method takes [adc_tr::W](adc_tr::W) writer structure"]
impl crate::Writable for ADC_TR {}
#[doc = "ADC_TR"]
pub mod adc_tr;
#[doc = "ADC_IMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ier](adc_ier) module"]
pub type ADC_IER = crate::Reg<u32, _ADC_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_IER;
#[doc = "`read()` method returns [adc_ier::R](adc_ier::R) reader structure"]
impl crate::Readable for ADC_IER {}
#[doc = "`write(|w| ..)` method takes [adc_ier::W](adc_ier::W) writer structure"]
impl crate::Writable for ADC_IER {}
#[doc = "ADC_IMR"]
pub mod adc_ier;
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
#[doc = "ADC_ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_isr](adc_isr) module"]
pub type ADC_ISR = crate::Reg<u32, _ADC_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_ISR;
#[doc = "`read()` method returns [adc_isr::R](adc_isr::R) reader structure"]
impl crate::Readable for ADC_ISR {}
#[doc = "`write(|w| ..)` method takes [adc_isr::W](adc_isr::W) writer structure"]
impl crate::Writable for ADC_ISR {}
#[doc = "ADC_ISR"]
pub mod adc_isr;
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
#[doc = "ADC_DMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dmar](adc_dmar) module"]
pub type ADC_DMAR = crate::Reg<u32, _ADC_DMAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC_DMAR;
#[doc = "`read()` method returns [adc_dmar::R](adc_dmar::R) reader structure"]
impl crate::Readable for ADC_DMAR {}
#[doc = "`write(|w| ..)` method takes [adc_dmar::W](adc_dmar::W) writer structure"]
impl crate::Writable for ADC_DMAR {}
#[doc = "ADC_DMAR"]
pub mod adc_dmar;
