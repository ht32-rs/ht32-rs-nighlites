#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CKCU_GCFGR"]
    pub ckcu_gcfgr: CKCU_GCFGR,
    #[doc = "0x04 - CKCU_GCCR"]
    pub ckcu_gccr: CKCU_GCCR,
    #[doc = "0x08 - CKCU_GCSR"]
    pub ckcu_gcsr: CKCU_GCSR,
    #[doc = "0x0c - CKCU_GCIR"]
    pub ckcu_gcir: CKCU_GCIR,
    _reserved4: [u8; 8usize],
    #[doc = "0x18 - CKCU_PLLCFGR"]
    pub ckcu_pllcfgr: CKCU_PLLCFGR,
    #[doc = "0x1c - CKCU_PLLCR"]
    pub ckcu_pllcr: CKCU_PLLCR,
    #[doc = "0x20 - CKCU_AHBCFGR"]
    pub ckcu_ahbcfgr: CKCU_AHBCFGR,
    #[doc = "0x24 - CKCU_AHBCCR"]
    pub ckcu_ahbccr: CKCU_AHBCCR,
    #[doc = "0x28 - CKCU_APBCFGR"]
    pub ckcu_apbcfgr: CKCU_APBCFGR,
    #[doc = "0x2c - CKCU_APBCCR0"]
    pub ckcu_apbccr0: CKCU_APBCCR0,
    #[doc = "0x30 - CKCU_APBCCR1"]
    pub ckcu_apbccr1: CKCU_APBCCR1,
    #[doc = "0x34 - CKCU_CKST"]
    pub ckcu_ckst: CKCU_CKST,
    #[doc = "0x38 - CKCU_APBPCSR0"]
    pub ckcu_apbpcsr0: CKCU_APBPCSR0,
    #[doc = "0x3c - CKCU_APBPCSR1"]
    pub ckcu_apbpcsr1: CKCU_APBPCSR1,
    #[doc = "0x40 - CKCU_HSICR"]
    pub ckcu_hsicr: CKCU_HSICR,
    #[doc = "0x44 - CKCU_HSIATCR"]
    pub ckcu_hsiatcr: CKCU_HSIATCR,
    _reserved16: [u8; 696usize],
    #[doc = "0x300 - CKCU_LPCR"]
    pub ckcu_lpcr: CKCU_LPCR,
    #[doc = "0x304 - CKCU_MCUDBGCR"]
    pub ckcu_mcudbgcr: CKCU_MCUDBGCR,
}
#[doc = "CKCU_GCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_gcfgr](ckcu_gcfgr) module"]
pub type CKCU_GCFGR = crate::Reg<u32, _CKCU_GCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_GCFGR;
#[doc = "`read()` method returns [ckcu_gcfgr::R](ckcu_gcfgr::R) reader structure"]
impl crate::Readable for CKCU_GCFGR {}
#[doc = "`write(|w| ..)` method takes [ckcu_gcfgr::W](ckcu_gcfgr::W) writer structure"]
impl crate::Writable for CKCU_GCFGR {}
#[doc = "CKCU_GCFGR"]
pub mod ckcu_gcfgr;
#[doc = "CKCU_GCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_gccr](ckcu_gccr) module"]
pub type CKCU_GCCR = crate::Reg<u32, _CKCU_GCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_GCCR;
#[doc = "`read()` method returns [ckcu_gccr::R](ckcu_gccr::R) reader structure"]
impl crate::Readable for CKCU_GCCR {}
#[doc = "`write(|w| ..)` method takes [ckcu_gccr::W](ckcu_gccr::W) writer structure"]
impl crate::Writable for CKCU_GCCR {}
#[doc = "CKCU_GCCR"]
pub mod ckcu_gccr;
#[doc = "CKCU_GCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_gcsr](ckcu_gcsr) module"]
pub type CKCU_GCSR = crate::Reg<u32, _CKCU_GCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_GCSR;
#[doc = "`read()` method returns [ckcu_gcsr::R](ckcu_gcsr::R) reader structure"]
impl crate::Readable for CKCU_GCSR {}
#[doc = "`write(|w| ..)` method takes [ckcu_gcsr::W](ckcu_gcsr::W) writer structure"]
impl crate::Writable for CKCU_GCSR {}
#[doc = "CKCU_GCSR"]
pub mod ckcu_gcsr;
#[doc = "CKCU_GCIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_gcir](ckcu_gcir) module"]
pub type CKCU_GCIR = crate::Reg<u32, _CKCU_GCIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_GCIR;
#[doc = "`read()` method returns [ckcu_gcir::R](ckcu_gcir::R) reader structure"]
impl crate::Readable for CKCU_GCIR {}
#[doc = "`write(|w| ..)` method takes [ckcu_gcir::W](ckcu_gcir::W) writer structure"]
impl crate::Writable for CKCU_GCIR {}
#[doc = "CKCU_GCIR"]
pub mod ckcu_gcir;
#[doc = "CKCU_PLLCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_pllcfgr](ckcu_pllcfgr) module"]
pub type CKCU_PLLCFGR = crate::Reg<u32, _CKCU_PLLCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_PLLCFGR;
#[doc = "`read()` method returns [ckcu_pllcfgr::R](ckcu_pllcfgr::R) reader structure"]
impl crate::Readable for CKCU_PLLCFGR {}
#[doc = "`write(|w| ..)` method takes [ckcu_pllcfgr::W](ckcu_pllcfgr::W) writer structure"]
impl crate::Writable for CKCU_PLLCFGR {}
#[doc = "CKCU_PLLCFGR"]
pub mod ckcu_pllcfgr;
#[doc = "CKCU_PLLCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_pllcr](ckcu_pllcr) module"]
pub type CKCU_PLLCR = crate::Reg<u32, _CKCU_PLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_PLLCR;
#[doc = "`read()` method returns [ckcu_pllcr::R](ckcu_pllcr::R) reader structure"]
impl crate::Readable for CKCU_PLLCR {}
#[doc = "`write(|w| ..)` method takes [ckcu_pllcr::W](ckcu_pllcr::W) writer structure"]
impl crate::Writable for CKCU_PLLCR {}
#[doc = "CKCU_PLLCR"]
pub mod ckcu_pllcr;
#[doc = "CKCU_AHBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_ahbcfgr](ckcu_ahbcfgr) module"]
pub type CKCU_AHBCFGR = crate::Reg<u32, _CKCU_AHBCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_AHBCFGR;
#[doc = "`read()` method returns [ckcu_ahbcfgr::R](ckcu_ahbcfgr::R) reader structure"]
impl crate::Readable for CKCU_AHBCFGR {}
#[doc = "`write(|w| ..)` method takes [ckcu_ahbcfgr::W](ckcu_ahbcfgr::W) writer structure"]
impl crate::Writable for CKCU_AHBCFGR {}
#[doc = "CKCU_AHBCFGR"]
pub mod ckcu_ahbcfgr;
#[doc = "CKCU_AHBCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_ahbccr](ckcu_ahbccr) module"]
pub type CKCU_AHBCCR = crate::Reg<u32, _CKCU_AHBCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_AHBCCR;
#[doc = "`read()` method returns [ckcu_ahbccr::R](ckcu_ahbccr::R) reader structure"]
impl crate::Readable for CKCU_AHBCCR {}
#[doc = "`write(|w| ..)` method takes [ckcu_ahbccr::W](ckcu_ahbccr::W) writer structure"]
impl crate::Writable for CKCU_AHBCCR {}
#[doc = "CKCU_AHBCCR"]
pub mod ckcu_ahbccr;
#[doc = "CKCU_APBCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbcfgr](ckcu_apbcfgr) module"]
pub type CKCU_APBCFGR = crate::Reg<u32, _CKCU_APBCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_APBCFGR;
#[doc = "`read()` method returns [ckcu_apbcfgr::R](ckcu_apbcfgr::R) reader structure"]
impl crate::Readable for CKCU_APBCFGR {}
#[doc = "`write(|w| ..)` method takes [ckcu_apbcfgr::W](ckcu_apbcfgr::W) writer structure"]
impl crate::Writable for CKCU_APBCFGR {}
#[doc = "CKCU_APBCFGR"]
pub mod ckcu_apbcfgr;
#[doc = "CKCU_APBCCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbccr0](ckcu_apbccr0) module"]
pub type CKCU_APBCCR0 = crate::Reg<u32, _CKCU_APBCCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_APBCCR0;
#[doc = "`read()` method returns [ckcu_apbccr0::R](ckcu_apbccr0::R) reader structure"]
impl crate::Readable for CKCU_APBCCR0 {}
#[doc = "`write(|w| ..)` method takes [ckcu_apbccr0::W](ckcu_apbccr0::W) writer structure"]
impl crate::Writable for CKCU_APBCCR0 {}
#[doc = "CKCU_APBCCR0"]
pub mod ckcu_apbccr0;
#[doc = "CKCU_APBCCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbccr1](ckcu_apbccr1) module"]
pub type CKCU_APBCCR1 = crate::Reg<u32, _CKCU_APBCCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_APBCCR1;
#[doc = "`read()` method returns [ckcu_apbccr1::R](ckcu_apbccr1::R) reader structure"]
impl crate::Readable for CKCU_APBCCR1 {}
#[doc = "`write(|w| ..)` method takes [ckcu_apbccr1::W](ckcu_apbccr1::W) writer structure"]
impl crate::Writable for CKCU_APBCCR1 {}
#[doc = "CKCU_APBCCR1"]
pub mod ckcu_apbccr1;
#[doc = "CKCU_CKST\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_ckst](ckcu_ckst) module"]
pub type CKCU_CKST = crate::Reg<u32, _CKCU_CKST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_CKST;
#[doc = "`read()` method returns [ckcu_ckst::R](ckcu_ckst::R) reader structure"]
impl crate::Readable for CKCU_CKST {}
#[doc = "`write(|w| ..)` method takes [ckcu_ckst::W](ckcu_ckst::W) writer structure"]
impl crate::Writable for CKCU_CKST {}
#[doc = "CKCU_CKST"]
pub mod ckcu_ckst;
#[doc = "CKCU_APBPCSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbpcsr0](ckcu_apbpcsr0) module"]
pub type CKCU_APBPCSR0 = crate::Reg<u32, _CKCU_APBPCSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_APBPCSR0;
#[doc = "`read()` method returns [ckcu_apbpcsr0::R](ckcu_apbpcsr0::R) reader structure"]
impl crate::Readable for CKCU_APBPCSR0 {}
#[doc = "`write(|w| ..)` method takes [ckcu_apbpcsr0::W](ckcu_apbpcsr0::W) writer structure"]
impl crate::Writable for CKCU_APBPCSR0 {}
#[doc = "CKCU_APBPCSR0"]
pub mod ckcu_apbpcsr0;
#[doc = "CKCU_APBPCSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_apbpcsr1](ckcu_apbpcsr1) module"]
pub type CKCU_APBPCSR1 = crate::Reg<u32, _CKCU_APBPCSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_APBPCSR1;
#[doc = "`read()` method returns [ckcu_apbpcsr1::R](ckcu_apbpcsr1::R) reader structure"]
impl crate::Readable for CKCU_APBPCSR1 {}
#[doc = "`write(|w| ..)` method takes [ckcu_apbpcsr1::W](ckcu_apbpcsr1::W) writer structure"]
impl crate::Writable for CKCU_APBPCSR1 {}
#[doc = "CKCU_APBPCSR1"]
pub mod ckcu_apbpcsr1;
#[doc = "CKCU_HSICR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_hsicr](ckcu_hsicr) module"]
pub type CKCU_HSICR = crate::Reg<u32, _CKCU_HSICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_HSICR;
#[doc = "`read()` method returns [ckcu_hsicr::R](ckcu_hsicr::R) reader structure"]
impl crate::Readable for CKCU_HSICR {}
#[doc = "`write(|w| ..)` method takes [ckcu_hsicr::W](ckcu_hsicr::W) writer structure"]
impl crate::Writable for CKCU_HSICR {}
#[doc = "CKCU_HSICR"]
pub mod ckcu_hsicr;
#[doc = "CKCU_HSIATCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_hsiatcr](ckcu_hsiatcr) module"]
pub type CKCU_HSIATCR = crate::Reg<u32, _CKCU_HSIATCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_HSIATCR;
#[doc = "`read()` method returns [ckcu_hsiatcr::R](ckcu_hsiatcr::R) reader structure"]
impl crate::Readable for CKCU_HSIATCR {}
#[doc = "`write(|w| ..)` method takes [ckcu_hsiatcr::W](ckcu_hsiatcr::W) writer structure"]
impl crate::Writable for CKCU_HSIATCR {}
#[doc = "CKCU_HSIATCR"]
pub mod ckcu_hsiatcr;
#[doc = "CKCU_LPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_lpcr](ckcu_lpcr) module"]
pub type CKCU_LPCR = crate::Reg<u32, _CKCU_LPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_LPCR;
#[doc = "`read()` method returns [ckcu_lpcr::R](ckcu_lpcr::R) reader structure"]
impl crate::Readable for CKCU_LPCR {}
#[doc = "`write(|w| ..)` method takes [ckcu_lpcr::W](ckcu_lpcr::W) writer structure"]
impl crate::Writable for CKCU_LPCR {}
#[doc = "CKCU_LPCR"]
pub mod ckcu_lpcr;
#[doc = "CKCU_MCUDBGCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_mcudbgcr](ckcu_mcudbgcr) module"]
pub type CKCU_MCUDBGCR = crate::Reg<u32, _CKCU_MCUDBGCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CKCU_MCUDBGCR;
#[doc = "`read()` method returns [ckcu_mcudbgcr::R](ckcu_mcudbgcr::R) reader structure"]
impl crate::Readable for CKCU_MCUDBGCR {}
#[doc = "`write(|w| ..)` method takes [ckcu_mcudbgcr::W](ckcu_mcudbgcr::W) writer structure"]
impl crate::Writable for CKCU_MCUDBGCR {}
#[doc = "CKCU_MCUDBGCR"]
pub mod ckcu_mcudbgcr;
