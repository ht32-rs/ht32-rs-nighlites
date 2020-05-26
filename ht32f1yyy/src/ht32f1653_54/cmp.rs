#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CMP_CR0"]
    pub cmp_cr0: CMP_CR0,
    #[doc = "0x04 - CMP_VALR0"]
    pub cmp_valr0: CMP_VALR0,
    #[doc = "0x08 - CMP_IER0"]
    pub cmp_ier0: CMP_IER0,
    #[doc = "0x0c - CMP_TFR0"]
    pub cmp_tfr0: CMP_TFR0,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - CMP_CR1"]
    pub cmp_cr1: CMP_CR1,
    #[doc = "0x104 - CMP_VALR1"]
    pub cmp_valr1: CMP_VALR1,
    #[doc = "0x108 - CMP_IER1"]
    pub cmp_ier1: CMP_IER1,
    #[doc = "0x10c - CMP_TFR1"]
    pub cmp_tfr1: CMP_TFR1,
}
#[doc = "CMP_CR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_cr0](cmp_cr0) module"]
pub type CMP_CR0 = crate::Reg<u32, _CMP_CR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP_CR0;
#[doc = "`read()` method returns [cmp_cr0::R](cmp_cr0::R) reader structure"]
impl crate::Readable for CMP_CR0 {}
#[doc = "`write(|w| ..)` method takes [cmp_cr0::W](cmp_cr0::W) writer structure"]
impl crate::Writable for CMP_CR0 {}
#[doc = "CMP_CR0"]
pub mod cmp_cr0;
#[doc = "CMP_VALR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_valr0](cmp_valr0) module"]
pub type CMP_VALR0 = crate::Reg<u32, _CMP_VALR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP_VALR0;
#[doc = "`read()` method returns [cmp_valr0::R](cmp_valr0::R) reader structure"]
impl crate::Readable for CMP_VALR0 {}
#[doc = "`write(|w| ..)` method takes [cmp_valr0::W](cmp_valr0::W) writer structure"]
impl crate::Writable for CMP_VALR0 {}
#[doc = "CMP_VALR0"]
pub mod cmp_valr0;
#[doc = "CMP_IER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_ier0](cmp_ier0) module"]
pub type CMP_IER0 = crate::Reg<u32, _CMP_IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP_IER0;
#[doc = "`read()` method returns [cmp_ier0::R](cmp_ier0::R) reader structure"]
impl crate::Readable for CMP_IER0 {}
#[doc = "`write(|w| ..)` method takes [cmp_ier0::W](cmp_ier0::W) writer structure"]
impl crate::Writable for CMP_IER0 {}
#[doc = "CMP_IER0"]
pub mod cmp_ier0;
#[doc = "CMP_TFR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_tfr0](cmp_tfr0) module"]
pub type CMP_TFR0 = crate::Reg<u32, _CMP_TFR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP_TFR0;
#[doc = "`read()` method returns [cmp_tfr0::R](cmp_tfr0::R) reader structure"]
impl crate::Readable for CMP_TFR0 {}
#[doc = "`write(|w| ..)` method takes [cmp_tfr0::W](cmp_tfr0::W) writer structure"]
impl crate::Writable for CMP_TFR0 {}
#[doc = "CMP_TFR0"]
pub mod cmp_tfr0;
#[doc = "CMP_CR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_cr1](cmp_cr1) module"]
pub type CMP_CR1 = crate::Reg<u32, _CMP_CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP_CR1;
#[doc = "`read()` method returns [cmp_cr1::R](cmp_cr1::R) reader structure"]
impl crate::Readable for CMP_CR1 {}
#[doc = "`write(|w| ..)` method takes [cmp_cr1::W](cmp_cr1::W) writer structure"]
impl crate::Writable for CMP_CR1 {}
#[doc = "CMP_CR1"]
pub mod cmp_cr1;
#[doc = "CMP_VALR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_valr1](cmp_valr1) module"]
pub type CMP_VALR1 = crate::Reg<u32, _CMP_VALR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP_VALR1;
#[doc = "`read()` method returns [cmp_valr1::R](cmp_valr1::R) reader structure"]
impl crate::Readable for CMP_VALR1 {}
#[doc = "`write(|w| ..)` method takes [cmp_valr1::W](cmp_valr1::W) writer structure"]
impl crate::Writable for CMP_VALR1 {}
#[doc = "CMP_VALR1"]
pub mod cmp_valr1;
#[doc = "CMP_IER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_ier1](cmp_ier1) module"]
pub type CMP_IER1 = crate::Reg<u32, _CMP_IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP_IER1;
#[doc = "`read()` method returns [cmp_ier1::R](cmp_ier1::R) reader structure"]
impl crate::Readable for CMP_IER1 {}
#[doc = "`write(|w| ..)` method takes [cmp_ier1::W](cmp_ier1::W) writer structure"]
impl crate::Writable for CMP_IER1 {}
#[doc = "CMP_IER1"]
pub mod cmp_ier1;
#[doc = "CMP_TFR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_tfr1](cmp_tfr1) module"]
pub type CMP_TFR1 = crate::Reg<u32, _CMP_TFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP_TFR1;
#[doc = "`read()` method returns [cmp_tfr1::R](cmp_tfr1::R) reader structure"]
impl crate::Readable for CMP_TFR1 {}
#[doc = "`write(|w| ..)` method takes [cmp_tfr1::W](cmp_tfr1::W) writer structure"]
impl crate::Writable for CMP_TFR1 {}
#[doc = "CMP_TFR1"]
pub mod cmp_tfr1;
