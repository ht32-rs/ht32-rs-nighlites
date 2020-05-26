#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOB_DIRCR"]
    pub gpiob_dircr: GPIOB_DIRCR,
    #[doc = "0x04 - GPIOB_INER"]
    pub gpiob_iner: GPIOB_INER,
    #[doc = "0x08 - GPIOB_PUR"]
    pub gpiob_pur: GPIOB_PUR,
    #[doc = "0x0c - GPIOB_PDR"]
    pub gpiob_pdr: GPIOB_PDR,
    #[doc = "0x10 - GPIOB_ODR"]
    pub gpiob_odr: GPIOB_ODR,
    #[doc = "0x14 - GPIOB_DRVR"]
    pub gpiob_drvr: GPIOB_DRVR,
    #[doc = "0x18 - GPIOB_LOCKR"]
    pub gpiob_lockr: GPIOB_LOCKR,
    #[doc = "0x1c - GPIOB_DINR"]
    pub gpiob_dinr: GPIOB_DINR,
    #[doc = "0x20 - GPIOB_DOUTR"]
    pub gpiob_doutr: GPIOB_DOUTR,
    #[doc = "0x24 - GPIOB_SRR"]
    pub gpiob_srr: GPIOB_SRR,
    #[doc = "0x28 - GPIOB_RR"]
    pub gpiob_rr: GPIOB_RR,
}
#[doc = "GPIOB_DIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_dircr](gpiob_dircr) module"]
pub type GPIOB_DIRCR = crate::Reg<u32, _GPIOB_DIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_DIRCR;
#[doc = "`read()` method returns [gpiob_dircr::R](gpiob_dircr::R) reader structure"]
impl crate::Readable for GPIOB_DIRCR {}
#[doc = "`write(|w| ..)` method takes [gpiob_dircr::W](gpiob_dircr::W) writer structure"]
impl crate::Writable for GPIOB_DIRCR {}
#[doc = "GPIOB_DIRCR"]
pub mod gpiob_dircr;
#[doc = "GPIOB_INER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_iner](gpiob_iner) module"]
pub type GPIOB_INER = crate::Reg<u32, _GPIOB_INER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_INER;
#[doc = "`read()` method returns [gpiob_iner::R](gpiob_iner::R) reader structure"]
impl crate::Readable for GPIOB_INER {}
#[doc = "`write(|w| ..)` method takes [gpiob_iner::W](gpiob_iner::W) writer structure"]
impl crate::Writable for GPIOB_INER {}
#[doc = "GPIOB_INER"]
pub mod gpiob_iner;
#[doc = "GPIOB_PUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_pur](gpiob_pur) module"]
pub type GPIOB_PUR = crate::Reg<u32, _GPIOB_PUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_PUR;
#[doc = "`read()` method returns [gpiob_pur::R](gpiob_pur::R) reader structure"]
impl crate::Readable for GPIOB_PUR {}
#[doc = "`write(|w| ..)` method takes [gpiob_pur::W](gpiob_pur::W) writer structure"]
impl crate::Writable for GPIOB_PUR {}
#[doc = "GPIOB_PUR"]
pub mod gpiob_pur;
#[doc = "GPIOB_PDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_pdr](gpiob_pdr) module"]
pub type GPIOB_PDR = crate::Reg<u32, _GPIOB_PDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_PDR;
#[doc = "`read()` method returns [gpiob_pdr::R](gpiob_pdr::R) reader structure"]
impl crate::Readable for GPIOB_PDR {}
#[doc = "`write(|w| ..)` method takes [gpiob_pdr::W](gpiob_pdr::W) writer structure"]
impl crate::Writable for GPIOB_PDR {}
#[doc = "GPIOB_PDR"]
pub mod gpiob_pdr;
#[doc = "GPIOB_ODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_odr](gpiob_odr) module"]
pub type GPIOB_ODR = crate::Reg<u32, _GPIOB_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_ODR;
#[doc = "`read()` method returns [gpiob_odr::R](gpiob_odr::R) reader structure"]
impl crate::Readable for GPIOB_ODR {}
#[doc = "`write(|w| ..)` method takes [gpiob_odr::W](gpiob_odr::W) writer structure"]
impl crate::Writable for GPIOB_ODR {}
#[doc = "GPIOB_ODR"]
pub mod gpiob_odr;
#[doc = "GPIOB_DRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_drvr](gpiob_drvr) module"]
pub type GPIOB_DRVR = crate::Reg<u32, _GPIOB_DRVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_DRVR;
#[doc = "`read()` method returns [gpiob_drvr::R](gpiob_drvr::R) reader structure"]
impl crate::Readable for GPIOB_DRVR {}
#[doc = "`write(|w| ..)` method takes [gpiob_drvr::W](gpiob_drvr::W) writer structure"]
impl crate::Writable for GPIOB_DRVR {}
#[doc = "GPIOB_DRVR"]
pub mod gpiob_drvr;
#[doc = "GPIOB_LOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_lockr](gpiob_lockr) module"]
pub type GPIOB_LOCKR = crate::Reg<u32, _GPIOB_LOCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_LOCKR;
#[doc = "`read()` method returns [gpiob_lockr::R](gpiob_lockr::R) reader structure"]
impl crate::Readable for GPIOB_LOCKR {}
#[doc = "`write(|w| ..)` method takes [gpiob_lockr::W](gpiob_lockr::W) writer structure"]
impl crate::Writable for GPIOB_LOCKR {}
#[doc = "GPIOB_LOCKR"]
pub mod gpiob_lockr;
#[doc = "GPIOB_DINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_dinr](gpiob_dinr) module"]
pub type GPIOB_DINR = crate::Reg<u32, _GPIOB_DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_DINR;
#[doc = "`read()` method returns [gpiob_dinr::R](gpiob_dinr::R) reader structure"]
impl crate::Readable for GPIOB_DINR {}
#[doc = "`write(|w| ..)` method takes [gpiob_dinr::W](gpiob_dinr::W) writer structure"]
impl crate::Writable for GPIOB_DINR {}
#[doc = "GPIOB_DINR"]
pub mod gpiob_dinr;
#[doc = "GPIOB_DOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_doutr](gpiob_doutr) module"]
pub type GPIOB_DOUTR = crate::Reg<u32, _GPIOB_DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_DOUTR;
#[doc = "`read()` method returns [gpiob_doutr::R](gpiob_doutr::R) reader structure"]
impl crate::Readable for GPIOB_DOUTR {}
#[doc = "`write(|w| ..)` method takes [gpiob_doutr::W](gpiob_doutr::W) writer structure"]
impl crate::Writable for GPIOB_DOUTR {}
#[doc = "GPIOB_DOUTR"]
pub mod gpiob_doutr;
#[doc = "GPIOB_SRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_srr](gpiob_srr) module"]
pub type GPIOB_SRR = crate::Reg<u32, _GPIOB_SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_SRR;
#[doc = "`read()` method returns [gpiob_srr::R](gpiob_srr::R) reader structure"]
impl crate::Readable for GPIOB_SRR {}
#[doc = "`write(|w| ..)` method takes [gpiob_srr::W](gpiob_srr::W) writer structure"]
impl crate::Writable for GPIOB_SRR {}
#[doc = "GPIOB_SRR"]
pub mod gpiob_srr;
#[doc = "GPIOB_RR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiob_rr](gpiob_rr) module"]
pub type GPIOB_RR = crate::Reg<u32, _GPIOB_RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOB_RR;
#[doc = "`read()` method returns [gpiob_rr::R](gpiob_rr::R) reader structure"]
impl crate::Readable for GPIOB_RR {}
#[doc = "`write(|w| ..)` method takes [gpiob_rr::W](gpiob_rr::W) writer structure"]
impl crate::Writable for GPIOB_RR {}
#[doc = "GPIOB_RR"]
pub mod gpiob_rr;
