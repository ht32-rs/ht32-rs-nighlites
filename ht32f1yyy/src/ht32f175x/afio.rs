#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AFIO_ESSR0"]
    pub afio_essr0: AFIO_ESSR0,
    #[doc = "0x04 - AFIO_ESSR1"]
    pub afio_essr1: AFIO_ESSR1,
    #[doc = "0x08 - AFIO_GPACFGR"]
    pub afio_gpacfgr: AFIO_GPACFGR,
    #[doc = "0x0c - AFIO_GPBCFGR"]
    pub afio_gpbcfgr: AFIO_GPBCFGR,
    #[doc = "0x10 - AFIO_GPCCFGR"]
    pub afio_gpccfgr: AFIO_GPCCFGR,
    #[doc = "0x14 - AFIO_GPDCFGR"]
    pub afio_gpdcfgr: AFIO_GPDCFGR,
    #[doc = "0x18 - AFIO_GPECFGR"]
    pub afio_gpecfgr: AFIO_GPECFGR,
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
#[doc = "AFIO_GPACFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpacfgr](afio_gpacfgr) module"]
pub type AFIO_GPACFGR = crate::Reg<u32, _AFIO_GPACFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPACFGR;
#[doc = "`read()` method returns [afio_gpacfgr::R](afio_gpacfgr::R) reader structure"]
impl crate::Readable for AFIO_GPACFGR {}
#[doc = "`write(|w| ..)` method takes [afio_gpacfgr::W](afio_gpacfgr::W) writer structure"]
impl crate::Writable for AFIO_GPACFGR {}
#[doc = "AFIO_GPACFGR"]
pub mod afio_gpacfgr;
#[doc = "AFIO_GPBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpbcfgr](afio_gpbcfgr) module"]
pub type AFIO_GPBCFGR = crate::Reg<u32, _AFIO_GPBCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPBCFGR;
#[doc = "`read()` method returns [afio_gpbcfgr::R](afio_gpbcfgr::R) reader structure"]
impl crate::Readable for AFIO_GPBCFGR {}
#[doc = "`write(|w| ..)` method takes [afio_gpbcfgr::W](afio_gpbcfgr::W) writer structure"]
impl crate::Writable for AFIO_GPBCFGR {}
#[doc = "AFIO_GPBCFGR"]
pub mod afio_gpbcfgr;
#[doc = "AFIO_GPCCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpccfgr](afio_gpccfgr) module"]
pub type AFIO_GPCCFGR = crate::Reg<u32, _AFIO_GPCCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPCCFGR;
#[doc = "`read()` method returns [afio_gpccfgr::R](afio_gpccfgr::R) reader structure"]
impl crate::Readable for AFIO_GPCCFGR {}
#[doc = "`write(|w| ..)` method takes [afio_gpccfgr::W](afio_gpccfgr::W) writer structure"]
impl crate::Writable for AFIO_GPCCFGR {}
#[doc = "AFIO_GPCCFGR"]
pub mod afio_gpccfgr;
#[doc = "AFIO_GPDCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpdcfgr](afio_gpdcfgr) module"]
pub type AFIO_GPDCFGR = crate::Reg<u32, _AFIO_GPDCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPDCFGR;
#[doc = "`read()` method returns [afio_gpdcfgr::R](afio_gpdcfgr::R) reader structure"]
impl crate::Readable for AFIO_GPDCFGR {}
#[doc = "`write(|w| ..)` method takes [afio_gpdcfgr::W](afio_gpdcfgr::W) writer structure"]
impl crate::Writable for AFIO_GPDCFGR {}
#[doc = "AFIO_GPDCFGR"]
pub mod afio_gpdcfgr;
#[doc = "AFIO_GPECFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afio_gpecfgr](afio_gpecfgr) module"]
pub type AFIO_GPECFGR = crate::Reg<u32, _AFIO_GPECFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFIO_GPECFGR;
#[doc = "`read()` method returns [afio_gpecfgr::R](afio_gpecfgr::R) reader structure"]
impl crate::Readable for AFIO_GPECFGR {}
#[doc = "`write(|w| ..)` method takes [afio_gpecfgr::W](afio_gpecfgr::W) writer structure"]
impl crate::Writable for AFIO_GPECFGR {}
#[doc = "AFIO_GPECFGR"]
pub mod afio_gpecfgr;
