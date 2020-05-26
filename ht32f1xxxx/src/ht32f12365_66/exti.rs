#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EXTI_CFGR0"]
    pub exti_cfgr0: EXTI_CFGR0,
    #[doc = "0x04 - EXTI_CFGR1"]
    pub exti_cfgr1: EXTI_CFGR1,
    #[doc = "0x08 - EXTI_CFGR2"]
    pub exti_cfgr2: EXTI_CFGR2,
    #[doc = "0x0c - EXTI_CFGR3"]
    pub exti_cfgr3: EXTI_CFGR3,
    #[doc = "0x10 - EXTI_CFGR4"]
    pub exti_cfgr4: EXTI_CFGR4,
    #[doc = "0x14 - EXTI_CFGR5"]
    pub exti_cfgr5: EXTI_CFGR5,
    #[doc = "0x18 - EXTI_CFGR6"]
    pub exti_cfgr6: EXTI_CFGR6,
    #[doc = "0x1c - EXTI_CFGR7"]
    pub exti_cfgr7: EXTI_CFGR7,
    #[doc = "0x20 - EXTI_CFGR8"]
    pub exti_cfgr8: EXTI_CFGR8,
    #[doc = "0x24 - EXTI_CFGR9"]
    pub exti_cfgr9: EXTI_CFGR9,
    #[doc = "0x28 - EXTI_CFGR10"]
    pub exti_cfgr10: EXTI_CFGR10,
    #[doc = "0x2c - EXTI_CFGR11"]
    pub exti_cfgr11: EXTI_CFGR11,
    #[doc = "0x30 - EXTI_CFGR12"]
    pub exti_cfgr12: EXTI_CFGR12,
    #[doc = "0x34 - EXTI_CFGR13"]
    pub exti_cfgr13: EXTI_CFGR13,
    #[doc = "0x38 - EXTI_CFGR14"]
    pub exti_cfgr14: EXTI_CFGR14,
    #[doc = "0x3c - EXTI_CFGR15"]
    pub exti_cfgr15: EXTI_CFGR15,
    #[doc = "0x40 - EXTI_CR"]
    pub exti_cr: EXTI_CR,
    #[doc = "0x44 - EXTI_EDGEFLGR"]
    pub exti_edgeflgr: EXTI_EDGEFLGR,
    #[doc = "0x48 - EXTI_EDGESR"]
    pub exti_edgesr: EXTI_EDGESR,
    #[doc = "0x4c - EXTI_SSCR"]
    pub exti_sscr: EXTI_SSCR,
    #[doc = "0x50 - EXTI_WAKUPCR"]
    pub exti_wakupcr: EXTI_WAKUPCR,
    #[doc = "0x54 - EXTI_WAKUPPOLR"]
    pub exti_wakuppolr: EXTI_WAKUPPOLR,
    #[doc = "0x58 - EXTI_WAKUPFLG"]
    pub exti_wakupflg: EXTI_WAKUPFLG,
}
#[doc = "EXTI_CFGR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr0](exti_cfgr0) module"]
pub type EXTI_CFGR0 = crate::Reg<u32, _EXTI_CFGR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR0;
#[doc = "`read()` method returns [exti_cfgr0::R](exti_cfgr0::R) reader structure"]
impl crate::Readable for EXTI_CFGR0 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr0::W](exti_cfgr0::W) writer structure"]
impl crate::Writable for EXTI_CFGR0 {}
#[doc = "EXTI_CFGR0"]
pub mod exti_cfgr0;
#[doc = "EXTI_CFGR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr1](exti_cfgr1) module"]
pub type EXTI_CFGR1 = crate::Reg<u32, _EXTI_CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR1;
#[doc = "`read()` method returns [exti_cfgr1::R](exti_cfgr1::R) reader structure"]
impl crate::Readable for EXTI_CFGR1 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr1::W](exti_cfgr1::W) writer structure"]
impl crate::Writable for EXTI_CFGR1 {}
#[doc = "EXTI_CFGR1"]
pub mod exti_cfgr1;
#[doc = "EXTI_CFGR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr2](exti_cfgr2) module"]
pub type EXTI_CFGR2 = crate::Reg<u32, _EXTI_CFGR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR2;
#[doc = "`read()` method returns [exti_cfgr2::R](exti_cfgr2::R) reader structure"]
impl crate::Readable for EXTI_CFGR2 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr2::W](exti_cfgr2::W) writer structure"]
impl crate::Writable for EXTI_CFGR2 {}
#[doc = "EXTI_CFGR2"]
pub mod exti_cfgr2;
#[doc = "EXTI_CFGR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr3](exti_cfgr3) module"]
pub type EXTI_CFGR3 = crate::Reg<u32, _EXTI_CFGR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR3;
#[doc = "`read()` method returns [exti_cfgr3::R](exti_cfgr3::R) reader structure"]
impl crate::Readable for EXTI_CFGR3 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr3::W](exti_cfgr3::W) writer structure"]
impl crate::Writable for EXTI_CFGR3 {}
#[doc = "EXTI_CFGR3"]
pub mod exti_cfgr3;
#[doc = "EXTI_CFGR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr4](exti_cfgr4) module"]
pub type EXTI_CFGR4 = crate::Reg<u32, _EXTI_CFGR4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR4;
#[doc = "`read()` method returns [exti_cfgr4::R](exti_cfgr4::R) reader structure"]
impl crate::Readable for EXTI_CFGR4 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr4::W](exti_cfgr4::W) writer structure"]
impl crate::Writable for EXTI_CFGR4 {}
#[doc = "EXTI_CFGR4"]
pub mod exti_cfgr4;
#[doc = "EXTI_CFGR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr5](exti_cfgr5) module"]
pub type EXTI_CFGR5 = crate::Reg<u32, _EXTI_CFGR5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR5;
#[doc = "`read()` method returns [exti_cfgr5::R](exti_cfgr5::R) reader structure"]
impl crate::Readable for EXTI_CFGR5 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr5::W](exti_cfgr5::W) writer structure"]
impl crate::Writable for EXTI_CFGR5 {}
#[doc = "EXTI_CFGR5"]
pub mod exti_cfgr5;
#[doc = "EXTI_CFGR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr6](exti_cfgr6) module"]
pub type EXTI_CFGR6 = crate::Reg<u32, _EXTI_CFGR6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR6;
#[doc = "`read()` method returns [exti_cfgr6::R](exti_cfgr6::R) reader structure"]
impl crate::Readable for EXTI_CFGR6 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr6::W](exti_cfgr6::W) writer structure"]
impl crate::Writable for EXTI_CFGR6 {}
#[doc = "EXTI_CFGR6"]
pub mod exti_cfgr6;
#[doc = "EXTI_CFGR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr7](exti_cfgr7) module"]
pub type EXTI_CFGR7 = crate::Reg<u32, _EXTI_CFGR7>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR7;
#[doc = "`read()` method returns [exti_cfgr7::R](exti_cfgr7::R) reader structure"]
impl crate::Readable for EXTI_CFGR7 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr7::W](exti_cfgr7::W) writer structure"]
impl crate::Writable for EXTI_CFGR7 {}
#[doc = "EXTI_CFGR7"]
pub mod exti_cfgr7;
#[doc = "EXTI_CFGR8\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr8](exti_cfgr8) module"]
pub type EXTI_CFGR8 = crate::Reg<u32, _EXTI_CFGR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR8;
#[doc = "`read()` method returns [exti_cfgr8::R](exti_cfgr8::R) reader structure"]
impl crate::Readable for EXTI_CFGR8 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr8::W](exti_cfgr8::W) writer structure"]
impl crate::Writable for EXTI_CFGR8 {}
#[doc = "EXTI_CFGR8"]
pub mod exti_cfgr8;
#[doc = "EXTI_CFGR9\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr9](exti_cfgr9) module"]
pub type EXTI_CFGR9 = crate::Reg<u32, _EXTI_CFGR9>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR9;
#[doc = "`read()` method returns [exti_cfgr9::R](exti_cfgr9::R) reader structure"]
impl crate::Readable for EXTI_CFGR9 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr9::W](exti_cfgr9::W) writer structure"]
impl crate::Writable for EXTI_CFGR9 {}
#[doc = "EXTI_CFGR9"]
pub mod exti_cfgr9;
#[doc = "EXTI_CFGR10\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr10](exti_cfgr10) module"]
pub type EXTI_CFGR10 = crate::Reg<u32, _EXTI_CFGR10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR10;
#[doc = "`read()` method returns [exti_cfgr10::R](exti_cfgr10::R) reader structure"]
impl crate::Readable for EXTI_CFGR10 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr10::W](exti_cfgr10::W) writer structure"]
impl crate::Writable for EXTI_CFGR10 {}
#[doc = "EXTI_CFGR10"]
pub mod exti_cfgr10;
#[doc = "EXTI_CFGR11\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr11](exti_cfgr11) module"]
pub type EXTI_CFGR11 = crate::Reg<u32, _EXTI_CFGR11>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR11;
#[doc = "`read()` method returns [exti_cfgr11::R](exti_cfgr11::R) reader structure"]
impl crate::Readable for EXTI_CFGR11 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr11::W](exti_cfgr11::W) writer structure"]
impl crate::Writable for EXTI_CFGR11 {}
#[doc = "EXTI_CFGR11"]
pub mod exti_cfgr11;
#[doc = "EXTI_CFGR12\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr12](exti_cfgr12) module"]
pub type EXTI_CFGR12 = crate::Reg<u32, _EXTI_CFGR12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR12;
#[doc = "`read()` method returns [exti_cfgr12::R](exti_cfgr12::R) reader structure"]
impl crate::Readable for EXTI_CFGR12 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr12::W](exti_cfgr12::W) writer structure"]
impl crate::Writable for EXTI_CFGR12 {}
#[doc = "EXTI_CFGR12"]
pub mod exti_cfgr12;
#[doc = "EXTI_CFGR13\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr13](exti_cfgr13) module"]
pub type EXTI_CFGR13 = crate::Reg<u32, _EXTI_CFGR13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR13;
#[doc = "`read()` method returns [exti_cfgr13::R](exti_cfgr13::R) reader structure"]
impl crate::Readable for EXTI_CFGR13 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr13::W](exti_cfgr13::W) writer structure"]
impl crate::Writable for EXTI_CFGR13 {}
#[doc = "EXTI_CFGR13"]
pub mod exti_cfgr13;
#[doc = "EXTI_CFGR14\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr14](exti_cfgr14) module"]
pub type EXTI_CFGR14 = crate::Reg<u32, _EXTI_CFGR14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR14;
#[doc = "`read()` method returns [exti_cfgr14::R](exti_cfgr14::R) reader structure"]
impl crate::Readable for EXTI_CFGR14 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr14::W](exti_cfgr14::W) writer structure"]
impl crate::Writable for EXTI_CFGR14 {}
#[doc = "EXTI_CFGR14"]
pub mod exti_cfgr14;
#[doc = "EXTI_CFGR15\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cfgr15](exti_cfgr15) module"]
pub type EXTI_CFGR15 = crate::Reg<u32, _EXTI_CFGR15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CFGR15;
#[doc = "`read()` method returns [exti_cfgr15::R](exti_cfgr15::R) reader structure"]
impl crate::Readable for EXTI_CFGR15 {}
#[doc = "`write(|w| ..)` method takes [exti_cfgr15::W](exti_cfgr15::W) writer structure"]
impl crate::Writable for EXTI_CFGR15 {}
#[doc = "EXTI_CFGR15"]
pub mod exti_cfgr15;
#[doc = "EXTI_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_cr](exti_cr) module"]
pub type EXTI_CR = crate::Reg<u32, _EXTI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_CR;
#[doc = "`read()` method returns [exti_cr::R](exti_cr::R) reader structure"]
impl crate::Readable for EXTI_CR {}
#[doc = "`write(|w| ..)` method takes [exti_cr::W](exti_cr::W) writer structure"]
impl crate::Writable for EXTI_CR {}
#[doc = "EXTI_CR"]
pub mod exti_cr;
#[doc = "EXTI_EDGEFLGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_edgeflgr](exti_edgeflgr) module"]
pub type EXTI_EDGEFLGR = crate::Reg<u32, _EXTI_EDGEFLGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_EDGEFLGR;
#[doc = "`read()` method returns [exti_edgeflgr::R](exti_edgeflgr::R) reader structure"]
impl crate::Readable for EXTI_EDGEFLGR {}
#[doc = "`write(|w| ..)` method takes [exti_edgeflgr::W](exti_edgeflgr::W) writer structure"]
impl crate::Writable for EXTI_EDGEFLGR {}
#[doc = "EXTI_EDGEFLGR"]
pub mod exti_edgeflgr;
#[doc = "EXTI_EDGESR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_edgesr](exti_edgesr) module"]
pub type EXTI_EDGESR = crate::Reg<u32, _EXTI_EDGESR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_EDGESR;
#[doc = "`read()` method returns [exti_edgesr::R](exti_edgesr::R) reader structure"]
impl crate::Readable for EXTI_EDGESR {}
#[doc = "`write(|w| ..)` method takes [exti_edgesr::W](exti_edgesr::W) writer structure"]
impl crate::Writable for EXTI_EDGESR {}
#[doc = "EXTI_EDGESR"]
pub mod exti_edgesr;
#[doc = "EXTI_SSCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_sscr](exti_sscr) module"]
pub type EXTI_SSCR = crate::Reg<u32, _EXTI_SSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_SSCR;
#[doc = "`read()` method returns [exti_sscr::R](exti_sscr::R) reader structure"]
impl crate::Readable for EXTI_SSCR {}
#[doc = "`write(|w| ..)` method takes [exti_sscr::W](exti_sscr::W) writer structure"]
impl crate::Writable for EXTI_SSCR {}
#[doc = "EXTI_SSCR"]
pub mod exti_sscr;
#[doc = "EXTI_WAKUPCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_wakupcr](exti_wakupcr) module"]
pub type EXTI_WAKUPCR = crate::Reg<u32, _EXTI_WAKUPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_WAKUPCR;
#[doc = "`read()` method returns [exti_wakupcr::R](exti_wakupcr::R) reader structure"]
impl crate::Readable for EXTI_WAKUPCR {}
#[doc = "`write(|w| ..)` method takes [exti_wakupcr::W](exti_wakupcr::W) writer structure"]
impl crate::Writable for EXTI_WAKUPCR {}
#[doc = "EXTI_WAKUPCR"]
pub mod exti_wakupcr;
#[doc = "EXTI_WAKUPPOLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_wakuppolr](exti_wakuppolr) module"]
pub type EXTI_WAKUPPOLR = crate::Reg<u32, _EXTI_WAKUPPOLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_WAKUPPOLR;
#[doc = "`read()` method returns [exti_wakuppolr::R](exti_wakuppolr::R) reader structure"]
impl crate::Readable for EXTI_WAKUPPOLR {}
#[doc = "`write(|w| ..)` method takes [exti_wakuppolr::W](exti_wakuppolr::W) writer structure"]
impl crate::Writable for EXTI_WAKUPPOLR {}
#[doc = "EXTI_WAKUPPOLR"]
pub mod exti_wakuppolr;
#[doc = "EXTI_WAKUPFLG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_wakupflg](exti_wakupflg) module"]
pub type EXTI_WAKUPFLG = crate::Reg<u32, _EXTI_WAKUPFLG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTI_WAKUPFLG;
#[doc = "`read()` method returns [exti_wakupflg::R](exti_wakupflg::R) reader structure"]
impl crate::Readable for EXTI_WAKUPFLG {}
#[doc = "`write(|w| ..)` method takes [exti_wakupflg::W](exti_wakupflg::W) writer structure"]
impl crate::Writable for EXTI_WAKUPFLG {}
#[doc = "EXTI_WAKUPFLG"]
pub mod exti_wakupflg;
