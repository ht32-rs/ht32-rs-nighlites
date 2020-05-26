#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CSIF_ENR"]
    pub csif_enr: CSIF_ENR,
    #[doc = "0x04 - CSIF_CR"]
    pub csif_cr: CSIF_CR,
    #[doc = "0x08 - CSIF_IMGWH"]
    pub csif_imgwh: CSIF_IMGWH,
    #[doc = "0x0c - CSIF_WCR0"]
    pub csif_wcr0: CSIF_WCR0,
    #[doc = "0x10 - CSIF_WCR1"]
    pub csif_wcr1: CSIF_WCR1,
    #[doc = "0x14 - CSIF_SMP"]
    pub csif_smp: CSIF_SMP,
    #[doc = "0x18 - CSIF_SMPCOL"]
    pub csif_smpcol: CSIF_SMPCOL,
    #[doc = "0x1c - CSIF_SMPROW"]
    pub csif_smprow: CSIF_SMPROW,
    #[doc = "0x20 - CSIF_FIFO0"]
    pub csif_fifo0: CSIF_FIFO0,
    #[doc = "0x24 - CSIF_FIFO1"]
    pub csif_fifo1: CSIF_FIFO1,
    #[doc = "0x28 - CSIF_FIFO2"]
    pub csif_fifo2: CSIF_FIFO2,
    #[doc = "0x2c - CSIF_FIFO3"]
    pub csif_fifo3: CSIF_FIFO3,
    #[doc = "0x30 - CSIF_FIFO4"]
    pub csif_fifo4: CSIF_FIFO4,
    #[doc = "0x34 - CSIF_FIFO5"]
    pub csif_fifo5: CSIF_FIFO5,
    #[doc = "0x38 - CSIF_FIFO6"]
    pub csif_fifo6: CSIF_FIFO6,
    #[doc = "0x3c - CSIF_FIFO7"]
    pub csif_fifo7: CSIF_FIFO7,
    #[doc = "0x40 - CSIF_IER"]
    pub csif_ier: CSIF_IER,
    #[doc = "0x44 - CSIF_SR"]
    pub csif_sr: CSIF_SR,
}
#[doc = "CSIF_ENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_enr](csif_enr) module"]
pub type CSIF_ENR = crate::Reg<u32, _CSIF_ENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_ENR;
#[doc = "`read()` method returns [csif_enr::R](csif_enr::R) reader structure"]
impl crate::Readable for CSIF_ENR {}
#[doc = "`write(|w| ..)` method takes [csif_enr::W](csif_enr::W) writer structure"]
impl crate::Writable for CSIF_ENR {}
#[doc = "CSIF_ENR"]
pub mod csif_enr;
#[doc = "CSIF_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_cr](csif_cr) module"]
pub type CSIF_CR = crate::Reg<u32, _CSIF_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_CR;
#[doc = "`read()` method returns [csif_cr::R](csif_cr::R) reader structure"]
impl crate::Readable for CSIF_CR {}
#[doc = "`write(|w| ..)` method takes [csif_cr::W](csif_cr::W) writer structure"]
impl crate::Writable for CSIF_CR {}
#[doc = "CSIF_CR"]
pub mod csif_cr;
#[doc = "CSIF_IMGWH\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_imgwh](csif_imgwh) module"]
pub type CSIF_IMGWH = crate::Reg<u32, _CSIF_IMGWH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_IMGWH;
#[doc = "`read()` method returns [csif_imgwh::R](csif_imgwh::R) reader structure"]
impl crate::Readable for CSIF_IMGWH {}
#[doc = "`write(|w| ..)` method takes [csif_imgwh::W](csif_imgwh::W) writer structure"]
impl crate::Writable for CSIF_IMGWH {}
#[doc = "CSIF_IMGWH"]
pub mod csif_imgwh;
#[doc = "CSIF_WCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_wcr0](csif_wcr0) module"]
pub type CSIF_WCR0 = crate::Reg<u32, _CSIF_WCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_WCR0;
#[doc = "`read()` method returns [csif_wcr0::R](csif_wcr0::R) reader structure"]
impl crate::Readable for CSIF_WCR0 {}
#[doc = "`write(|w| ..)` method takes [csif_wcr0::W](csif_wcr0::W) writer structure"]
impl crate::Writable for CSIF_WCR0 {}
#[doc = "CSIF_WCR0"]
pub mod csif_wcr0;
#[doc = "CSIF_WCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_wcr1](csif_wcr1) module"]
pub type CSIF_WCR1 = crate::Reg<u32, _CSIF_WCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_WCR1;
#[doc = "`read()` method returns [csif_wcr1::R](csif_wcr1::R) reader structure"]
impl crate::Readable for CSIF_WCR1 {}
#[doc = "`write(|w| ..)` method takes [csif_wcr1::W](csif_wcr1::W) writer structure"]
impl crate::Writable for CSIF_WCR1 {}
#[doc = "CSIF_WCR1"]
pub mod csif_wcr1;
#[doc = "CSIF_SMP\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_smp](csif_smp) module"]
pub type CSIF_SMP = crate::Reg<u32, _CSIF_SMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_SMP;
#[doc = "`read()` method returns [csif_smp::R](csif_smp::R) reader structure"]
impl crate::Readable for CSIF_SMP {}
#[doc = "`write(|w| ..)` method takes [csif_smp::W](csif_smp::W) writer structure"]
impl crate::Writable for CSIF_SMP {}
#[doc = "CSIF_SMP"]
pub mod csif_smp;
#[doc = "CSIF_SMPCOL\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_smpcol](csif_smpcol) module"]
pub type CSIF_SMPCOL = crate::Reg<u32, _CSIF_SMPCOL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_SMPCOL;
#[doc = "`read()` method returns [csif_smpcol::R](csif_smpcol::R) reader structure"]
impl crate::Readable for CSIF_SMPCOL {}
#[doc = "`write(|w| ..)` method takes [csif_smpcol::W](csif_smpcol::W) writer structure"]
impl crate::Writable for CSIF_SMPCOL {}
#[doc = "CSIF_SMPCOL"]
pub mod csif_smpcol;
#[doc = "CSIF_SMPROW\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_smprow](csif_smprow) module"]
pub type CSIF_SMPROW = crate::Reg<u32, _CSIF_SMPROW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_SMPROW;
#[doc = "`read()` method returns [csif_smprow::R](csif_smprow::R) reader structure"]
impl crate::Readable for CSIF_SMPROW {}
#[doc = "`write(|w| ..)` method takes [csif_smprow::W](csif_smprow::W) writer structure"]
impl crate::Writable for CSIF_SMPROW {}
#[doc = "CSIF_SMPROW"]
pub mod csif_smprow;
#[doc = "CSIF_FIFO0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_fifo0](csif_fifo0) module"]
pub type CSIF_FIFO0 = crate::Reg<u32, _CSIF_FIFO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_FIFO0;
#[doc = "`read()` method returns [csif_fifo0::R](csif_fifo0::R) reader structure"]
impl crate::Readable for CSIF_FIFO0 {}
#[doc = "`write(|w| ..)` method takes [csif_fifo0::W](csif_fifo0::W) writer structure"]
impl crate::Writable for CSIF_FIFO0 {}
#[doc = "CSIF_FIFO0"]
pub mod csif_fifo0;
#[doc = "CSIF_FIFO1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_fifo1](csif_fifo1) module"]
pub type CSIF_FIFO1 = crate::Reg<u32, _CSIF_FIFO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_FIFO1;
#[doc = "`read()` method returns [csif_fifo1::R](csif_fifo1::R) reader structure"]
impl crate::Readable for CSIF_FIFO1 {}
#[doc = "`write(|w| ..)` method takes [csif_fifo1::W](csif_fifo1::W) writer structure"]
impl crate::Writable for CSIF_FIFO1 {}
#[doc = "CSIF_FIFO1"]
pub mod csif_fifo1;
#[doc = "CSIF_FIFO2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_fifo2](csif_fifo2) module"]
pub type CSIF_FIFO2 = crate::Reg<u32, _CSIF_FIFO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_FIFO2;
#[doc = "`read()` method returns [csif_fifo2::R](csif_fifo2::R) reader structure"]
impl crate::Readable for CSIF_FIFO2 {}
#[doc = "`write(|w| ..)` method takes [csif_fifo2::W](csif_fifo2::W) writer structure"]
impl crate::Writable for CSIF_FIFO2 {}
#[doc = "CSIF_FIFO2"]
pub mod csif_fifo2;
#[doc = "CSIF_FIFO3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_fifo3](csif_fifo3) module"]
pub type CSIF_FIFO3 = crate::Reg<u32, _CSIF_FIFO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_FIFO3;
#[doc = "`read()` method returns [csif_fifo3::R](csif_fifo3::R) reader structure"]
impl crate::Readable for CSIF_FIFO3 {}
#[doc = "`write(|w| ..)` method takes [csif_fifo3::W](csif_fifo3::W) writer structure"]
impl crate::Writable for CSIF_FIFO3 {}
#[doc = "CSIF_FIFO3"]
pub mod csif_fifo3;
#[doc = "CSIF_FIFO4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_fifo4](csif_fifo4) module"]
pub type CSIF_FIFO4 = crate::Reg<u32, _CSIF_FIFO4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_FIFO4;
#[doc = "`read()` method returns [csif_fifo4::R](csif_fifo4::R) reader structure"]
impl crate::Readable for CSIF_FIFO4 {}
#[doc = "`write(|w| ..)` method takes [csif_fifo4::W](csif_fifo4::W) writer structure"]
impl crate::Writable for CSIF_FIFO4 {}
#[doc = "CSIF_FIFO4"]
pub mod csif_fifo4;
#[doc = "CSIF_FIFO5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_fifo5](csif_fifo5) module"]
pub type CSIF_FIFO5 = crate::Reg<u32, _CSIF_FIFO5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_FIFO5;
#[doc = "`read()` method returns [csif_fifo5::R](csif_fifo5::R) reader structure"]
impl crate::Readable for CSIF_FIFO5 {}
#[doc = "`write(|w| ..)` method takes [csif_fifo5::W](csif_fifo5::W) writer structure"]
impl crate::Writable for CSIF_FIFO5 {}
#[doc = "CSIF_FIFO5"]
pub mod csif_fifo5;
#[doc = "CSIF_FIFO6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_fifo6](csif_fifo6) module"]
pub type CSIF_FIFO6 = crate::Reg<u32, _CSIF_FIFO6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_FIFO6;
#[doc = "`read()` method returns [csif_fifo6::R](csif_fifo6::R) reader structure"]
impl crate::Readable for CSIF_FIFO6 {}
#[doc = "`write(|w| ..)` method takes [csif_fifo6::W](csif_fifo6::W) writer structure"]
impl crate::Writable for CSIF_FIFO6 {}
#[doc = "CSIF_FIFO6"]
pub mod csif_fifo6;
#[doc = "CSIF_FIFO7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_fifo7](csif_fifo7) module"]
pub type CSIF_FIFO7 = crate::Reg<u32, _CSIF_FIFO7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_FIFO7;
#[doc = "`read()` method returns [csif_fifo7::R](csif_fifo7::R) reader structure"]
impl crate::Readable for CSIF_FIFO7 {}
#[doc = "`write(|w| ..)` method takes [csif_fifo7::W](csif_fifo7::W) writer structure"]
impl crate::Writable for CSIF_FIFO7 {}
#[doc = "CSIF_FIFO7"]
pub mod csif_fifo7;
#[doc = "CSIF_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_ier](csif_ier) module"]
pub type CSIF_IER = crate::Reg<u32, _CSIF_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_IER;
#[doc = "`read()` method returns [csif_ier::R](csif_ier::R) reader structure"]
impl crate::Readable for CSIF_IER {}
#[doc = "`write(|w| ..)` method takes [csif_ier::W](csif_ier::W) writer structure"]
impl crate::Writable for CSIF_IER {}
#[doc = "CSIF_IER"]
pub mod csif_ier;
#[doc = "CSIF_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_sr](csif_sr) module"]
pub type CSIF_SR = crate::Reg<u32, _CSIF_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSIF_SR;
#[doc = "`read()` method returns [csif_sr::R](csif_sr::R) reader structure"]
impl crate::Readable for CSIF_SR {}
#[doc = "`write(|w| ..)` method takes [csif_sr::W](csif_sr::W) writer structure"]
impl crate::Writable for CSIF_SR {}
#[doc = "CSIF_SR"]
pub mod csif_sr;
