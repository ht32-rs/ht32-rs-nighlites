#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOC_DIRCR"]
    pub gpioc_dircr: GPIOC_DIRCR,
    #[doc = "0x04 - GPIOC_INER"]
    pub gpioc_iner: GPIOC_INER,
    #[doc = "0x08 - GPIOC_PUR"]
    pub gpioc_pur: GPIOC_PUR,
    #[doc = "0x0c - GPIOC_PDR"]
    pub gpioc_pdr: GPIOC_PDR,
    #[doc = "0x10 - GPIOC_ODR"]
    pub gpioc_odr: GPIOC_ODR,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - GPIOC_LOCKR"]
    pub gpioc_lockr: GPIOC_LOCKR,
    #[doc = "0x1c - GPIOC_DINR"]
    pub gpioc_dinr: GPIOC_DINR,
    #[doc = "0x20 - GPIOC_DOUTR"]
    pub gpioc_doutr: GPIOC_DOUTR,
    #[doc = "0x24 - GPIOC_SRR"]
    pub gpioc_srr: GPIOC_SRR,
    #[doc = "0x28 - GPIOC_RR"]
    pub gpioc_rr: GPIOC_RR,
}
#[doc = "GPIOC_DIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_dircr](gpioc_dircr) module"]
pub type GPIOC_DIRCR = crate::Reg<u32, _GPIOC_DIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_DIRCR;
#[doc = "`read()` method returns [gpioc_dircr::R](gpioc_dircr::R) reader structure"]
impl crate::Readable for GPIOC_DIRCR {}
#[doc = "`write(|w| ..)` method takes [gpioc_dircr::W](gpioc_dircr::W) writer structure"]
impl crate::Writable for GPIOC_DIRCR {}
#[doc = "GPIOC_DIRCR"]
pub mod gpioc_dircr;
#[doc = "GPIOC_INER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_iner](gpioc_iner) module"]
pub type GPIOC_INER = crate::Reg<u32, _GPIOC_INER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_INER;
#[doc = "`read()` method returns [gpioc_iner::R](gpioc_iner::R) reader structure"]
impl crate::Readable for GPIOC_INER {}
#[doc = "`write(|w| ..)` method takes [gpioc_iner::W](gpioc_iner::W) writer structure"]
impl crate::Writable for GPIOC_INER {}
#[doc = "GPIOC_INER"]
pub mod gpioc_iner;
#[doc = "GPIOC_PUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_pur](gpioc_pur) module"]
pub type GPIOC_PUR = crate::Reg<u32, _GPIOC_PUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_PUR;
#[doc = "`read()` method returns [gpioc_pur::R](gpioc_pur::R) reader structure"]
impl crate::Readable for GPIOC_PUR {}
#[doc = "`write(|w| ..)` method takes [gpioc_pur::W](gpioc_pur::W) writer structure"]
impl crate::Writable for GPIOC_PUR {}
#[doc = "GPIOC_PUR"]
pub mod gpioc_pur;
#[doc = "GPIOC_PDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_pdr](gpioc_pdr) module"]
pub type GPIOC_PDR = crate::Reg<u32, _GPIOC_PDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_PDR;
#[doc = "`read()` method returns [gpioc_pdr::R](gpioc_pdr::R) reader structure"]
impl crate::Readable for GPIOC_PDR {}
#[doc = "`write(|w| ..)` method takes [gpioc_pdr::W](gpioc_pdr::W) writer structure"]
impl crate::Writable for GPIOC_PDR {}
#[doc = "GPIOC_PDR"]
pub mod gpioc_pdr;
#[doc = "GPIOC_ODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_odr](gpioc_odr) module"]
pub type GPIOC_ODR = crate::Reg<u32, _GPIOC_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_ODR;
#[doc = "`read()` method returns [gpioc_odr::R](gpioc_odr::R) reader structure"]
impl crate::Readable for GPIOC_ODR {}
#[doc = "`write(|w| ..)` method takes [gpioc_odr::W](gpioc_odr::W) writer structure"]
impl crate::Writable for GPIOC_ODR {}
#[doc = "GPIOC_ODR"]
pub mod gpioc_odr;
#[doc = "GPIOC_LOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_lockr](gpioc_lockr) module"]
pub type GPIOC_LOCKR = crate::Reg<u32, _GPIOC_LOCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_LOCKR;
#[doc = "`read()` method returns [gpioc_lockr::R](gpioc_lockr::R) reader structure"]
impl crate::Readable for GPIOC_LOCKR {}
#[doc = "`write(|w| ..)` method takes [gpioc_lockr::W](gpioc_lockr::W) writer structure"]
impl crate::Writable for GPIOC_LOCKR {}
#[doc = "GPIOC_LOCKR"]
pub mod gpioc_lockr;
#[doc = "GPIOC_DINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_dinr](gpioc_dinr) module"]
pub type GPIOC_DINR = crate::Reg<u32, _GPIOC_DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_DINR;
#[doc = "`read()` method returns [gpioc_dinr::R](gpioc_dinr::R) reader structure"]
impl crate::Readable for GPIOC_DINR {}
#[doc = "`write(|w| ..)` method takes [gpioc_dinr::W](gpioc_dinr::W) writer structure"]
impl crate::Writable for GPIOC_DINR {}
#[doc = "GPIOC_DINR"]
pub mod gpioc_dinr;
#[doc = "GPIOC_DOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_doutr](gpioc_doutr) module"]
pub type GPIOC_DOUTR = crate::Reg<u32, _GPIOC_DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_DOUTR;
#[doc = "`read()` method returns [gpioc_doutr::R](gpioc_doutr::R) reader structure"]
impl crate::Readable for GPIOC_DOUTR {}
#[doc = "`write(|w| ..)` method takes [gpioc_doutr::W](gpioc_doutr::W) writer structure"]
impl crate::Writable for GPIOC_DOUTR {}
#[doc = "GPIOC_DOUTR"]
pub mod gpioc_doutr;
#[doc = "GPIOC_SRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_srr](gpioc_srr) module"]
pub type GPIOC_SRR = crate::Reg<u32, _GPIOC_SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_SRR;
#[doc = "`read()` method returns [gpioc_srr::R](gpioc_srr::R) reader structure"]
impl crate::Readable for GPIOC_SRR {}
#[doc = "`write(|w| ..)` method takes [gpioc_srr::W](gpioc_srr::W) writer structure"]
impl crate::Writable for GPIOC_SRR {}
#[doc = "GPIOC_SRR"]
pub mod gpioc_srr;
#[doc = "GPIOC_RR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioc_rr](gpioc_rr) module"]
pub type GPIOC_RR = crate::Reg<u32, _GPIOC_RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOC_RR;
#[doc = "`read()` method returns [gpioc_rr::R](gpioc_rr::R) reader structure"]
impl crate::Readable for GPIOC_RR {}
#[doc = "`write(|w| ..)` method takes [gpioc_rr::W](gpioc_rr::W) writer structure"]
impl crate::Writable for GPIOC_RR {}
#[doc = "GPIOC_RR"]
pub mod gpioc_rr;
