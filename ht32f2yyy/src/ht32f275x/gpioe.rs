#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOE_DIRCR"]
    pub gpioe_dircr: GPIOE_DIRCR,
    #[doc = "0x04 - GPIOE_INER"]
    pub gpioe_iner: GPIOE_INER,
    #[doc = "0x08 - GPIOE_PUR"]
    pub gpioe_pur: GPIOE_PUR,
    #[doc = "0x0c - GPIOE_PDR"]
    pub gpioe_pdr: GPIOE_PDR,
    #[doc = "0x10 - GPIOE_ODR"]
    pub gpioe_odr: GPIOE_ODR,
    #[doc = "0x14 - GPIOE_DRVR"]
    pub gpioe_drvr: GPIOE_DRVR,
    #[doc = "0x18 - GPIOE_LOCKR"]
    pub gpioe_lockr: GPIOE_LOCKR,
    #[doc = "0x1c - GPIOE_DINR"]
    pub gpioe_dinr: GPIOE_DINR,
    #[doc = "0x20 - GPIOE_DOUTR"]
    pub gpioe_doutr: GPIOE_DOUTR,
    #[doc = "0x24 - GPIOE_SRR"]
    pub gpioe_srr: GPIOE_SRR,
    #[doc = "0x28 - GPIOE_RR"]
    pub gpioe_rr: GPIOE_RR,
}
#[doc = "GPIOE_DIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_dircr](gpioe_dircr) module"]
pub type GPIOE_DIRCR = crate::Reg<u32, _GPIOE_DIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_DIRCR;
#[doc = "`read()` method returns [gpioe_dircr::R](gpioe_dircr::R) reader structure"]
impl crate::Readable for GPIOE_DIRCR {}
#[doc = "`write(|w| ..)` method takes [gpioe_dircr::W](gpioe_dircr::W) writer structure"]
impl crate::Writable for GPIOE_DIRCR {}
#[doc = "GPIOE_DIRCR"]
pub mod gpioe_dircr;
#[doc = "GPIOE_INER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_iner](gpioe_iner) module"]
pub type GPIOE_INER = crate::Reg<u32, _GPIOE_INER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_INER;
#[doc = "`read()` method returns [gpioe_iner::R](gpioe_iner::R) reader structure"]
impl crate::Readable for GPIOE_INER {}
#[doc = "`write(|w| ..)` method takes [gpioe_iner::W](gpioe_iner::W) writer structure"]
impl crate::Writable for GPIOE_INER {}
#[doc = "GPIOE_INER"]
pub mod gpioe_iner;
#[doc = "GPIOE_PUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_pur](gpioe_pur) module"]
pub type GPIOE_PUR = crate::Reg<u32, _GPIOE_PUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_PUR;
#[doc = "`read()` method returns [gpioe_pur::R](gpioe_pur::R) reader structure"]
impl crate::Readable for GPIOE_PUR {}
#[doc = "`write(|w| ..)` method takes [gpioe_pur::W](gpioe_pur::W) writer structure"]
impl crate::Writable for GPIOE_PUR {}
#[doc = "GPIOE_PUR"]
pub mod gpioe_pur;
#[doc = "GPIOE_PDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_pdr](gpioe_pdr) module"]
pub type GPIOE_PDR = crate::Reg<u32, _GPIOE_PDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_PDR;
#[doc = "`read()` method returns [gpioe_pdr::R](gpioe_pdr::R) reader structure"]
impl crate::Readable for GPIOE_PDR {}
#[doc = "`write(|w| ..)` method takes [gpioe_pdr::W](gpioe_pdr::W) writer structure"]
impl crate::Writable for GPIOE_PDR {}
#[doc = "GPIOE_PDR"]
pub mod gpioe_pdr;
#[doc = "GPIOE_ODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_odr](gpioe_odr) module"]
pub type GPIOE_ODR = crate::Reg<u32, _GPIOE_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_ODR;
#[doc = "`read()` method returns [gpioe_odr::R](gpioe_odr::R) reader structure"]
impl crate::Readable for GPIOE_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioe_odr::W](gpioe_odr::W) writer structure"]
impl crate::Writable for GPIOE_ODR {}
#[doc = "GPIOE_ODR"]
pub mod gpioe_odr;
#[doc = "GPIOE_DRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_drvr](gpioe_drvr) module"]
pub type GPIOE_DRVR = crate::Reg<u32, _GPIOE_DRVR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_DRVR;
#[doc = "`read()` method returns [gpioe_drvr::R](gpioe_drvr::R) reader structure"]
impl crate::Readable for GPIOE_DRVR {}
#[doc = "`write(|w| ..)` method takes [gpioe_drvr::W](gpioe_drvr::W) writer structure"]
impl crate::Writable for GPIOE_DRVR {}
#[doc = "GPIOE_DRVR"]
pub mod gpioe_drvr;
#[doc = "GPIOE_LOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_lockr](gpioe_lockr) module"]
pub type GPIOE_LOCKR = crate::Reg<u32, _GPIOE_LOCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_LOCKR;
#[doc = "`read()` method returns [gpioe_lockr::R](gpioe_lockr::R) reader structure"]
impl crate::Readable for GPIOE_LOCKR {}
#[doc = "`write(|w| ..)` method takes [gpioe_lockr::W](gpioe_lockr::W) writer structure"]
impl crate::Writable for GPIOE_LOCKR {}
#[doc = "GPIOE_LOCKR"]
pub mod gpioe_lockr;
#[doc = "GPIOE_DINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_dinr](gpioe_dinr) module"]
pub type GPIOE_DINR = crate::Reg<u32, _GPIOE_DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_DINR;
#[doc = "`read()` method returns [gpioe_dinr::R](gpioe_dinr::R) reader structure"]
impl crate::Readable for GPIOE_DINR {}
#[doc = "`write(|w| ..)` method takes [gpioe_dinr::W](gpioe_dinr::W) writer structure"]
impl crate::Writable for GPIOE_DINR {}
#[doc = "GPIOE_DINR"]
pub mod gpioe_dinr;
#[doc = "GPIOE_DOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_doutr](gpioe_doutr) module"]
pub type GPIOE_DOUTR = crate::Reg<u32, _GPIOE_DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_DOUTR;
#[doc = "`read()` method returns [gpioe_doutr::R](gpioe_doutr::R) reader structure"]
impl crate::Readable for GPIOE_DOUTR {}
#[doc = "`write(|w| ..)` method takes [gpioe_doutr::W](gpioe_doutr::W) writer structure"]
impl crate::Writable for GPIOE_DOUTR {}
#[doc = "GPIOE_DOUTR"]
pub mod gpioe_doutr;
#[doc = "GPIOE_SRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_srr](gpioe_srr) module"]
pub type GPIOE_SRR = crate::Reg<u32, _GPIOE_SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_SRR;
#[doc = "`read()` method returns [gpioe_srr::R](gpioe_srr::R) reader structure"]
impl crate::Readable for GPIOE_SRR {}
#[doc = "`write(|w| ..)` method takes [gpioe_srr::W](gpioe_srr::W) writer structure"]
impl crate::Writable for GPIOE_SRR {}
#[doc = "GPIOE_SRR"]
pub mod gpioe_srr;
#[doc = "GPIOE_RR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioe_rr](gpioe_rr) module"]
pub type GPIOE_RR = crate::Reg<u32, _GPIOE_RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOE_RR;
#[doc = "`read()` method returns [gpioe_rr::R](gpioe_rr::R) reader structure"]
impl crate::Readable for GPIOE_RR {}
#[doc = "`write(|w| ..)` method takes [gpioe_rr::W](gpioe_rr::W) writer structure"]
impl crate::Writable for GPIOE_RR {}
#[doc = "GPIOE_RR"]
pub mod gpioe_rr;
