#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GPIOD_DIRCR"]
    pub gpiod_dircr: GPIOD_DIRCR,
    #[doc = "0x04 - GPIOD_INER"]
    pub gpiod_iner: GPIOD_INER,
    #[doc = "0x08 - GPIOD_PUR"]
    pub gpiod_pur: GPIOD_PUR,
    #[doc = "0x0c - GPIOD_PDR"]
    pub gpiod_pdr: GPIOD_PDR,
    #[doc = "0x10 - GPIOD_ODR"]
    pub gpiod_odr: GPIOD_ODR,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - GPIOD_LOCKR"]
    pub gpiod_lockr: GPIOD_LOCKR,
    #[doc = "0x1c - GPIOD_DINR"]
    pub gpiod_dinr: GPIOD_DINR,
    #[doc = "0x20 - GPIOD_DOUTR"]
    pub gpiod_doutr: GPIOD_DOUTR,
    #[doc = "0x24 - GPIOD_SRR"]
    pub gpiod_srr: GPIOD_SRR,
    #[doc = "0x28 - GPIOD_RR"]
    pub gpiod_rr: GPIOD_RR,
}
#[doc = "GPIOD_DIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_dircr](gpiod_dircr) module"]
pub type GPIOD_DIRCR = crate::Reg<u32, _GPIOD_DIRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_DIRCR;
#[doc = "`read()` method returns [gpiod_dircr::R](gpiod_dircr::R) reader structure"]
impl crate::Readable for GPIOD_DIRCR {}
#[doc = "`write(|w| ..)` method takes [gpiod_dircr::W](gpiod_dircr::W) writer structure"]
impl crate::Writable for GPIOD_DIRCR {}
#[doc = "GPIOD_DIRCR"]
pub mod gpiod_dircr;
#[doc = "GPIOD_INER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_iner](gpiod_iner) module"]
pub type GPIOD_INER = crate::Reg<u32, _GPIOD_INER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_INER;
#[doc = "`read()` method returns [gpiod_iner::R](gpiod_iner::R) reader structure"]
impl crate::Readable for GPIOD_INER {}
#[doc = "`write(|w| ..)` method takes [gpiod_iner::W](gpiod_iner::W) writer structure"]
impl crate::Writable for GPIOD_INER {}
#[doc = "GPIOD_INER"]
pub mod gpiod_iner;
#[doc = "GPIOD_PUR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_pur](gpiod_pur) module"]
pub type GPIOD_PUR = crate::Reg<u32, _GPIOD_PUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_PUR;
#[doc = "`read()` method returns [gpiod_pur::R](gpiod_pur::R) reader structure"]
impl crate::Readable for GPIOD_PUR {}
#[doc = "`write(|w| ..)` method takes [gpiod_pur::W](gpiod_pur::W) writer structure"]
impl crate::Writable for GPIOD_PUR {}
#[doc = "GPIOD_PUR"]
pub mod gpiod_pur;
#[doc = "GPIOD_PDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_pdr](gpiod_pdr) module"]
pub type GPIOD_PDR = crate::Reg<u32, _GPIOD_PDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_PDR;
#[doc = "`read()` method returns [gpiod_pdr::R](gpiod_pdr::R) reader structure"]
impl crate::Readable for GPIOD_PDR {}
#[doc = "`write(|w| ..)` method takes [gpiod_pdr::W](gpiod_pdr::W) writer structure"]
impl crate::Writable for GPIOD_PDR {}
#[doc = "GPIOD_PDR"]
pub mod gpiod_pdr;
#[doc = "GPIOD_ODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_odr](gpiod_odr) module"]
pub type GPIOD_ODR = crate::Reg<u32, _GPIOD_ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_ODR;
#[doc = "`read()` method returns [gpiod_odr::R](gpiod_odr::R) reader structure"]
impl crate::Readable for GPIOD_ODR {}
#[doc = "`write(|w| ..)` method takes [gpiod_odr::W](gpiod_odr::W) writer structure"]
impl crate::Writable for GPIOD_ODR {}
#[doc = "GPIOD_ODR"]
pub mod gpiod_odr;
#[doc = "GPIOD_LOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_lockr](gpiod_lockr) module"]
pub type GPIOD_LOCKR = crate::Reg<u32, _GPIOD_LOCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_LOCKR;
#[doc = "`read()` method returns [gpiod_lockr::R](gpiod_lockr::R) reader structure"]
impl crate::Readable for GPIOD_LOCKR {}
#[doc = "`write(|w| ..)` method takes [gpiod_lockr::W](gpiod_lockr::W) writer structure"]
impl crate::Writable for GPIOD_LOCKR {}
#[doc = "GPIOD_LOCKR"]
pub mod gpiod_lockr;
#[doc = "GPIOD_DINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_dinr](gpiod_dinr) module"]
pub type GPIOD_DINR = crate::Reg<u32, _GPIOD_DINR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_DINR;
#[doc = "`read()` method returns [gpiod_dinr::R](gpiod_dinr::R) reader structure"]
impl crate::Readable for GPIOD_DINR {}
#[doc = "`write(|w| ..)` method takes [gpiod_dinr::W](gpiod_dinr::W) writer structure"]
impl crate::Writable for GPIOD_DINR {}
#[doc = "GPIOD_DINR"]
pub mod gpiod_dinr;
#[doc = "GPIOD_DOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_doutr](gpiod_doutr) module"]
pub type GPIOD_DOUTR = crate::Reg<u32, _GPIOD_DOUTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_DOUTR;
#[doc = "`read()` method returns [gpiod_doutr::R](gpiod_doutr::R) reader structure"]
impl crate::Readable for GPIOD_DOUTR {}
#[doc = "`write(|w| ..)` method takes [gpiod_doutr::W](gpiod_doutr::W) writer structure"]
impl crate::Writable for GPIOD_DOUTR {}
#[doc = "GPIOD_DOUTR"]
pub mod gpiod_doutr;
#[doc = "GPIOD_SRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_srr](gpiod_srr) module"]
pub type GPIOD_SRR = crate::Reg<u32, _GPIOD_SRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_SRR;
#[doc = "`read()` method returns [gpiod_srr::R](gpiod_srr::R) reader structure"]
impl crate::Readable for GPIOD_SRR {}
#[doc = "`write(|w| ..)` method takes [gpiod_srr::W](gpiod_srr::W) writer structure"]
impl crate::Writable for GPIOD_SRR {}
#[doc = "GPIOD_SRR"]
pub mod gpiod_srr;
#[doc = "GPIOD_RR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpiod_rr](gpiod_rr) module"]
pub type GPIOD_RR = crate::Reg<u32, _GPIOD_RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIOD_RR;
#[doc = "`read()` method returns [gpiod_rr::R](gpiod_rr::R) reader structure"]
impl crate::Readable for GPIOD_RR {}
#[doc = "`write(|w| ..)` method takes [gpiod_rr::W](gpiod_rr::W) writer structure"]
impl crate::Writable for GPIOD_RR {}
#[doc = "GPIOD_RR"]
pub mod gpiod_rr;
