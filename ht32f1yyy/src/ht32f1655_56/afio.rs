#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFIO_ESSR0"]
    pub afio_essr0: AFIO_ESSR0,
    #[doc = "0x04 - AFIO_ESSR1"]
    pub afio_essr1: AFIO_ESSR1,
    _reserved2: [u8; 24usize],
    #[doc = "0x20 - AFIO_GPACFGLR"]
    pub afio_gpacfglr: AFIO_GPACFGLR,
    #[doc = "0x24 - AFIO_GPACFGHR"]
    pub afio_gpacfghr: AFIO_GPACFGHR,
    #[doc = "0x28 - AFIO_GPBCFGLR"]
    pub afio_gpbcfglr: AFIO_GPBCFGLR,
    #[doc = "0x2c - AFIO_GPBCFGHR"]
    pub afio_gpbcfghr: AFIO_GPBCFGHR,
    #[doc = "0x30 - AFIO_GPCCFGLR"]
    pub afio_gpccfglr: AFIO_GPCCFGLR,
    #[doc = "0x34 - AFIO_GPCCFGHR"]
    pub afio_gpccfghr: AFIO_GPCCFGHR,
    #[doc = "0x38 - AFIO_GPDCFGLR"]
    pub afio_gpdcfglr: AFIO_GPDCFGLR,
    #[doc = "0x3c - AFIO_GPDCFGHR"]
    pub afio_gpdcfghr: AFIO_GPDCFGHR,
    #[doc = "0x40 - AFIO_GPECFGLR"]
    pub afio_gpecfglr: AFIO_GPECFGLR,
    #[doc = "0x44 - AFIO_GPECFGHR"]
    pub afio_gpecfghr: AFIO_GPECFGHR,
}
#[doc = "AFIO_ESSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_essr0](afio_essr0) module"]
pub type AFIO_ESSR0 = crate::Reg<u32, _AFIO_ESSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_ESSR0;
#[doc = "`read()` method returns [afio_essr0::R](afio_essr0::R) reader structure"]
impl crate::Readable for AFIO_ESSR0 {}
#[doc = "`write(|w| ..)` method takes [afio_essr0::W](afio_essr0::W) writer structure"]
impl crate::Writable for AFIO_ESSR0 {}
#[doc = "AFIO_ESSR0"]
pub mod afio_essr0;
#[doc = "AFIO_ESSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_essr1](afio_essr1) module"]
pub type AFIO_ESSR1 = crate::Reg<u32, _AFIO_ESSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_ESSR1;
#[doc = "`read()` method returns [afio_essr1::R](afio_essr1::R) reader structure"]
impl crate::Readable for AFIO_ESSR1 {}
#[doc = "`write(|w| ..)` method takes [afio_essr1::W](afio_essr1::W) writer structure"]
impl crate::Writable for AFIO_ESSR1 {}
#[doc = "AFIO_ESSR1"]
pub mod afio_essr1;
#[doc = "AFIO_GPACFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpacfglr](afio_gpacfglr) module"]
pub type AFIO_GPACFGLR = crate::Reg<u32, _AFIO_GPACFGLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPACFGLR;
#[doc = "`read()` method returns [afio_gpacfglr::R](afio_gpacfglr::R) reader structure"]
impl crate::Readable for AFIO_GPACFGLR {}
#[doc = "`write(|w| ..)` method takes [afio_gpacfglr::W](afio_gpacfglr::W) writer structure"]
impl crate::Writable for AFIO_GPACFGLR {}
#[doc = "AFIO_GPACFGLR"]
pub mod afio_gpacfglr;
#[doc = "AFIO_GPACFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpacfghr](afio_gpacfghr) module"]
pub type AFIO_GPACFGHR = crate::Reg<u32, _AFIO_GPACFGHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPACFGHR;
#[doc = "`read()` method returns [afio_gpacfghr::R](afio_gpacfghr::R) reader structure"]
impl crate::Readable for AFIO_GPACFGHR {}
#[doc = "`write(|w| ..)` method takes [afio_gpacfghr::W](afio_gpacfghr::W) writer structure"]
impl crate::Writable for AFIO_GPACFGHR {}
#[doc = "AFIO_GPACFGHR"]
pub mod afio_gpacfghr;
#[doc = "AFIO_GPBCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpbcfglr](afio_gpbcfglr) module"]
pub type AFIO_GPBCFGLR = crate::Reg<u32, _AFIO_GPBCFGLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPBCFGLR;
#[doc = "`read()` method returns [afio_gpbcfglr::R](afio_gpbcfglr::R) reader structure"]
impl crate::Readable for AFIO_GPBCFGLR {}
#[doc = "`write(|w| ..)` method takes [afio_gpbcfglr::W](afio_gpbcfglr::W) writer structure"]
impl crate::Writable for AFIO_GPBCFGLR {}
#[doc = "AFIO_GPBCFGLR"]
pub mod afio_gpbcfglr;
#[doc = "AFIO_GPBCFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpbcfghr](afio_gpbcfghr) module"]
pub type AFIO_GPBCFGHR = crate::Reg<u32, _AFIO_GPBCFGHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPBCFGHR;
#[doc = "`read()` method returns [afio_gpbcfghr::R](afio_gpbcfghr::R) reader structure"]
impl crate::Readable for AFIO_GPBCFGHR {}
#[doc = "`write(|w| ..)` method takes [afio_gpbcfghr::W](afio_gpbcfghr::W) writer structure"]
impl crate::Writable for AFIO_GPBCFGHR {}
#[doc = "AFIO_GPBCFGHR"]
pub mod afio_gpbcfghr;
#[doc = "AFIO_GPCCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpccfglr](afio_gpccfglr) module"]
pub type AFIO_GPCCFGLR = crate::Reg<u32, _AFIO_GPCCFGLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPCCFGLR;
#[doc = "`read()` method returns [afio_gpccfglr::R](afio_gpccfglr::R) reader structure"]
impl crate::Readable for AFIO_GPCCFGLR {}
#[doc = "`write(|w| ..)` method takes [afio_gpccfglr::W](afio_gpccfglr::W) writer structure"]
impl crate::Writable for AFIO_GPCCFGLR {}
#[doc = "AFIO_GPCCFGLR"]
pub mod afio_gpccfglr;
#[doc = "AFIO_GPCCFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpccfghr](afio_gpccfghr) module"]
pub type AFIO_GPCCFGHR = crate::Reg<u32, _AFIO_GPCCFGHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPCCFGHR;
#[doc = "`read()` method returns [afio_gpccfghr::R](afio_gpccfghr::R) reader structure"]
impl crate::Readable for AFIO_GPCCFGHR {}
#[doc = "`write(|w| ..)` method takes [afio_gpccfghr::W](afio_gpccfghr::W) writer structure"]
impl crate::Writable for AFIO_GPCCFGHR {}
#[doc = "AFIO_GPCCFGHR"]
pub mod afio_gpccfghr;
#[doc = "AFIO_GPDCFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpdcfglr](afio_gpdcfglr) module"]
pub type AFIO_GPDCFGLR = crate::Reg<u32, _AFIO_GPDCFGLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPDCFGLR;
#[doc = "`read()` method returns [afio_gpdcfglr::R](afio_gpdcfglr::R) reader structure"]
impl crate::Readable for AFIO_GPDCFGLR {}
#[doc = "`write(|w| ..)` method takes [afio_gpdcfglr::W](afio_gpdcfglr::W) writer structure"]
impl crate::Writable for AFIO_GPDCFGLR {}
#[doc = "AFIO_GPDCFGLR"]
pub mod afio_gpdcfglr;
#[doc = "AFIO_GPDCFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpdcfghr](afio_gpdcfghr) module"]
pub type AFIO_GPDCFGHR = crate::Reg<u32, _AFIO_GPDCFGHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPDCFGHR;
#[doc = "`read()` method returns [afio_gpdcfghr::R](afio_gpdcfghr::R) reader structure"]
impl crate::Readable for AFIO_GPDCFGHR {}
#[doc = "`write(|w| ..)` method takes [afio_gpdcfghr::W](afio_gpdcfghr::W) writer structure"]
impl crate::Writable for AFIO_GPDCFGHR {}
#[doc = "AFIO_GPDCFGHR"]
pub mod afio_gpdcfghr;
#[doc = "AFIO_GPECFGLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpecfglr](afio_gpecfglr) module"]
pub type AFIO_GPECFGLR = crate::Reg<u32, _AFIO_GPECFGLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPECFGLR;
#[doc = "`read()` method returns [afio_gpecfglr::R](afio_gpecfglr::R) reader structure"]
impl crate::Readable for AFIO_GPECFGLR {}
#[doc = "`write(|w| ..)` method takes [afio_gpecfglr::W](afio_gpecfglr::W) writer structure"]
impl crate::Writable for AFIO_GPECFGLR {}
#[doc = "AFIO_GPECFGLR"]
pub mod afio_gpecfglr;
#[doc = "AFIO_GPECFGHR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpecfghr](afio_gpecfghr) module"]
pub type AFIO_GPECFGHR = crate::Reg<u32, _AFIO_GPECFGHR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPECFGHR;
#[doc = "`read()` method returns [afio_gpecfghr::R](afio_gpecfghr::R) reader structure"]
impl crate::Readable for AFIO_GPECFGHR {}
#[doc = "`write(|w| ..)` method takes [afio_gpecfghr::W](afio_gpecfghr::W) writer structure"]
impl crate::Writable for AFIO_GPECFGHR {}
#[doc = "AFIO_GPECFGHR"]
pub mod afio_gpecfghr;
