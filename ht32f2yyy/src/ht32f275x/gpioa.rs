#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOA_DIRCR"]
    pub gpioa_dircr: GPIOA_DIRCR,
    #[doc = "0x04 - GPIOA_INER"]
    pub gpioa_iner: GPIOA_INER,
    #[doc = "0x08 - GPIOA_PUR"]
    pub gpioa_pur: GPIOA_PUR,
    #[doc = "0x0c - GPIOA_PDR"]
    pub gpioa_pdr: GPIOA_PDR,
    #[doc = "0x10 - GPIOA_ODR"]
    pub gpioa_odr: GPIOA_ODR,
    #[doc = "0x14 - GPIOA_DRVR"]
    pub gpioa_drvr: GPIOA_DRVR,
    #[doc = "0x18 - GPIOA_LOCKR"]
    pub gpioa_lockr: GPIOA_LOCKR,
    #[doc = "0x1c - GPIOA_DINR"]
    pub gpioa_dinr: GPIOA_DINR,
    #[doc = "0x20 - GPIOA_DOUTR"]
    pub gpioa_doutr: GPIOA_DOUTR,
    #[doc = "0x24 - GPIOA_SRR"]
    pub gpioa_srr: GPIOA_SRR,
    #[doc = "0x28 - GPIOA_RR"]
    pub gpioa_rr: GPIOA_RR,
}
#[doc = "GPIOA_DIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_dircr](gpioa_dircr) module"]
pub type GPIOA_DIRCR = crate::Reg<u32, _GPIOA_DIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_DIRCR;
#[doc = "`read()` method returns [gpioa_dircr::R](gpioa_dircr::R) reader structure"]
impl crate::Readable for GPIOA_DIRCR {}
#[doc = "`write(|w| ..)` method takes [gpioa_dircr::W](gpioa_dircr::W) writer structure"]
impl crate::Writable for GPIOA_DIRCR {}
#[doc = "GPIOA_DIRCR"]
pub mod gpioa_dircr;
#[doc = "GPIOA_INER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_iner](gpioa_iner) module"]
pub type GPIOA_INER = crate::Reg<u32, _GPIOA_INER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_INER;
#[doc = "`read()` method returns [gpioa_iner::R](gpioa_iner::R) reader structure"]
impl crate::Readable for GPIOA_INER {}
#[doc = "`write(|w| ..)` method takes [gpioa_iner::W](gpioa_iner::W) writer structure"]
impl crate::Writable for GPIOA_INER {}
#[doc = "GPIOA_INER"]
pub mod gpioa_iner;
#[doc = "GPIOA_PUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_pur](gpioa_pur) module"]
pub type GPIOA_PUR = crate::Reg<u32, _GPIOA_PUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_PUR;
#[doc = "`read()` method returns [gpioa_pur::R](gpioa_pur::R) reader structure"]
impl crate::Readable for GPIOA_PUR {}
#[doc = "`write(|w| ..)` method takes [gpioa_pur::W](gpioa_pur::W) writer structure"]
impl crate::Writable for GPIOA_PUR {}
#[doc = "GPIOA_PUR"]
pub mod gpioa_pur;
#[doc = "GPIOA_PDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_pdr](gpioa_pdr) module"]
pub type GPIOA_PDR = crate::Reg<u32, _GPIOA_PDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_PDR;
#[doc = "`read()` method returns [gpioa_pdr::R](gpioa_pdr::R) reader structure"]
impl crate::Readable for GPIOA_PDR {}
#[doc = "`write(|w| ..)` method takes [gpioa_pdr::W](gpioa_pdr::W) writer structure"]
impl crate::Writable for GPIOA_PDR {}
#[doc = "GPIOA_PDR"]
pub mod gpioa_pdr;
#[doc = "GPIOA_ODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_odr](gpioa_odr) module"]
pub type GPIOA_ODR = crate::Reg<u32, _GPIOA_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_ODR;
#[doc = "`read()` method returns [gpioa_odr::R](gpioa_odr::R) reader structure"]
impl crate::Readable for GPIOA_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioa_odr::W](gpioa_odr::W) writer structure"]
impl crate::Writable for GPIOA_ODR {}
#[doc = "GPIOA_ODR"]
pub mod gpioa_odr;
#[doc = "GPIOA_DRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_drvr](gpioa_drvr) module"]
pub type GPIOA_DRVR = crate::Reg<u32, _GPIOA_DRVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_DRVR;
#[doc = "`read()` method returns [gpioa_drvr::R](gpioa_drvr::R) reader structure"]
impl crate::Readable for GPIOA_DRVR {}
#[doc = "`write(|w| ..)` method takes [gpioa_drvr::W](gpioa_drvr::W) writer structure"]
impl crate::Writable for GPIOA_DRVR {}
#[doc = "GPIOA_DRVR"]
pub mod gpioa_drvr;
#[doc = "GPIOA_LOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_lockr](gpioa_lockr) module"]
pub type GPIOA_LOCKR = crate::Reg<u32, _GPIOA_LOCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_LOCKR;
#[doc = "`read()` method returns [gpioa_lockr::R](gpioa_lockr::R) reader structure"]
impl crate::Readable for GPIOA_LOCKR {}
#[doc = "`write(|w| ..)` method takes [gpioa_lockr::W](gpioa_lockr::W) writer structure"]
impl crate::Writable for GPIOA_LOCKR {}
#[doc = "GPIOA_LOCKR"]
pub mod gpioa_lockr;
#[doc = "GPIOA_DINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_dinr](gpioa_dinr) module"]
pub type GPIOA_DINR = crate::Reg<u32, _GPIOA_DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_DINR;
#[doc = "`read()` method returns [gpioa_dinr::R](gpioa_dinr::R) reader structure"]
impl crate::Readable for GPIOA_DINR {}
#[doc = "`write(|w| ..)` method takes [gpioa_dinr::W](gpioa_dinr::W) writer structure"]
impl crate::Writable for GPIOA_DINR {}
#[doc = "GPIOA_DINR"]
pub mod gpioa_dinr;
#[doc = "GPIOA_DOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_doutr](gpioa_doutr) module"]
pub type GPIOA_DOUTR = crate::Reg<u32, _GPIOA_DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_DOUTR;
#[doc = "`read()` method returns [gpioa_doutr::R](gpioa_doutr::R) reader structure"]
impl crate::Readable for GPIOA_DOUTR {}
#[doc = "`write(|w| ..)` method takes [gpioa_doutr::W](gpioa_doutr::W) writer structure"]
impl crate::Writable for GPIOA_DOUTR {}
#[doc = "GPIOA_DOUTR"]
pub mod gpioa_doutr;
#[doc = "GPIOA_SRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_srr](gpioa_srr) module"]
pub type GPIOA_SRR = crate::Reg<u32, _GPIOA_SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_SRR;
#[doc = "`read()` method returns [gpioa_srr::R](gpioa_srr::R) reader structure"]
impl crate::Readable for GPIOA_SRR {}
#[doc = "`write(|w| ..)` method takes [gpioa_srr::W](gpioa_srr::W) writer structure"]
impl crate::Writable for GPIOA_SRR {}
#[doc = "GPIOA_SRR"]
pub mod gpioa_srr;
#[doc = "GPIOA_RR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioa_rr](gpioa_rr) module"]
pub type GPIOA_RR = crate::Reg<u32, _GPIOA_RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOA_RR;
#[doc = "`read()` method returns [gpioa_rr::R](gpioa_rr::R) reader structure"]
impl crate::Readable for GPIOA_RR {}
#[doc = "`write(|w| ..)` method takes [gpioa_rr::W](gpioa_rr::W) writer structure"]
impl crate::Writable for GPIOA_RR {}
#[doc = "GPIOA_RR"]
pub mod gpioa_rr;
