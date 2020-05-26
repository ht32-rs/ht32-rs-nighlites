#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCTM_CNTCFR"]
    pub mctm_cntcfr: MCTM_CNTCFR,
    #[doc = "0x04 - MCTM_MDCFR"]
    pub mctm_mdcfr: MCTM_MDCFR,
    #[doc = "0x08 - MCTM_TRCFR"]
    pub mctm_trcfr: MCTM_TRCFR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - MCTM_CTR"]
    pub mctm_ctr: MCTM_CTR,
    _reserved4: [u8; 12usize],
    #[doc = "0x20 - MCTM_CH0ICFR"]
    pub mctm_ch0icfr: MCTM_CH0ICFR,
    #[doc = "0x24 - MCTM_CH1ICFR"]
    pub mctm_ch1icfr: MCTM_CH1ICFR,
    #[doc = "0x28 - MCTM_CH2ICFR"]
    pub mctm_ch2icfr: MCTM_CH2ICFR,
    #[doc = "0x2c - MCTM_CH3ICFR"]
    pub mctm_ch3icfr: MCTM_CH3ICFR,
    _reserved8: [u8; 16usize],
    #[doc = "0x40 - MCTM_CH0OCFR"]
    pub mctm_ch0ocfr: MCTM_CH0OCFR,
    #[doc = "0x44 - MCTM_CH1OCFR"]
    pub mctm_ch1ocfr: MCTM_CH1OCFR,
    #[doc = "0x48 - MCTM_CH2OCFR"]
    pub mctm_ch2ocfr: MCTM_CH2OCFR,
    #[doc = "0x4c - MCTM_CH3OCFR"]
    pub mctm_ch3ocfr: MCTM_CH3OCFR,
    #[doc = "0x50 - MCTM_CHCTR"]
    pub mctm_chctr: MCTM_CHCTR,
    #[doc = "0x54 - MCTM_CHPOLR"]
    pub mctm_chpolr: MCTM_CHPOLR,
    _reserved14: [u8; 20usize],
    #[doc = "0x6c - MCTM_CHBRKCFR"]
    pub mctm_chbrkcfr: MCTM_CHBRKCFR,
    #[doc = "0x70 - MCTM_CHBRKCTR"]
    pub mctm_chbrkctr: MCTM_CHBRKCTR,
    #[doc = "0x74 - MCTM_DICTR"]
    pub mctm_dictr: MCTM_DICTR,
    #[doc = "0x78 - MCTM_EVGR"]
    pub mctm_evgr: MCTM_EVGR,
    #[doc = "0x7c - MCTM_INTSR"]
    pub mctm_intsr: MCTM_INTSR,
    #[doc = "0x80 - MCTM_CNTR"]
    pub mctm_cntr: MCTM_CNTR,
    #[doc = "0x84 - MCTM_PSCR"]
    pub mctm_pscr: MCTM_PSCR,
    #[doc = "0x88 - MCTM_CRR"]
    pub mctm_crr: MCTM_CRR,
    #[doc = "0x8c - MCTM_REPR"]
    pub mctm_repr: MCTM_REPR,
    #[doc = "0x90 - MCTM_CH0CCR"]
    pub mctm_ch0ccr: MCTM_CH0CCR,
    #[doc = "0x94 - MCTM_CH1CCR"]
    pub mctm_ch1ccr: MCTM_CH1CCR,
    #[doc = "0x98 - MCTM_CH2CCR"]
    pub mctm_ch2ccr: MCTM_CH2CCR,
    #[doc = "0x9c - MCTM_CH3CCR"]
    pub mctm_ch3ccr: MCTM_CH3CCR,
    #[doc = "0xa0 - MCTM_CH0ACR"]
    pub mctm_ch0acr: MCTM_CH0ACR,
    #[doc = "0xa4 - MCTM_CH1ACR"]
    pub mctm_ch1acr: MCTM_CH1ACR,
    #[doc = "0xa8 - MCTM_CH2ACR"]
    pub mctm_ch2acr: MCTM_CH2ACR,
    #[doc = "0xac - MCTM_CH3ACR"]
    pub mctm_ch3acr: MCTM_CH3ACR,
}
#[doc = "MCTM_CNTCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_cntcfr](mctm_cntcfr) module"]
pub type MCTM_CNTCFR = crate::Reg<u32, _MCTM_CNTCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CNTCFR;
#[doc = "`read()` method returns [mctm_cntcfr::R](mctm_cntcfr::R) reader structure"]
impl crate::Readable for MCTM_CNTCFR {}
#[doc = "`write(|w| ..)` method takes [mctm_cntcfr::W](mctm_cntcfr::W) writer structure"]
impl crate::Writable for MCTM_CNTCFR {}
#[doc = "MCTM_CNTCFR"]
pub mod mctm_cntcfr;
#[doc = "MCTM_MDCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_mdcfr](mctm_mdcfr) module"]
pub type MCTM_MDCFR = crate::Reg<u32, _MCTM_MDCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_MDCFR;
#[doc = "`read()` method returns [mctm_mdcfr::R](mctm_mdcfr::R) reader structure"]
impl crate::Readable for MCTM_MDCFR {}
#[doc = "`write(|w| ..)` method takes [mctm_mdcfr::W](mctm_mdcfr::W) writer structure"]
impl crate::Writable for MCTM_MDCFR {}
#[doc = "MCTM_MDCFR"]
pub mod mctm_mdcfr;
#[doc = "MCTM_TRCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_trcfr](mctm_trcfr) module"]
pub type MCTM_TRCFR = crate::Reg<u32, _MCTM_TRCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_TRCFR;
#[doc = "`read()` method returns [mctm_trcfr::R](mctm_trcfr::R) reader structure"]
impl crate::Readable for MCTM_TRCFR {}
#[doc = "`write(|w| ..)` method takes [mctm_trcfr::W](mctm_trcfr::W) writer structure"]
impl crate::Writable for MCTM_TRCFR {}
#[doc = "MCTM_TRCFR"]
pub mod mctm_trcfr;
#[doc = "MCTM_CTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ctr](mctm_ctr) module"]
pub type MCTM_CTR = crate::Reg<u32, _MCTM_CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CTR;
#[doc = "`read()` method returns [mctm_ctr::R](mctm_ctr::R) reader structure"]
impl crate::Readable for MCTM_CTR {}
#[doc = "`write(|w| ..)` method takes [mctm_ctr::W](mctm_ctr::W) writer structure"]
impl crate::Writable for MCTM_CTR {}
#[doc = "MCTM_CTR"]
pub mod mctm_ctr;
#[doc = "MCTM_CH0ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch0icfr](mctm_ch0icfr) module"]
pub type MCTM_CH0ICFR = crate::Reg<u32, _MCTM_CH0ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH0ICFR;
#[doc = "`read()` method returns [mctm_ch0icfr::R](mctm_ch0icfr::R) reader structure"]
impl crate::Readable for MCTM_CH0ICFR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch0icfr::W](mctm_ch0icfr::W) writer structure"]
impl crate::Writable for MCTM_CH0ICFR {}
#[doc = "MCTM_CH0ICFR"]
pub mod mctm_ch0icfr;
#[doc = "MCTM_CH1ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch1icfr](mctm_ch1icfr) module"]
pub type MCTM_CH1ICFR = crate::Reg<u32, _MCTM_CH1ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH1ICFR;
#[doc = "`read()` method returns [mctm_ch1icfr::R](mctm_ch1icfr::R) reader structure"]
impl crate::Readable for MCTM_CH1ICFR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch1icfr::W](mctm_ch1icfr::W) writer structure"]
impl crate::Writable for MCTM_CH1ICFR {}
#[doc = "MCTM_CH1ICFR"]
pub mod mctm_ch1icfr;
#[doc = "MCTM_CH2ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch2icfr](mctm_ch2icfr) module"]
pub type MCTM_CH2ICFR = crate::Reg<u32, _MCTM_CH2ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH2ICFR;
#[doc = "`read()` method returns [mctm_ch2icfr::R](mctm_ch2icfr::R) reader structure"]
impl crate::Readable for MCTM_CH2ICFR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch2icfr::W](mctm_ch2icfr::W) writer structure"]
impl crate::Writable for MCTM_CH2ICFR {}
#[doc = "MCTM_CH2ICFR"]
pub mod mctm_ch2icfr;
#[doc = "MCTM_CH3ICFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch3icfr](mctm_ch3icfr) module"]
pub type MCTM_CH3ICFR = crate::Reg<u32, _MCTM_CH3ICFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH3ICFR;
#[doc = "`read()` method returns [mctm_ch3icfr::R](mctm_ch3icfr::R) reader structure"]
impl crate::Readable for MCTM_CH3ICFR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch3icfr::W](mctm_ch3icfr::W) writer structure"]
impl crate::Writable for MCTM_CH3ICFR {}
#[doc = "MCTM_CH3ICFR"]
pub mod mctm_ch3icfr;
#[doc = "MCTM_CH0OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch0ocfr](mctm_ch0ocfr) module"]
pub type MCTM_CH0OCFR = crate::Reg<u32, _MCTM_CH0OCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH0OCFR;
#[doc = "`read()` method returns [mctm_ch0ocfr::R](mctm_ch0ocfr::R) reader structure"]
impl crate::Readable for MCTM_CH0OCFR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch0ocfr::W](mctm_ch0ocfr::W) writer structure"]
impl crate::Writable for MCTM_CH0OCFR {}
#[doc = "MCTM_CH0OCFR"]
pub mod mctm_ch0ocfr;
#[doc = "MCTM_CH1OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch1ocfr](mctm_ch1ocfr) module"]
pub type MCTM_CH1OCFR = crate::Reg<u32, _MCTM_CH1OCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH1OCFR;
#[doc = "`read()` method returns [mctm_ch1ocfr::R](mctm_ch1ocfr::R) reader structure"]
impl crate::Readable for MCTM_CH1OCFR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch1ocfr::W](mctm_ch1ocfr::W) writer structure"]
impl crate::Writable for MCTM_CH1OCFR {}
#[doc = "MCTM_CH1OCFR"]
pub mod mctm_ch1ocfr;
#[doc = "MCTM_CH2OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch2ocfr](mctm_ch2ocfr) module"]
pub type MCTM_CH2OCFR = crate::Reg<u32, _MCTM_CH2OCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH2OCFR;
#[doc = "`read()` method returns [mctm_ch2ocfr::R](mctm_ch2ocfr::R) reader structure"]
impl crate::Readable for MCTM_CH2OCFR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch2ocfr::W](mctm_ch2ocfr::W) writer structure"]
impl crate::Writable for MCTM_CH2OCFR {}
#[doc = "MCTM_CH2OCFR"]
pub mod mctm_ch2ocfr;
#[doc = "MCTM_CH3OCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch3ocfr](mctm_ch3ocfr) module"]
pub type MCTM_CH3OCFR = crate::Reg<u32, _MCTM_CH3OCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH3OCFR;
#[doc = "`read()` method returns [mctm_ch3ocfr::R](mctm_ch3ocfr::R) reader structure"]
impl crate::Readable for MCTM_CH3OCFR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch3ocfr::W](mctm_ch3ocfr::W) writer structure"]
impl crate::Writable for MCTM_CH3OCFR {}
#[doc = "MCTM_CH3OCFR"]
pub mod mctm_ch3ocfr;
#[doc = "MCTM_CHCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_chctr](mctm_chctr) module"]
pub type MCTM_CHCTR = crate::Reg<u32, _MCTM_CHCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CHCTR;
#[doc = "`read()` method returns [mctm_chctr::R](mctm_chctr::R) reader structure"]
impl crate::Readable for MCTM_CHCTR {}
#[doc = "`write(|w| ..)` method takes [mctm_chctr::W](mctm_chctr::W) writer structure"]
impl crate::Writable for MCTM_CHCTR {}
#[doc = "MCTM_CHCTR"]
pub mod mctm_chctr;
#[doc = "MCTM_CHPOLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_chpolr](mctm_chpolr) module"]
pub type MCTM_CHPOLR = crate::Reg<u32, _MCTM_CHPOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CHPOLR;
#[doc = "`read()` method returns [mctm_chpolr::R](mctm_chpolr::R) reader structure"]
impl crate::Readable for MCTM_CHPOLR {}
#[doc = "`write(|w| ..)` method takes [mctm_chpolr::W](mctm_chpolr::W) writer structure"]
impl crate::Writable for MCTM_CHPOLR {}
#[doc = "MCTM_CHPOLR"]
pub mod mctm_chpolr;
#[doc = "MCTM_CHBRKCFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_chbrkcfr](mctm_chbrkcfr) module"]
pub type MCTM_CHBRKCFR = crate::Reg<u32, _MCTM_CHBRKCFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CHBRKCFR;
#[doc = "`read()` method returns [mctm_chbrkcfr::R](mctm_chbrkcfr::R) reader structure"]
impl crate::Readable for MCTM_CHBRKCFR {}
#[doc = "`write(|w| ..)` method takes [mctm_chbrkcfr::W](mctm_chbrkcfr::W) writer structure"]
impl crate::Writable for MCTM_CHBRKCFR {}
#[doc = "MCTM_CHBRKCFR"]
pub mod mctm_chbrkcfr;
#[doc = "MCTM_CHBRKCTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_chbrkctr](mctm_chbrkctr) module"]
pub type MCTM_CHBRKCTR = crate::Reg<u32, _MCTM_CHBRKCTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CHBRKCTR;
#[doc = "`read()` method returns [mctm_chbrkctr::R](mctm_chbrkctr::R) reader structure"]
impl crate::Readable for MCTM_CHBRKCTR {}
#[doc = "`write(|w| ..)` method takes [mctm_chbrkctr::W](mctm_chbrkctr::W) writer structure"]
impl crate::Writable for MCTM_CHBRKCTR {}
#[doc = "MCTM_CHBRKCTR"]
pub mod mctm_chbrkctr;
#[doc = "MCTM_DICTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_dictr](mctm_dictr) module"]
pub type MCTM_DICTR = crate::Reg<u32, _MCTM_DICTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_DICTR;
#[doc = "`read()` method returns [mctm_dictr::R](mctm_dictr::R) reader structure"]
impl crate::Readable for MCTM_DICTR {}
#[doc = "`write(|w| ..)` method takes [mctm_dictr::W](mctm_dictr::W) writer structure"]
impl crate::Writable for MCTM_DICTR {}
#[doc = "MCTM_DICTR"]
pub mod mctm_dictr;
#[doc = "MCTM_EVGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_evgr](mctm_evgr) module"]
pub type MCTM_EVGR = crate::Reg<u32, _MCTM_EVGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_EVGR;
#[doc = "`read()` method returns [mctm_evgr::R](mctm_evgr::R) reader structure"]
impl crate::Readable for MCTM_EVGR {}
#[doc = "`write(|w| ..)` method takes [mctm_evgr::W](mctm_evgr::W) writer structure"]
impl crate::Writable for MCTM_EVGR {}
#[doc = "MCTM_EVGR"]
pub mod mctm_evgr;
#[doc = "MCTM_INTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_intsr](mctm_intsr) module"]
pub type MCTM_INTSR = crate::Reg<u32, _MCTM_INTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_INTSR;
#[doc = "`read()` method returns [mctm_intsr::R](mctm_intsr::R) reader structure"]
impl crate::Readable for MCTM_INTSR {}
#[doc = "`write(|w| ..)` method takes [mctm_intsr::W](mctm_intsr::W) writer structure"]
impl crate::Writable for MCTM_INTSR {}
#[doc = "MCTM_INTSR"]
pub mod mctm_intsr;
#[doc = "MCTM_CNTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_cntr](mctm_cntr) module"]
pub type MCTM_CNTR = crate::Reg<u32, _MCTM_CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CNTR;
#[doc = "`read()` method returns [mctm_cntr::R](mctm_cntr::R) reader structure"]
impl crate::Readable for MCTM_CNTR {}
#[doc = "`write(|w| ..)` method takes [mctm_cntr::W](mctm_cntr::W) writer structure"]
impl crate::Writable for MCTM_CNTR {}
#[doc = "MCTM_CNTR"]
pub mod mctm_cntr;
#[doc = "MCTM_PSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_pscr](mctm_pscr) module"]
pub type MCTM_PSCR = crate::Reg<u32, _MCTM_PSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_PSCR;
#[doc = "`read()` method returns [mctm_pscr::R](mctm_pscr::R) reader structure"]
impl crate::Readable for MCTM_PSCR {}
#[doc = "`write(|w| ..)` method takes [mctm_pscr::W](mctm_pscr::W) writer structure"]
impl crate::Writable for MCTM_PSCR {}
#[doc = "MCTM_PSCR"]
pub mod mctm_pscr;
#[doc = "MCTM_CRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_crr](mctm_crr) module"]
pub type MCTM_CRR = crate::Reg<u32, _MCTM_CRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CRR;
#[doc = "`read()` method returns [mctm_crr::R](mctm_crr::R) reader structure"]
impl crate::Readable for MCTM_CRR {}
#[doc = "`write(|w| ..)` method takes [mctm_crr::W](mctm_crr::W) writer structure"]
impl crate::Writable for MCTM_CRR {}
#[doc = "MCTM_CRR"]
pub mod mctm_crr;
#[doc = "MCTM_REPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_repr](mctm_repr) module"]
pub type MCTM_REPR = crate::Reg<u32, _MCTM_REPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_REPR;
#[doc = "`read()` method returns [mctm_repr::R](mctm_repr::R) reader structure"]
impl crate::Readable for MCTM_REPR {}
#[doc = "`write(|w| ..)` method takes [mctm_repr::W](mctm_repr::W) writer structure"]
impl crate::Writable for MCTM_REPR {}
#[doc = "MCTM_REPR"]
pub mod mctm_repr;
#[doc = "MCTM_CH0CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch0ccr](mctm_ch0ccr) module"]
pub type MCTM_CH0CCR = crate::Reg<u32, _MCTM_CH0CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH0CCR;
#[doc = "`read()` method returns [mctm_ch0ccr::R](mctm_ch0ccr::R) reader structure"]
impl crate::Readable for MCTM_CH0CCR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch0ccr::W](mctm_ch0ccr::W) writer structure"]
impl crate::Writable for MCTM_CH0CCR {}
#[doc = "MCTM_CH0CCR"]
pub mod mctm_ch0ccr;
#[doc = "MCTM_CH1CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch1ccr](mctm_ch1ccr) module"]
pub type MCTM_CH1CCR = crate::Reg<u32, _MCTM_CH1CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH1CCR;
#[doc = "`read()` method returns [mctm_ch1ccr::R](mctm_ch1ccr::R) reader structure"]
impl crate::Readable for MCTM_CH1CCR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch1ccr::W](mctm_ch1ccr::W) writer structure"]
impl crate::Writable for MCTM_CH1CCR {}
#[doc = "MCTM_CH1CCR"]
pub mod mctm_ch1ccr;
#[doc = "MCTM_CH2CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch2ccr](mctm_ch2ccr) module"]
pub type MCTM_CH2CCR = crate::Reg<u32, _MCTM_CH2CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH2CCR;
#[doc = "`read()` method returns [mctm_ch2ccr::R](mctm_ch2ccr::R) reader structure"]
impl crate::Readable for MCTM_CH2CCR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch2ccr::W](mctm_ch2ccr::W) writer structure"]
impl crate::Writable for MCTM_CH2CCR {}
#[doc = "MCTM_CH2CCR"]
pub mod mctm_ch2ccr;
#[doc = "MCTM_CH3CCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch3ccr](mctm_ch3ccr) module"]
pub type MCTM_CH3CCR = crate::Reg<u32, _MCTM_CH3CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH3CCR;
#[doc = "`read()` method returns [mctm_ch3ccr::R](mctm_ch3ccr::R) reader structure"]
impl crate::Readable for MCTM_CH3CCR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch3ccr::W](mctm_ch3ccr::W) writer structure"]
impl crate::Writable for MCTM_CH3CCR {}
#[doc = "MCTM_CH3CCR"]
pub mod mctm_ch3ccr;
#[doc = "MCTM_CH0ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch0acr](mctm_ch0acr) module"]
pub type MCTM_CH0ACR = crate::Reg<u32, _MCTM_CH0ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH0ACR;
#[doc = "`read()` method returns [mctm_ch0acr::R](mctm_ch0acr::R) reader structure"]
impl crate::Readable for MCTM_CH0ACR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch0acr::W](mctm_ch0acr::W) writer structure"]
impl crate::Writable for MCTM_CH0ACR {}
#[doc = "MCTM_CH0ACR"]
pub mod mctm_ch0acr;
#[doc = "MCTM_CH1ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch1acr](mctm_ch1acr) module"]
pub type MCTM_CH1ACR = crate::Reg<u32, _MCTM_CH1ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH1ACR;
#[doc = "`read()` method returns [mctm_ch1acr::R](mctm_ch1acr::R) reader structure"]
impl crate::Readable for MCTM_CH1ACR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch1acr::W](mctm_ch1acr::W) writer structure"]
impl crate::Writable for MCTM_CH1ACR {}
#[doc = "MCTM_CH1ACR"]
pub mod mctm_ch1acr;
#[doc = "MCTM_CH2ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch2acr](mctm_ch2acr) module"]
pub type MCTM_CH2ACR = crate::Reg<u32, _MCTM_CH2ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH2ACR;
#[doc = "`read()` method returns [mctm_ch2acr::R](mctm_ch2acr::R) reader structure"]
impl crate::Readable for MCTM_CH2ACR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch2acr::W](mctm_ch2acr::W) writer structure"]
impl crate::Writable for MCTM_CH2ACR {}
#[doc = "MCTM_CH2ACR"]
pub mod mctm_ch2acr;
#[doc = "MCTM_CH3ACR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_ch3acr](mctm_ch3acr) module"]
pub type MCTM_CH3ACR = crate::Reg<u32, _MCTM_CH3ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTM_CH3ACR;
#[doc = "`read()` method returns [mctm_ch3acr::R](mctm_ch3acr::R) reader structure"]
impl crate::Readable for MCTM_CH3ACR {}
#[doc = "`write(|w| ..)` method takes [mctm_ch3acr::W](mctm_ch3acr::W) writer structure"]
impl crate::Writable for MCTM_CH3ACR {}
#[doc = "MCTM_CH3ACR"]
pub mod mctm_ch3acr;
