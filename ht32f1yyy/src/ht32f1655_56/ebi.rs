#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - EBI_CR"]
    pub ebi_cr: EBI_CR,
    #[doc = "0x04 - EBI_PCR"]
    pub ebi_pcr: EBI_PCR,
    #[doc = "0x08 - EBI_SR"]
    pub ebi_sr: EBI_SR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - EBI_ATR0"]
    pub ebi_atr0: EBI_ATR0,
    #[doc = "0x14 - EBI_RTR0"]
    pub ebi_rtr0: EBI_RTR0,
    #[doc = "0x18 - EBI_WTR0"]
    pub ebi_wtr0: EBI_WTR0,
    #[doc = "0x1c - EBI_PR0"]
    pub ebi_pr0: EBI_PR0,
    #[doc = "0x20 - EBI_ATR1"]
    pub ebi_atr1: EBI_ATR1,
    #[doc = "0x24 - EBI_RTR1"]
    pub ebi_rtr1: EBI_RTR1,
    #[doc = "0x28 - EBI_WTR1"]
    pub ebi_wtr1: EBI_WTR1,
    #[doc = "0x2c - EBI_PR1"]
    pub ebi_pr1: EBI_PR1,
    #[doc = "0x30 - EBI_ATR2"]
    pub ebi_atr2: EBI_ATR2,
    #[doc = "0x34 - EBI_RTR2"]
    pub ebi_rtr2: EBI_RTR2,
    #[doc = "0x38 - EBI_WTR2"]
    pub ebi_wtr2: EBI_WTR2,
    #[doc = "0x3c - EBI_PR2"]
    pub ebi_pr2: EBI_PR2,
    #[doc = "0x40 - EBI_ATR3"]
    pub ebi_atr3: EBI_ATR3,
    #[doc = "0x44 - EBI_RTR3"]
    pub ebi_rtr3: EBI_RTR3,
    #[doc = "0x48 - EBI_WTR3"]
    pub ebi_wtr3: EBI_WTR3,
    #[doc = "0x4c - EBI_PR3"]
    pub ebi_pr3: EBI_PR3,
    #[doc = "0x50 - EBI_IENR"]
    pub ebi_ienr: EBI_IENR,
    #[doc = "0x54 - EBI_IFR"]
    pub ebi_ifr: EBI_IFR,
    #[doc = "0x58 - EBI_IFCR"]
    pub ebi_ifcr: EBI_IFCR,
}
#[doc = "EBI_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_cr](ebi_cr) module"]
pub type EBI_CR = crate::Reg<u32, _EBI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_CR;
#[doc = "`read()` method returns [ebi_cr::R](ebi_cr::R) reader structure"]
impl crate::Readable for EBI_CR {}
#[doc = "`write(|w| ..)` method takes [ebi_cr::W](ebi_cr::W) writer structure"]
impl crate::Writable for EBI_CR {}
#[doc = "EBI_CR"]
pub mod ebi_cr;
#[doc = "EBI_PCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_pcr](ebi_pcr) module"]
pub type EBI_PCR = crate::Reg<u32, _EBI_PCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_PCR;
#[doc = "`read()` method returns [ebi_pcr::R](ebi_pcr::R) reader structure"]
impl crate::Readable for EBI_PCR {}
#[doc = "`write(|w| ..)` method takes [ebi_pcr::W](ebi_pcr::W) writer structure"]
impl crate::Writable for EBI_PCR {}
#[doc = "EBI_PCR"]
pub mod ebi_pcr;
#[doc = "EBI_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_sr](ebi_sr) module"]
pub type EBI_SR = crate::Reg<u32, _EBI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_SR;
#[doc = "`read()` method returns [ebi_sr::R](ebi_sr::R) reader structure"]
impl crate::Readable for EBI_SR {}
#[doc = "`write(|w| ..)` method takes [ebi_sr::W](ebi_sr::W) writer structure"]
impl crate::Writable for EBI_SR {}
#[doc = "EBI_SR"]
pub mod ebi_sr;
#[doc = "EBI_ATR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_atr0](ebi_atr0) module"]
pub type EBI_ATR0 = crate::Reg<u32, _EBI_ATR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_ATR0;
#[doc = "`read()` method returns [ebi_atr0::R](ebi_atr0::R) reader structure"]
impl crate::Readable for EBI_ATR0 {}
#[doc = "`write(|w| ..)` method takes [ebi_atr0::W](ebi_atr0::W) writer structure"]
impl crate::Writable for EBI_ATR0 {}
#[doc = "EBI_ATR0"]
pub mod ebi_atr0;
#[doc = "EBI_ATR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_atr1](ebi_atr1) module"]
pub type EBI_ATR1 = crate::Reg<u32, _EBI_ATR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_ATR1;
#[doc = "`read()` method returns [ebi_atr1::R](ebi_atr1::R) reader structure"]
impl crate::Readable for EBI_ATR1 {}
#[doc = "`write(|w| ..)` method takes [ebi_atr1::W](ebi_atr1::W) writer structure"]
impl crate::Writable for EBI_ATR1 {}
#[doc = "EBI_ATR1"]
pub mod ebi_atr1;
#[doc = "EBI_ATR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_atr2](ebi_atr2) module"]
pub type EBI_ATR2 = crate::Reg<u32, _EBI_ATR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_ATR2;
#[doc = "`read()` method returns [ebi_atr2::R](ebi_atr2::R) reader structure"]
impl crate::Readable for EBI_ATR2 {}
#[doc = "`write(|w| ..)` method takes [ebi_atr2::W](ebi_atr2::W) writer structure"]
impl crate::Writable for EBI_ATR2 {}
#[doc = "EBI_ATR2"]
pub mod ebi_atr2;
#[doc = "EBI_ATR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_atr3](ebi_atr3) module"]
pub type EBI_ATR3 = crate::Reg<u32, _EBI_ATR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_ATR3;
#[doc = "`read()` method returns [ebi_atr3::R](ebi_atr3::R) reader structure"]
impl crate::Readable for EBI_ATR3 {}
#[doc = "`write(|w| ..)` method takes [ebi_atr3::W](ebi_atr3::W) writer structure"]
impl crate::Writable for EBI_ATR3 {}
#[doc = "EBI_ATR3"]
pub mod ebi_atr3;
#[doc = "EBI_RTR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_rtr0](ebi_rtr0) module"]
pub type EBI_RTR0 = crate::Reg<u32, _EBI_RTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_RTR0;
#[doc = "`read()` method returns [ebi_rtr0::R](ebi_rtr0::R) reader structure"]
impl crate::Readable for EBI_RTR0 {}
#[doc = "`write(|w| ..)` method takes [ebi_rtr0::W](ebi_rtr0::W) writer structure"]
impl crate::Writable for EBI_RTR0 {}
#[doc = "EBI_RTR0"]
pub mod ebi_rtr0;
#[doc = "EBI_RTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_rtr1](ebi_rtr1) module"]
pub type EBI_RTR1 = crate::Reg<u32, _EBI_RTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_RTR1;
#[doc = "`read()` method returns [ebi_rtr1::R](ebi_rtr1::R) reader structure"]
impl crate::Readable for EBI_RTR1 {}
#[doc = "`write(|w| ..)` method takes [ebi_rtr1::W](ebi_rtr1::W) writer structure"]
impl crate::Writable for EBI_RTR1 {}
#[doc = "EBI_RTR1"]
pub mod ebi_rtr1;
#[doc = "EBI_RTR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_rtr2](ebi_rtr2) module"]
pub type EBI_RTR2 = crate::Reg<u32, _EBI_RTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_RTR2;
#[doc = "`read()` method returns [ebi_rtr2::R](ebi_rtr2::R) reader structure"]
impl crate::Readable for EBI_RTR2 {}
#[doc = "`write(|w| ..)` method takes [ebi_rtr2::W](ebi_rtr2::W) writer structure"]
impl crate::Writable for EBI_RTR2 {}
#[doc = "EBI_RTR2"]
pub mod ebi_rtr2;
#[doc = "EBI_RTR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_rtr3](ebi_rtr3) module"]
pub type EBI_RTR3 = crate::Reg<u32, _EBI_RTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_RTR3;
#[doc = "`read()` method returns [ebi_rtr3::R](ebi_rtr3::R) reader structure"]
impl crate::Readable for EBI_RTR3 {}
#[doc = "`write(|w| ..)` method takes [ebi_rtr3::W](ebi_rtr3::W) writer structure"]
impl crate::Writable for EBI_RTR3 {}
#[doc = "EBI_RTR3"]
pub mod ebi_rtr3;
#[doc = "EBI_WTR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_wtr0](ebi_wtr0) module"]
pub type EBI_WTR0 = crate::Reg<u32, _EBI_WTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_WTR0;
#[doc = "`read()` method returns [ebi_wtr0::R](ebi_wtr0::R) reader structure"]
impl crate::Readable for EBI_WTR0 {}
#[doc = "`write(|w| ..)` method takes [ebi_wtr0::W](ebi_wtr0::W) writer structure"]
impl crate::Writable for EBI_WTR0 {}
#[doc = "EBI_WTR0"]
pub mod ebi_wtr0;
#[doc = "EBI_WTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_wtr1](ebi_wtr1) module"]
pub type EBI_WTR1 = crate::Reg<u32, _EBI_WTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_WTR1;
#[doc = "`read()` method returns [ebi_wtr1::R](ebi_wtr1::R) reader structure"]
impl crate::Readable for EBI_WTR1 {}
#[doc = "`write(|w| ..)` method takes [ebi_wtr1::W](ebi_wtr1::W) writer structure"]
impl crate::Writable for EBI_WTR1 {}
#[doc = "EBI_WTR1"]
pub mod ebi_wtr1;
#[doc = "EBI_WTR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_wtr2](ebi_wtr2) module"]
pub type EBI_WTR2 = crate::Reg<u32, _EBI_WTR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_WTR2;
#[doc = "`read()` method returns [ebi_wtr2::R](ebi_wtr2::R) reader structure"]
impl crate::Readable for EBI_WTR2 {}
#[doc = "`write(|w| ..)` method takes [ebi_wtr2::W](ebi_wtr2::W) writer structure"]
impl crate::Writable for EBI_WTR2 {}
#[doc = "EBI_WTR2"]
pub mod ebi_wtr2;
#[doc = "EBI_WTR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_wtr3](ebi_wtr3) module"]
pub type EBI_WTR3 = crate::Reg<u32, _EBI_WTR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_WTR3;
#[doc = "`read()` method returns [ebi_wtr3::R](ebi_wtr3::R) reader structure"]
impl crate::Readable for EBI_WTR3 {}
#[doc = "`write(|w| ..)` method takes [ebi_wtr3::W](ebi_wtr3::W) writer structure"]
impl crate::Writable for EBI_WTR3 {}
#[doc = "EBI_WTR3"]
pub mod ebi_wtr3;
#[doc = "EBI_PR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_pr0](ebi_pr0) module"]
pub type EBI_PR0 = crate::Reg<u32, _EBI_PR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_PR0;
#[doc = "`read()` method returns [ebi_pr0::R](ebi_pr0::R) reader structure"]
impl crate::Readable for EBI_PR0 {}
#[doc = "`write(|w| ..)` method takes [ebi_pr0::W](ebi_pr0::W) writer structure"]
impl crate::Writable for EBI_PR0 {}
#[doc = "EBI_PR0"]
pub mod ebi_pr0;
#[doc = "EBI_PR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_pr1](ebi_pr1) module"]
pub type EBI_PR1 = crate::Reg<u32, _EBI_PR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_PR1;
#[doc = "`read()` method returns [ebi_pr1::R](ebi_pr1::R) reader structure"]
impl crate::Readable for EBI_PR1 {}
#[doc = "`write(|w| ..)` method takes [ebi_pr1::W](ebi_pr1::W) writer structure"]
impl crate::Writable for EBI_PR1 {}
#[doc = "EBI_PR1"]
pub mod ebi_pr1;
#[doc = "EBI_PR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_pr2](ebi_pr2) module"]
pub type EBI_PR2 = crate::Reg<u32, _EBI_PR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_PR2;
#[doc = "`read()` method returns [ebi_pr2::R](ebi_pr2::R) reader structure"]
impl crate::Readable for EBI_PR2 {}
#[doc = "`write(|w| ..)` method takes [ebi_pr2::W](ebi_pr2::W) writer structure"]
impl crate::Writable for EBI_PR2 {}
#[doc = "EBI_PR2"]
pub mod ebi_pr2;
#[doc = "EBI_PR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_pr3](ebi_pr3) module"]
pub type EBI_PR3 = crate::Reg<u32, _EBI_PR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_PR3;
#[doc = "`read()` method returns [ebi_pr3::R](ebi_pr3::R) reader structure"]
impl crate::Readable for EBI_PR3 {}
#[doc = "`write(|w| ..)` method takes [ebi_pr3::W](ebi_pr3::W) writer structure"]
impl crate::Writable for EBI_PR3 {}
#[doc = "EBI_PR3"]
pub mod ebi_pr3;
#[doc = "EBI_IENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_ienr](ebi_ienr) module"]
pub type EBI_IENR = crate::Reg<u32, _EBI_IENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_IENR;
#[doc = "`read()` method returns [ebi_ienr::R](ebi_ienr::R) reader structure"]
impl crate::Readable for EBI_IENR {}
#[doc = "`write(|w| ..)` method takes [ebi_ienr::W](ebi_ienr::W) writer structure"]
impl crate::Writable for EBI_IENR {}
#[doc = "EBI_IENR"]
pub mod ebi_ienr;
#[doc = "EBI_IFR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_ifr](ebi_ifr) module"]
pub type EBI_IFR = crate::Reg<u32, _EBI_IFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_IFR;
#[doc = "`read()` method returns [ebi_ifr::R](ebi_ifr::R) reader structure"]
impl crate::Readable for EBI_IFR {}
#[doc = "`write(|w| ..)` method takes [ebi_ifr::W](ebi_ifr::W) writer structure"]
impl crate::Writable for EBI_IFR {}
#[doc = "EBI_IFR"]
pub mod ebi_ifr;
#[doc = "EBI_IFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebi_ifcr](ebi_ifcr) module"]
pub type EBI_IFCR = crate::Reg<u32, _EBI_IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EBI_IFCR;
#[doc = "`read()` method returns [ebi_ifcr::R](ebi_ifcr::R) reader structure"]
impl crate::Readable for EBI_IFCR {}
#[doc = "`write(|w| ..)` method takes [ebi_ifcr::W](ebi_ifcr::W) writer structure"]
impl crate::Writable for EBI_IFCR {}
#[doc = "EBI_IFCR"]
pub mod ebi_ifcr;
