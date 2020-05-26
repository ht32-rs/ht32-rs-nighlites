#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDMA_CH0CR"]
    pub pdma_ch0cr: PDMA_CH0CR,
    #[doc = "0x04 - PDMA_CH0SADR"]
    pub pdma_ch0sadr: PDMA_CH0SADR,
    #[doc = "0x08 - PDMA_CH0DADR"]
    pub pdma_ch0dadr: PDMA_CH0DADR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - PDMA_CH0TSR"]
    pub pdma_ch0tsr: PDMA_CH0TSR,
    #[doc = "0x14 - PDMA_CH0CTSR"]
    pub pdma_ch0ctsr: PDMA_CH0CTSR,
    #[doc = "0x18 - PDMA_CH1CR"]
    pub pdma_ch1cr: PDMA_CH1CR,
    #[doc = "0x1c - PDMA_CH1SADR"]
    pub pdma_ch1sadr: PDMA_CH1SADR,
    #[doc = "0x20 - PDMA_CH1DADR"]
    pub pdma_ch1dadr: PDMA_CH1DADR,
    _reserved8: [u8; 4usize],
    #[doc = "0x28 - PDMA_CH1TSR"]
    pub pdma_ch1tsr: PDMA_CH1TSR,
    #[doc = "0x2c - PDMA_CH1CTSR"]
    pub pdma_ch1ctsr: PDMA_CH1CTSR,
    #[doc = "0x30 - PDMA_CH2CR"]
    pub pdma_ch2cr: PDMA_CH2CR,
    #[doc = "0x34 - PDMA_CH2SADR"]
    pub pdma_ch2sadr: PDMA_CH2SADR,
    #[doc = "0x38 - PDMA_CH2DADR"]
    pub pdma_ch2dadr: PDMA_CH2DADR,
    _reserved13: [u8; 4usize],
    #[doc = "0x40 - PDMA_CH2TSR"]
    pub pdma_ch2tsr: PDMA_CH2TSR,
    #[doc = "0x44 - PDMA_CH2CTSR"]
    pub pdma_ch2ctsr: PDMA_CH2CTSR,
    #[doc = "0x48 - PDMA_CH3CR"]
    pub pdma_ch3cr: PDMA_CH3CR,
    #[doc = "0x4c - PDMA_CH3SADR"]
    pub pdma_ch3sadr: PDMA_CH3SADR,
    #[doc = "0x50 - PDMA_CH3DADR"]
    pub pdma_ch3dadr: PDMA_CH3DADR,
    _reserved18: [u8; 4usize],
    #[doc = "0x58 - PDMA_CH3TSR"]
    pub pdma_ch3tsr: PDMA_CH3TSR,
    #[doc = "0x5c - PDMA_CH3CTSR"]
    pub pdma_ch3ctsr: PDMA_CH3CTSR,
    #[doc = "0x60 - PDMA_CH4CR"]
    pub pdma_ch4cr: PDMA_CH4CR,
    #[doc = "0x64 - PDMA_CH4SADR"]
    pub pdma_ch4sadr: PDMA_CH4SADR,
    #[doc = "0x68 - PDMA_CH4DADR"]
    pub pdma_ch4dadr: PDMA_CH4DADR,
    _reserved23: [u8; 4usize],
    #[doc = "0x70 - PDMA_CH4TSR"]
    pub pdma_ch4tsr: PDMA_CH4TSR,
    #[doc = "0x74 - PDMA_CH4CTSR"]
    pub pdma_ch4ctsr: PDMA_CH4CTSR,
    #[doc = "0x78 - PDMA_CH5CR"]
    pub pdma_ch5cr: PDMA_CH5CR,
    #[doc = "0x7c - PDMA_CH5SADR"]
    pub pdma_ch5sadr: PDMA_CH5SADR,
    #[doc = "0x80 - PDMA_CH5DADR"]
    pub pdma_ch5dadr: PDMA_CH5DADR,
    _reserved28: [u8; 4usize],
    #[doc = "0x88 - PDMA_CH5TSR"]
    pub pdma_ch5tsr: PDMA_CH5TSR,
    #[doc = "0x8c - PDMA_CH5CTSR"]
    pub pdma_ch5ctsr: PDMA_CH5CTSR,
    _reserved30: [u8; 144usize],
    #[doc = "0x120 - PDMA_ISR"]
    pub pdma_isr: PDMA_ISR,
    _reserved31: [u8; 4usize],
    #[doc = "0x128 - PDMA_ISCR"]
    pub pdma_iscr: PDMA_ISCR,
    _reserved32: [u8; 4usize],
    #[doc = "0x130 - PDMA_IER"]
    pub pdma_ier: PDMA_IER,
}
#[doc = "PDMA_CH0CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch0cr](pdma_ch0cr) module"]
pub type PDMA_CH0CR = crate::Reg<u32, _PDMA_CH0CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH0CR;
#[doc = "`read()` method returns [pdma_ch0cr::R](pdma_ch0cr::R) reader structure"]
impl crate::Readable for PDMA_CH0CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch0cr::W](pdma_ch0cr::W) writer structure"]
impl crate::Writable for PDMA_CH0CR {}
#[doc = "PDMA_CH0CR"]
pub mod pdma_ch0cr;
#[doc = "PDMA_CH0SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch0sadr](pdma_ch0sadr) module"]
pub type PDMA_CH0SADR = crate::Reg<u32, _PDMA_CH0SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH0SADR;
#[doc = "`read()` method returns [pdma_ch0sadr::R](pdma_ch0sadr::R) reader structure"]
impl crate::Readable for PDMA_CH0SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch0sadr::W](pdma_ch0sadr::W) writer structure"]
impl crate::Writable for PDMA_CH0SADR {}
#[doc = "PDMA_CH0SADR"]
pub mod pdma_ch0sadr;
#[doc = "PDMA_CH0DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch0dadr](pdma_ch0dadr) module"]
pub type PDMA_CH0DADR = crate::Reg<u32, _PDMA_CH0DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH0DADR;
#[doc = "`read()` method returns [pdma_ch0dadr::R](pdma_ch0dadr::R) reader structure"]
impl crate::Readable for PDMA_CH0DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch0dadr::W](pdma_ch0dadr::W) writer structure"]
impl crate::Writable for PDMA_CH0DADR {}
#[doc = "PDMA_CH0DADR"]
pub mod pdma_ch0dadr;
#[doc = "PDMA_CH0TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch0tsr](pdma_ch0tsr) module"]
pub type PDMA_CH0TSR = crate::Reg<u32, _PDMA_CH0TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH0TSR;
#[doc = "`read()` method returns [pdma_ch0tsr::R](pdma_ch0tsr::R) reader structure"]
impl crate::Readable for PDMA_CH0TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch0tsr::W](pdma_ch0tsr::W) writer structure"]
impl crate::Writable for PDMA_CH0TSR {}
#[doc = "PDMA_CH0TSR"]
pub mod pdma_ch0tsr;
#[doc = "PDMA_CH0CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch0ctsr](pdma_ch0ctsr) module"]
pub type PDMA_CH0CTSR = crate::Reg<u32, _PDMA_CH0CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH0CTSR;
#[doc = "`read()` method returns [pdma_ch0ctsr::R](pdma_ch0ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH0CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch0ctsr::W](pdma_ch0ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH0CTSR {}
#[doc = "PDMA_CH0CTSR"]
pub mod pdma_ch0ctsr;
#[doc = "PDMA_CH1CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch1cr](pdma_ch1cr) module"]
pub type PDMA_CH1CR = crate::Reg<u32, _PDMA_CH1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH1CR;
#[doc = "`read()` method returns [pdma_ch1cr::R](pdma_ch1cr::R) reader structure"]
impl crate::Readable for PDMA_CH1CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch1cr::W](pdma_ch1cr::W) writer structure"]
impl crate::Writable for PDMA_CH1CR {}
#[doc = "PDMA_CH1CR"]
pub mod pdma_ch1cr;
#[doc = "PDMA_CH1SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch1sadr](pdma_ch1sadr) module"]
pub type PDMA_CH1SADR = crate::Reg<u32, _PDMA_CH1SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH1SADR;
#[doc = "`read()` method returns [pdma_ch1sadr::R](pdma_ch1sadr::R) reader structure"]
impl crate::Readable for PDMA_CH1SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch1sadr::W](pdma_ch1sadr::W) writer structure"]
impl crate::Writable for PDMA_CH1SADR {}
#[doc = "PDMA_CH1SADR"]
pub mod pdma_ch1sadr;
#[doc = "PDMA_CH1DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch1dadr](pdma_ch1dadr) module"]
pub type PDMA_CH1DADR = crate::Reg<u32, _PDMA_CH1DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH1DADR;
#[doc = "`read()` method returns [pdma_ch1dadr::R](pdma_ch1dadr::R) reader structure"]
impl crate::Readable for PDMA_CH1DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch1dadr::W](pdma_ch1dadr::W) writer structure"]
impl crate::Writable for PDMA_CH1DADR {}
#[doc = "PDMA_CH1DADR"]
pub mod pdma_ch1dadr;
#[doc = "PDMA_CH1TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch1tsr](pdma_ch1tsr) module"]
pub type PDMA_CH1TSR = crate::Reg<u32, _PDMA_CH1TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH1TSR;
#[doc = "`read()` method returns [pdma_ch1tsr::R](pdma_ch1tsr::R) reader structure"]
impl crate::Readable for PDMA_CH1TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch1tsr::W](pdma_ch1tsr::W) writer structure"]
impl crate::Writable for PDMA_CH1TSR {}
#[doc = "PDMA_CH1TSR"]
pub mod pdma_ch1tsr;
#[doc = "PDMA_CH1CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch1ctsr](pdma_ch1ctsr) module"]
pub type PDMA_CH1CTSR = crate::Reg<u32, _PDMA_CH1CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH1CTSR;
#[doc = "`read()` method returns [pdma_ch1ctsr::R](pdma_ch1ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH1CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch1ctsr::W](pdma_ch1ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH1CTSR {}
#[doc = "PDMA_CH1CTSR"]
pub mod pdma_ch1ctsr;
#[doc = "PDMA_CH2CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch2cr](pdma_ch2cr) module"]
pub type PDMA_CH2CR = crate::Reg<u32, _PDMA_CH2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH2CR;
#[doc = "`read()` method returns [pdma_ch2cr::R](pdma_ch2cr::R) reader structure"]
impl crate::Readable for PDMA_CH2CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch2cr::W](pdma_ch2cr::W) writer structure"]
impl crate::Writable for PDMA_CH2CR {}
#[doc = "PDMA_CH2CR"]
pub mod pdma_ch2cr;
#[doc = "PDMA_CH2SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch2sadr](pdma_ch2sadr) module"]
pub type PDMA_CH2SADR = crate::Reg<u32, _PDMA_CH2SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH2SADR;
#[doc = "`read()` method returns [pdma_ch2sadr::R](pdma_ch2sadr::R) reader structure"]
impl crate::Readable for PDMA_CH2SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch2sadr::W](pdma_ch2sadr::W) writer structure"]
impl crate::Writable for PDMA_CH2SADR {}
#[doc = "PDMA_CH2SADR"]
pub mod pdma_ch2sadr;
#[doc = "PDMA_CH2DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch2dadr](pdma_ch2dadr) module"]
pub type PDMA_CH2DADR = crate::Reg<u32, _PDMA_CH2DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH2DADR;
#[doc = "`read()` method returns [pdma_ch2dadr::R](pdma_ch2dadr::R) reader structure"]
impl crate::Readable for PDMA_CH2DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch2dadr::W](pdma_ch2dadr::W) writer structure"]
impl crate::Writable for PDMA_CH2DADR {}
#[doc = "PDMA_CH2DADR"]
pub mod pdma_ch2dadr;
#[doc = "PDMA_CH2TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch2tsr](pdma_ch2tsr) module"]
pub type PDMA_CH2TSR = crate::Reg<u32, _PDMA_CH2TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH2TSR;
#[doc = "`read()` method returns [pdma_ch2tsr::R](pdma_ch2tsr::R) reader structure"]
impl crate::Readable for PDMA_CH2TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch2tsr::W](pdma_ch2tsr::W) writer structure"]
impl crate::Writable for PDMA_CH2TSR {}
#[doc = "PDMA_CH2TSR"]
pub mod pdma_ch2tsr;
#[doc = "PDMA_CH2CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch2ctsr](pdma_ch2ctsr) module"]
pub type PDMA_CH2CTSR = crate::Reg<u32, _PDMA_CH2CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH2CTSR;
#[doc = "`read()` method returns [pdma_ch2ctsr::R](pdma_ch2ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH2CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch2ctsr::W](pdma_ch2ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH2CTSR {}
#[doc = "PDMA_CH2CTSR"]
pub mod pdma_ch2ctsr;
#[doc = "PDMA_CH3CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch3cr](pdma_ch3cr) module"]
pub type PDMA_CH3CR = crate::Reg<u32, _PDMA_CH3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH3CR;
#[doc = "`read()` method returns [pdma_ch3cr::R](pdma_ch3cr::R) reader structure"]
impl crate::Readable for PDMA_CH3CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch3cr::W](pdma_ch3cr::W) writer structure"]
impl crate::Writable for PDMA_CH3CR {}
#[doc = "PDMA_CH3CR"]
pub mod pdma_ch3cr;
#[doc = "PDMA_CH3SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch3sadr](pdma_ch3sadr) module"]
pub type PDMA_CH3SADR = crate::Reg<u32, _PDMA_CH3SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH3SADR;
#[doc = "`read()` method returns [pdma_ch3sadr::R](pdma_ch3sadr::R) reader structure"]
impl crate::Readable for PDMA_CH3SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch3sadr::W](pdma_ch3sadr::W) writer structure"]
impl crate::Writable for PDMA_CH3SADR {}
#[doc = "PDMA_CH3SADR"]
pub mod pdma_ch3sadr;
#[doc = "PDMA_CH3DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch3dadr](pdma_ch3dadr) module"]
pub type PDMA_CH3DADR = crate::Reg<u32, _PDMA_CH3DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH3DADR;
#[doc = "`read()` method returns [pdma_ch3dadr::R](pdma_ch3dadr::R) reader structure"]
impl crate::Readable for PDMA_CH3DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch3dadr::W](pdma_ch3dadr::W) writer structure"]
impl crate::Writable for PDMA_CH3DADR {}
#[doc = "PDMA_CH3DADR"]
pub mod pdma_ch3dadr;
#[doc = "PDMA_CH3TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch3tsr](pdma_ch3tsr) module"]
pub type PDMA_CH3TSR = crate::Reg<u32, _PDMA_CH3TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH3TSR;
#[doc = "`read()` method returns [pdma_ch3tsr::R](pdma_ch3tsr::R) reader structure"]
impl crate::Readable for PDMA_CH3TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch3tsr::W](pdma_ch3tsr::W) writer structure"]
impl crate::Writable for PDMA_CH3TSR {}
#[doc = "PDMA_CH3TSR"]
pub mod pdma_ch3tsr;
#[doc = "PDMA_CH3CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch3ctsr](pdma_ch3ctsr) module"]
pub type PDMA_CH3CTSR = crate::Reg<u32, _PDMA_CH3CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH3CTSR;
#[doc = "`read()` method returns [pdma_ch3ctsr::R](pdma_ch3ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH3CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch3ctsr::W](pdma_ch3ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH3CTSR {}
#[doc = "PDMA_CH3CTSR"]
pub mod pdma_ch3ctsr;
#[doc = "PDMA_CH4CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch4cr](pdma_ch4cr) module"]
pub type PDMA_CH4CR = crate::Reg<u32, _PDMA_CH4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH4CR;
#[doc = "`read()` method returns [pdma_ch4cr::R](pdma_ch4cr::R) reader structure"]
impl crate::Readable for PDMA_CH4CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch4cr::W](pdma_ch4cr::W) writer structure"]
impl crate::Writable for PDMA_CH4CR {}
#[doc = "PDMA_CH4CR"]
pub mod pdma_ch4cr;
#[doc = "PDMA_CH4SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch4sadr](pdma_ch4sadr) module"]
pub type PDMA_CH4SADR = crate::Reg<u32, _PDMA_CH4SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH4SADR;
#[doc = "`read()` method returns [pdma_ch4sadr::R](pdma_ch4sadr::R) reader structure"]
impl crate::Readable for PDMA_CH4SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch4sadr::W](pdma_ch4sadr::W) writer structure"]
impl crate::Writable for PDMA_CH4SADR {}
#[doc = "PDMA_CH4SADR"]
pub mod pdma_ch4sadr;
#[doc = "PDMA_CH4DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch4dadr](pdma_ch4dadr) module"]
pub type PDMA_CH4DADR = crate::Reg<u32, _PDMA_CH4DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH4DADR;
#[doc = "`read()` method returns [pdma_ch4dadr::R](pdma_ch4dadr::R) reader structure"]
impl crate::Readable for PDMA_CH4DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch4dadr::W](pdma_ch4dadr::W) writer structure"]
impl crate::Writable for PDMA_CH4DADR {}
#[doc = "PDMA_CH4DADR"]
pub mod pdma_ch4dadr;
#[doc = "PDMA_CH4TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch4tsr](pdma_ch4tsr) module"]
pub type PDMA_CH4TSR = crate::Reg<u32, _PDMA_CH4TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH4TSR;
#[doc = "`read()` method returns [pdma_ch4tsr::R](pdma_ch4tsr::R) reader structure"]
impl crate::Readable for PDMA_CH4TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch4tsr::W](pdma_ch4tsr::W) writer structure"]
impl crate::Writable for PDMA_CH4TSR {}
#[doc = "PDMA_CH4TSR"]
pub mod pdma_ch4tsr;
#[doc = "PDMA_CH4CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch4ctsr](pdma_ch4ctsr) module"]
pub type PDMA_CH4CTSR = crate::Reg<u32, _PDMA_CH4CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH4CTSR;
#[doc = "`read()` method returns [pdma_ch4ctsr::R](pdma_ch4ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH4CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch4ctsr::W](pdma_ch4ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH4CTSR {}
#[doc = "PDMA_CH4CTSR"]
pub mod pdma_ch4ctsr;
#[doc = "PDMA_CH5CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch5cr](pdma_ch5cr) module"]
pub type PDMA_CH5CR = crate::Reg<u32, _PDMA_CH5CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH5CR;
#[doc = "`read()` method returns [pdma_ch5cr::R](pdma_ch5cr::R) reader structure"]
impl crate::Readable for PDMA_CH5CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch5cr::W](pdma_ch5cr::W) writer structure"]
impl crate::Writable for PDMA_CH5CR {}
#[doc = "PDMA_CH5CR"]
pub mod pdma_ch5cr;
#[doc = "PDMA_CH5SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch5sadr](pdma_ch5sadr) module"]
pub type PDMA_CH5SADR = crate::Reg<u32, _PDMA_CH5SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH5SADR;
#[doc = "`read()` method returns [pdma_ch5sadr::R](pdma_ch5sadr::R) reader structure"]
impl crate::Readable for PDMA_CH5SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch5sadr::W](pdma_ch5sadr::W) writer structure"]
impl crate::Writable for PDMA_CH5SADR {}
#[doc = "PDMA_CH5SADR"]
pub mod pdma_ch5sadr;
#[doc = "PDMA_CH5DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch5dadr](pdma_ch5dadr) module"]
pub type PDMA_CH5DADR = crate::Reg<u32, _PDMA_CH5DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH5DADR;
#[doc = "`read()` method returns [pdma_ch5dadr::R](pdma_ch5dadr::R) reader structure"]
impl crate::Readable for PDMA_CH5DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch5dadr::W](pdma_ch5dadr::W) writer structure"]
impl crate::Writable for PDMA_CH5DADR {}
#[doc = "PDMA_CH5DADR"]
pub mod pdma_ch5dadr;
#[doc = "PDMA_CH5TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch5tsr](pdma_ch5tsr) module"]
pub type PDMA_CH5TSR = crate::Reg<u32, _PDMA_CH5TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH5TSR;
#[doc = "`read()` method returns [pdma_ch5tsr::R](pdma_ch5tsr::R) reader structure"]
impl crate::Readable for PDMA_CH5TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch5tsr::W](pdma_ch5tsr::W) writer structure"]
impl crate::Writable for PDMA_CH5TSR {}
#[doc = "PDMA_CH5TSR"]
pub mod pdma_ch5tsr;
#[doc = "PDMA_CH5CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch5ctsr](pdma_ch5ctsr) module"]
pub type PDMA_CH5CTSR = crate::Reg<u32, _PDMA_CH5CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH5CTSR;
#[doc = "`read()` method returns [pdma_ch5ctsr::R](pdma_ch5ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH5CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch5ctsr::W](pdma_ch5ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH5CTSR {}
#[doc = "PDMA_CH5CTSR"]
pub mod pdma_ch5ctsr;
#[doc = "PDMA_ISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_isr](pdma_isr) module"]
pub type PDMA_ISR = crate::Reg<u32, _PDMA_ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_ISR;
#[doc = "`read()` method returns [pdma_isr::R](pdma_isr::R) reader structure"]
impl crate::Readable for PDMA_ISR {}
#[doc = "`write(|w| ..)` method takes [pdma_isr::W](pdma_isr::W) writer structure"]
impl crate::Writable for PDMA_ISR {}
#[doc = "PDMA_ISR"]
pub mod pdma_isr;
#[doc = "PDMA_ISCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_iscr](pdma_iscr) module"]
pub type PDMA_ISCR = crate::Reg<u32, _PDMA_ISCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_ISCR;
#[doc = "`read()` method returns [pdma_iscr::R](pdma_iscr::R) reader structure"]
impl crate::Readable for PDMA_ISCR {}
#[doc = "`write(|w| ..)` method takes [pdma_iscr::W](pdma_iscr::W) writer structure"]
impl crate::Writable for PDMA_ISCR {}
#[doc = "PDMA_ISCR"]
pub mod pdma_iscr;
#[doc = "PDMA_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ier](pdma_ier) module"]
pub type PDMA_IER = crate::Reg<u32, _PDMA_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_IER;
#[doc = "`read()` method returns [pdma_ier::R](pdma_ier::R) reader structure"]
impl crate::Readable for PDMA_IER {}
#[doc = "`write(|w| ..)` method takes [pdma_ier::W](pdma_ier::W) writer structure"]
impl crate::Writable for PDMA_IER {}
#[doc = "PDMA_IER"]
pub mod pdma_ier;
