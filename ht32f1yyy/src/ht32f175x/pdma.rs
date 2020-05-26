#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PDMA_CH0CR"]
    pub pdma_ch0cr: PDMA_CH0CR,
    #[doc = "0x04 - PDMA_CH0SADR"]
    pub pdma_ch0sadr: PDMA_CH0SADR,
    #[doc = "0x08 - PDMA_CH0DADR"]
    pub pdma_ch0dadr: PDMA_CH0DADR,
    #[doc = "0x0c - PDMA_CH0CADR"]
    pub pdma_ch0cadr: PDMA_CH0CADR,
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
    #[doc = "0x24 - PDMA_CH1CADR"]
    pub pdma_ch1cadr: PDMA_CH1CADR,
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
    #[doc = "0x3c - PDMA_CH2CADR"]
    pub pdma_ch2cadr: PDMA_CH2CADR,
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
    #[doc = "0x54 - PDMA_CH3CADR"]
    pub pdma_ch3cadr: PDMA_CH3CADR,
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
    #[doc = "0x6c - PDMA_CH4CADR"]
    pub pdma_ch4cadr: PDMA_CH4CADR,
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
    #[doc = "0x84 - PDMA_CH5CADR"]
    pub pdma_ch5cadr: PDMA_CH5CADR,
    #[doc = "0x88 - PDMA_CH5TSR"]
    pub pdma_ch5tsr: PDMA_CH5TSR,
    #[doc = "0x8c - PDMA_CH5CTSR"]
    pub pdma_ch5ctsr: PDMA_CH5CTSR,
    #[doc = "0x90 - PDMA_CH6CR"]
    pub pdma_ch6cr: PDMA_CH6CR,
    #[doc = "0x94 - PDMA_CH6SADR"]
    pub pdma_ch6sadr: PDMA_CH6SADR,
    #[doc = "0x98 - PDMA_CH6DADR"]
    pub pdma_ch6dadr: PDMA_CH6DADR,
    #[doc = "0x9c - PDMA_CH6CADR"]
    pub pdma_ch6cadr: PDMA_CH6CADR,
    #[doc = "0xa0 - PDMA_CH6TSR"]
    pub pdma_ch6tsr: PDMA_CH6TSR,
    #[doc = "0xa4 - PDMA_CH6CTSR"]
    pub pdma_ch6ctsr: PDMA_CH6CTSR,
    #[doc = "0xa8 - PDMA_CH7CR"]
    pub pdma_ch7cr: PDMA_CH7CR,
    #[doc = "0xac - PDMA_CH7SADR"]
    pub pdma_ch7sadr: PDMA_CH7SADR,
    #[doc = "0xb0 - PDMA_CH7DADR"]
    pub pdma_ch7dadr: PDMA_CH7DADR,
    #[doc = "0xb4 - PDMA_CH7CADR"]
    pub pdma_ch7cadr: PDMA_CH7CADR,
    #[doc = "0xb8 - PDMA_CH7TSR"]
    pub pdma_ch7tsr: PDMA_CH7TSR,
    #[doc = "0xbc - PDMA_CH7CTSR"]
    pub pdma_ch7ctsr: PDMA_CH7CTSR,
    #[doc = "0xc0 - PDMA_CH8CR"]
    pub pdma_ch8cr: PDMA_CH8CR,
    #[doc = "0xc4 - PDMA_CH8SADR"]
    pub pdma_ch8sadr: PDMA_CH8SADR,
    #[doc = "0xc8 - PDMA_CH8DADR"]
    pub pdma_ch8dadr: PDMA_CH8DADR,
    #[doc = "0xcc - PDMA_CH8CADR"]
    pub pdma_ch8cadr: PDMA_CH8CADR,
    #[doc = "0xd0 - PDMA_CH8TSR"]
    pub pdma_ch8tsr: PDMA_CH8TSR,
    #[doc = "0xd4 - PDMA_CH8CTSR"]
    pub pdma_ch8ctsr: PDMA_CH8CTSR,
    #[doc = "0xd8 - PDMA_CH9CR"]
    pub pdma_ch9cr: PDMA_CH9CR,
    #[doc = "0xdc - PDMA_CH9SADR"]
    pub pdma_ch9sadr: PDMA_CH9SADR,
    #[doc = "0xe0 - PDMA_CH9DADR"]
    pub pdma_ch9dadr: PDMA_CH9DADR,
    #[doc = "0xe4 - PDMA_CH9CADR"]
    pub pdma_ch9cadr: PDMA_CH9CADR,
    #[doc = "0xe8 - PDMA_CH9TSR"]
    pub pdma_ch9tsr: PDMA_CH9TSR,
    #[doc = "0xec - PDMA_CH9CTSR"]
    pub pdma_ch9ctsr: PDMA_CH9CTSR,
    #[doc = "0xf0 - PDMA_CH10CR"]
    pub pdma_ch10cr: PDMA_CH10CR,
    #[doc = "0xf4 - PDMA_CH10SADR"]
    pub pdma_ch10sadr: PDMA_CH10SADR,
    #[doc = "0xf8 - PDMA_CH10DADR"]
    pub pdma_ch10dadr: PDMA_CH10DADR,
    #[doc = "0xfc - PDMA_CH10CADR"]
    pub pdma_ch10cadr: PDMA_CH10CADR,
    #[doc = "0x100 - PDMA_CH10TSR"]
    pub pdma_ch10tsr: PDMA_CH10TSR,
    #[doc = "0x104 - PDMA_CH10CTSR"]
    pub pdma_ch10ctsr: PDMA_CH10CTSR,
    #[doc = "0x108 - PDMA_CH11CR"]
    pub pdma_ch11cr: PDMA_CH11CR,
    #[doc = "0x10c - PDMA_CH11SADR"]
    pub pdma_ch11sadr: PDMA_CH11SADR,
    #[doc = "0x110 - PDMA_CH11DADR"]
    pub pdma_ch11dadr: PDMA_CH11DADR,
    #[doc = "0x114 - PDMA_CH11CADR"]
    pub pdma_ch11cadr: PDMA_CH11CADR,
    #[doc = "0x118 - PDMA_CH11TSR"]
    pub pdma_ch11tsr: PDMA_CH11TSR,
    #[doc = "0x11c - PDMA_CH11CTSR"]
    pub pdma_ch11ctsr: PDMA_CH11CTSR,
    #[doc = "0x120 - PDMA_ISR0"]
    pub pdma_isr0: PDMA_ISR0,
    #[doc = "0x124 - PDMA_ISR1"]
    pub pdma_isr1: PDMA_ISR1,
    #[doc = "0x128 - PDMA_ISCR0"]
    pub pdma_iscr0: PDMA_ISCR0,
    #[doc = "0x12c - PDMA_ISCR1"]
    pub pdma_iscr1: PDMA_ISCR1,
    #[doc = "0x130 - PDMA_IER0"]
    pub pdma_ier0: PDMA_IER0,
    #[doc = "0x134 - PDMA_IER1"]
    pub pdma_ier1: PDMA_IER1,
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
#[doc = "PDMA_CH0CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch0cadr](pdma_ch0cadr) module"]
pub type PDMA_CH0CADR = crate::Reg<u32, _PDMA_CH0CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH0CADR;
#[doc = "`read()` method returns [pdma_ch0cadr::R](pdma_ch0cadr::R) reader structure"]
impl crate::Readable for PDMA_CH0CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch0cadr::W](pdma_ch0cadr::W) writer structure"]
impl crate::Writable for PDMA_CH0CADR {}
#[doc = "PDMA_CH0CADR"]
pub mod pdma_ch0cadr;
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
#[doc = "PDMA_CH1CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch1cadr](pdma_ch1cadr) module"]
pub type PDMA_CH1CADR = crate::Reg<u32, _PDMA_CH1CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH1CADR;
#[doc = "`read()` method returns [pdma_ch1cadr::R](pdma_ch1cadr::R) reader structure"]
impl crate::Readable for PDMA_CH1CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch1cadr::W](pdma_ch1cadr::W) writer structure"]
impl crate::Writable for PDMA_CH1CADR {}
#[doc = "PDMA_CH1CADR"]
pub mod pdma_ch1cadr;
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
#[doc = "PDMA_CH2CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch2cadr](pdma_ch2cadr) module"]
pub type PDMA_CH2CADR = crate::Reg<u32, _PDMA_CH2CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH2CADR;
#[doc = "`read()` method returns [pdma_ch2cadr::R](pdma_ch2cadr::R) reader structure"]
impl crate::Readable for PDMA_CH2CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch2cadr::W](pdma_ch2cadr::W) writer structure"]
impl crate::Writable for PDMA_CH2CADR {}
#[doc = "PDMA_CH2CADR"]
pub mod pdma_ch2cadr;
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
#[doc = "PDMA_CH3CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch3cadr](pdma_ch3cadr) module"]
pub type PDMA_CH3CADR = crate::Reg<u32, _PDMA_CH3CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH3CADR;
#[doc = "`read()` method returns [pdma_ch3cadr::R](pdma_ch3cadr::R) reader structure"]
impl crate::Readable for PDMA_CH3CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch3cadr::W](pdma_ch3cadr::W) writer structure"]
impl crate::Writable for PDMA_CH3CADR {}
#[doc = "PDMA_CH3CADR"]
pub mod pdma_ch3cadr;
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
#[doc = "PDMA_CH4CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch4cadr](pdma_ch4cadr) module"]
pub type PDMA_CH4CADR = crate::Reg<u32, _PDMA_CH4CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH4CADR;
#[doc = "`read()` method returns [pdma_ch4cadr::R](pdma_ch4cadr::R) reader structure"]
impl crate::Readable for PDMA_CH4CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch4cadr::W](pdma_ch4cadr::W) writer structure"]
impl crate::Writable for PDMA_CH4CADR {}
#[doc = "PDMA_CH4CADR"]
pub mod pdma_ch4cadr;
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
#[doc = "PDMA_CH5CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch5cadr](pdma_ch5cadr) module"]
pub type PDMA_CH5CADR = crate::Reg<u32, _PDMA_CH5CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH5CADR;
#[doc = "`read()` method returns [pdma_ch5cadr::R](pdma_ch5cadr::R) reader structure"]
impl crate::Readable for PDMA_CH5CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch5cadr::W](pdma_ch5cadr::W) writer structure"]
impl crate::Writable for PDMA_CH5CADR {}
#[doc = "PDMA_CH5CADR"]
pub mod pdma_ch5cadr;
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
#[doc = "PDMA_CH6CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch6cr](pdma_ch6cr) module"]
pub type PDMA_CH6CR = crate::Reg<u32, _PDMA_CH6CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH6CR;
#[doc = "`read()` method returns [pdma_ch6cr::R](pdma_ch6cr::R) reader structure"]
impl crate::Readable for PDMA_CH6CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch6cr::W](pdma_ch6cr::W) writer structure"]
impl crate::Writable for PDMA_CH6CR {}
#[doc = "PDMA_CH6CR"]
pub mod pdma_ch6cr;
#[doc = "PDMA_CH6SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch6sadr](pdma_ch6sadr) module"]
pub type PDMA_CH6SADR = crate::Reg<u32, _PDMA_CH6SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH6SADR;
#[doc = "`read()` method returns [pdma_ch6sadr::R](pdma_ch6sadr::R) reader structure"]
impl crate::Readable for PDMA_CH6SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch6sadr::W](pdma_ch6sadr::W) writer structure"]
impl crate::Writable for PDMA_CH6SADR {}
#[doc = "PDMA_CH6SADR"]
pub mod pdma_ch6sadr;
#[doc = "PDMA_CH6DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch6dadr](pdma_ch6dadr) module"]
pub type PDMA_CH6DADR = crate::Reg<u32, _PDMA_CH6DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH6DADR;
#[doc = "`read()` method returns [pdma_ch6dadr::R](pdma_ch6dadr::R) reader structure"]
impl crate::Readable for PDMA_CH6DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch6dadr::W](pdma_ch6dadr::W) writer structure"]
impl crate::Writable for PDMA_CH6DADR {}
#[doc = "PDMA_CH6DADR"]
pub mod pdma_ch6dadr;
#[doc = "PDMA_CH6CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch6cadr](pdma_ch6cadr) module"]
pub type PDMA_CH6CADR = crate::Reg<u32, _PDMA_CH6CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH6CADR;
#[doc = "`read()` method returns [pdma_ch6cadr::R](pdma_ch6cadr::R) reader structure"]
impl crate::Readable for PDMA_CH6CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch6cadr::W](pdma_ch6cadr::W) writer structure"]
impl crate::Writable for PDMA_CH6CADR {}
#[doc = "PDMA_CH6CADR"]
pub mod pdma_ch6cadr;
#[doc = "PDMA_CH6TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch6tsr](pdma_ch6tsr) module"]
pub type PDMA_CH6TSR = crate::Reg<u32, _PDMA_CH6TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH6TSR;
#[doc = "`read()` method returns [pdma_ch6tsr::R](pdma_ch6tsr::R) reader structure"]
impl crate::Readable for PDMA_CH6TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch6tsr::W](pdma_ch6tsr::W) writer structure"]
impl crate::Writable for PDMA_CH6TSR {}
#[doc = "PDMA_CH6TSR"]
pub mod pdma_ch6tsr;
#[doc = "PDMA_CH6CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch6ctsr](pdma_ch6ctsr) module"]
pub type PDMA_CH6CTSR = crate::Reg<u32, _PDMA_CH6CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH6CTSR;
#[doc = "`read()` method returns [pdma_ch6ctsr::R](pdma_ch6ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH6CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch6ctsr::W](pdma_ch6ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH6CTSR {}
#[doc = "PDMA_CH6CTSR"]
pub mod pdma_ch6ctsr;
#[doc = "PDMA_CH7CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch7cr](pdma_ch7cr) module"]
pub type PDMA_CH7CR = crate::Reg<u32, _PDMA_CH7CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH7CR;
#[doc = "`read()` method returns [pdma_ch7cr::R](pdma_ch7cr::R) reader structure"]
impl crate::Readable for PDMA_CH7CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch7cr::W](pdma_ch7cr::W) writer structure"]
impl crate::Writable for PDMA_CH7CR {}
#[doc = "PDMA_CH7CR"]
pub mod pdma_ch7cr;
#[doc = "PDMA_CH7SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch7sadr](pdma_ch7sadr) module"]
pub type PDMA_CH7SADR = crate::Reg<u32, _PDMA_CH7SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH7SADR;
#[doc = "`read()` method returns [pdma_ch7sadr::R](pdma_ch7sadr::R) reader structure"]
impl crate::Readable for PDMA_CH7SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch7sadr::W](pdma_ch7sadr::W) writer structure"]
impl crate::Writable for PDMA_CH7SADR {}
#[doc = "PDMA_CH7SADR"]
pub mod pdma_ch7sadr;
#[doc = "PDMA_CH7DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch7dadr](pdma_ch7dadr) module"]
pub type PDMA_CH7DADR = crate::Reg<u32, _PDMA_CH7DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH7DADR;
#[doc = "`read()` method returns [pdma_ch7dadr::R](pdma_ch7dadr::R) reader structure"]
impl crate::Readable for PDMA_CH7DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch7dadr::W](pdma_ch7dadr::W) writer structure"]
impl crate::Writable for PDMA_CH7DADR {}
#[doc = "PDMA_CH7DADR"]
pub mod pdma_ch7dadr;
#[doc = "PDMA_CH7CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch7cadr](pdma_ch7cadr) module"]
pub type PDMA_CH7CADR = crate::Reg<u32, _PDMA_CH7CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH7CADR;
#[doc = "`read()` method returns [pdma_ch7cadr::R](pdma_ch7cadr::R) reader structure"]
impl crate::Readable for PDMA_CH7CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch7cadr::W](pdma_ch7cadr::W) writer structure"]
impl crate::Writable for PDMA_CH7CADR {}
#[doc = "PDMA_CH7CADR"]
pub mod pdma_ch7cadr;
#[doc = "PDMA_CH7TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch7tsr](pdma_ch7tsr) module"]
pub type PDMA_CH7TSR = crate::Reg<u32, _PDMA_CH7TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH7TSR;
#[doc = "`read()` method returns [pdma_ch7tsr::R](pdma_ch7tsr::R) reader structure"]
impl crate::Readable for PDMA_CH7TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch7tsr::W](pdma_ch7tsr::W) writer structure"]
impl crate::Writable for PDMA_CH7TSR {}
#[doc = "PDMA_CH7TSR"]
pub mod pdma_ch7tsr;
#[doc = "PDMA_CH7CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch7ctsr](pdma_ch7ctsr) module"]
pub type PDMA_CH7CTSR = crate::Reg<u32, _PDMA_CH7CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH7CTSR;
#[doc = "`read()` method returns [pdma_ch7ctsr::R](pdma_ch7ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH7CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch7ctsr::W](pdma_ch7ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH7CTSR {}
#[doc = "PDMA_CH7CTSR"]
pub mod pdma_ch7ctsr;
#[doc = "PDMA_CH8CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch8cr](pdma_ch8cr) module"]
pub type PDMA_CH8CR = crate::Reg<u32, _PDMA_CH8CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH8CR;
#[doc = "`read()` method returns [pdma_ch8cr::R](pdma_ch8cr::R) reader structure"]
impl crate::Readable for PDMA_CH8CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch8cr::W](pdma_ch8cr::W) writer structure"]
impl crate::Writable for PDMA_CH8CR {}
#[doc = "PDMA_CH8CR"]
pub mod pdma_ch8cr;
#[doc = "PDMA_CH8SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch8sadr](pdma_ch8sadr) module"]
pub type PDMA_CH8SADR = crate::Reg<u32, _PDMA_CH8SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH8SADR;
#[doc = "`read()` method returns [pdma_ch8sadr::R](pdma_ch8sadr::R) reader structure"]
impl crate::Readable for PDMA_CH8SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch8sadr::W](pdma_ch8sadr::W) writer structure"]
impl crate::Writable for PDMA_CH8SADR {}
#[doc = "PDMA_CH8SADR"]
pub mod pdma_ch8sadr;
#[doc = "PDMA_CH8DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch8dadr](pdma_ch8dadr) module"]
pub type PDMA_CH8DADR = crate::Reg<u32, _PDMA_CH8DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH8DADR;
#[doc = "`read()` method returns [pdma_ch8dadr::R](pdma_ch8dadr::R) reader structure"]
impl crate::Readable for PDMA_CH8DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch8dadr::W](pdma_ch8dadr::W) writer structure"]
impl crate::Writable for PDMA_CH8DADR {}
#[doc = "PDMA_CH8DADR"]
pub mod pdma_ch8dadr;
#[doc = "PDMA_CH8CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch8cadr](pdma_ch8cadr) module"]
pub type PDMA_CH8CADR = crate::Reg<u32, _PDMA_CH8CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH8CADR;
#[doc = "`read()` method returns [pdma_ch8cadr::R](pdma_ch8cadr::R) reader structure"]
impl crate::Readable for PDMA_CH8CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch8cadr::W](pdma_ch8cadr::W) writer structure"]
impl crate::Writable for PDMA_CH8CADR {}
#[doc = "PDMA_CH8CADR"]
pub mod pdma_ch8cadr;
#[doc = "PDMA_CH8TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch8tsr](pdma_ch8tsr) module"]
pub type PDMA_CH8TSR = crate::Reg<u32, _PDMA_CH8TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH8TSR;
#[doc = "`read()` method returns [pdma_ch8tsr::R](pdma_ch8tsr::R) reader structure"]
impl crate::Readable for PDMA_CH8TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch8tsr::W](pdma_ch8tsr::W) writer structure"]
impl crate::Writable for PDMA_CH8TSR {}
#[doc = "PDMA_CH8TSR"]
pub mod pdma_ch8tsr;
#[doc = "PDMA_CH8CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch8ctsr](pdma_ch8ctsr) module"]
pub type PDMA_CH8CTSR = crate::Reg<u32, _PDMA_CH8CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH8CTSR;
#[doc = "`read()` method returns [pdma_ch8ctsr::R](pdma_ch8ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH8CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch8ctsr::W](pdma_ch8ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH8CTSR {}
#[doc = "PDMA_CH8CTSR"]
pub mod pdma_ch8ctsr;
#[doc = "PDMA_CH9CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch9cr](pdma_ch9cr) module"]
pub type PDMA_CH9CR = crate::Reg<u32, _PDMA_CH9CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH9CR;
#[doc = "`read()` method returns [pdma_ch9cr::R](pdma_ch9cr::R) reader structure"]
impl crate::Readable for PDMA_CH9CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch9cr::W](pdma_ch9cr::W) writer structure"]
impl crate::Writable for PDMA_CH9CR {}
#[doc = "PDMA_CH9CR"]
pub mod pdma_ch9cr;
#[doc = "PDMA_CH9SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch9sadr](pdma_ch9sadr) module"]
pub type PDMA_CH9SADR = crate::Reg<u32, _PDMA_CH9SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH9SADR;
#[doc = "`read()` method returns [pdma_ch9sadr::R](pdma_ch9sadr::R) reader structure"]
impl crate::Readable for PDMA_CH9SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch9sadr::W](pdma_ch9sadr::W) writer structure"]
impl crate::Writable for PDMA_CH9SADR {}
#[doc = "PDMA_CH9SADR"]
pub mod pdma_ch9sadr;
#[doc = "PDMA_CH9DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch9dadr](pdma_ch9dadr) module"]
pub type PDMA_CH9DADR = crate::Reg<u32, _PDMA_CH9DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH9DADR;
#[doc = "`read()` method returns [pdma_ch9dadr::R](pdma_ch9dadr::R) reader structure"]
impl crate::Readable for PDMA_CH9DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch9dadr::W](pdma_ch9dadr::W) writer structure"]
impl crate::Writable for PDMA_CH9DADR {}
#[doc = "PDMA_CH9DADR"]
pub mod pdma_ch9dadr;
#[doc = "PDMA_CH9CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch9cadr](pdma_ch9cadr) module"]
pub type PDMA_CH9CADR = crate::Reg<u32, _PDMA_CH9CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH9CADR;
#[doc = "`read()` method returns [pdma_ch9cadr::R](pdma_ch9cadr::R) reader structure"]
impl crate::Readable for PDMA_CH9CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch9cadr::W](pdma_ch9cadr::W) writer structure"]
impl crate::Writable for PDMA_CH9CADR {}
#[doc = "PDMA_CH9CADR"]
pub mod pdma_ch9cadr;
#[doc = "PDMA_CH9TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch9tsr](pdma_ch9tsr) module"]
pub type PDMA_CH9TSR = crate::Reg<u32, _PDMA_CH9TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH9TSR;
#[doc = "`read()` method returns [pdma_ch9tsr::R](pdma_ch9tsr::R) reader structure"]
impl crate::Readable for PDMA_CH9TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch9tsr::W](pdma_ch9tsr::W) writer structure"]
impl crate::Writable for PDMA_CH9TSR {}
#[doc = "PDMA_CH9TSR"]
pub mod pdma_ch9tsr;
#[doc = "PDMA_CH9CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch9ctsr](pdma_ch9ctsr) module"]
pub type PDMA_CH9CTSR = crate::Reg<u32, _PDMA_CH9CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH9CTSR;
#[doc = "`read()` method returns [pdma_ch9ctsr::R](pdma_ch9ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH9CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch9ctsr::W](pdma_ch9ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH9CTSR {}
#[doc = "PDMA_CH9CTSR"]
pub mod pdma_ch9ctsr;
#[doc = "PDMA_CH10CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch10cr](pdma_ch10cr) module"]
pub type PDMA_CH10CR = crate::Reg<u32, _PDMA_CH10CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH10CR;
#[doc = "`read()` method returns [pdma_ch10cr::R](pdma_ch10cr::R) reader structure"]
impl crate::Readable for PDMA_CH10CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch10cr::W](pdma_ch10cr::W) writer structure"]
impl crate::Writable for PDMA_CH10CR {}
#[doc = "PDMA_CH10CR"]
pub mod pdma_ch10cr;
#[doc = "PDMA_CH10SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch10sadr](pdma_ch10sadr) module"]
pub type PDMA_CH10SADR = crate::Reg<u32, _PDMA_CH10SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH10SADR;
#[doc = "`read()` method returns [pdma_ch10sadr::R](pdma_ch10sadr::R) reader structure"]
impl crate::Readable for PDMA_CH10SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch10sadr::W](pdma_ch10sadr::W) writer structure"]
impl crate::Writable for PDMA_CH10SADR {}
#[doc = "PDMA_CH10SADR"]
pub mod pdma_ch10sadr;
#[doc = "PDMA_CH10DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch10dadr](pdma_ch10dadr) module"]
pub type PDMA_CH10DADR = crate::Reg<u32, _PDMA_CH10DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH10DADR;
#[doc = "`read()` method returns [pdma_ch10dadr::R](pdma_ch10dadr::R) reader structure"]
impl crate::Readable for PDMA_CH10DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch10dadr::W](pdma_ch10dadr::W) writer structure"]
impl crate::Writable for PDMA_CH10DADR {}
#[doc = "PDMA_CH10DADR"]
pub mod pdma_ch10dadr;
#[doc = "PDMA_CH10CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch10cadr](pdma_ch10cadr) module"]
pub type PDMA_CH10CADR = crate::Reg<u32, _PDMA_CH10CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH10CADR;
#[doc = "`read()` method returns [pdma_ch10cadr::R](pdma_ch10cadr::R) reader structure"]
impl crate::Readable for PDMA_CH10CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch10cadr::W](pdma_ch10cadr::W) writer structure"]
impl crate::Writable for PDMA_CH10CADR {}
#[doc = "PDMA_CH10CADR"]
pub mod pdma_ch10cadr;
#[doc = "PDMA_CH10TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch10tsr](pdma_ch10tsr) module"]
pub type PDMA_CH10TSR = crate::Reg<u32, _PDMA_CH10TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH10TSR;
#[doc = "`read()` method returns [pdma_ch10tsr::R](pdma_ch10tsr::R) reader structure"]
impl crate::Readable for PDMA_CH10TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch10tsr::W](pdma_ch10tsr::W) writer structure"]
impl crate::Writable for PDMA_CH10TSR {}
#[doc = "PDMA_CH10TSR"]
pub mod pdma_ch10tsr;
#[doc = "PDMA_CH10CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch10ctsr](pdma_ch10ctsr) module"]
pub type PDMA_CH10CTSR = crate::Reg<u32, _PDMA_CH10CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH10CTSR;
#[doc = "`read()` method returns [pdma_ch10ctsr::R](pdma_ch10ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH10CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch10ctsr::W](pdma_ch10ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH10CTSR {}
#[doc = "PDMA_CH10CTSR"]
pub mod pdma_ch10ctsr;
#[doc = "PDMA_CH11CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch11cr](pdma_ch11cr) module"]
pub type PDMA_CH11CR = crate::Reg<u32, _PDMA_CH11CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH11CR;
#[doc = "`read()` method returns [pdma_ch11cr::R](pdma_ch11cr::R) reader structure"]
impl crate::Readable for PDMA_CH11CR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch11cr::W](pdma_ch11cr::W) writer structure"]
impl crate::Writable for PDMA_CH11CR {}
#[doc = "PDMA_CH11CR"]
pub mod pdma_ch11cr;
#[doc = "PDMA_CH11SADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch11sadr](pdma_ch11sadr) module"]
pub type PDMA_CH11SADR = crate::Reg<u32, _PDMA_CH11SADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH11SADR;
#[doc = "`read()` method returns [pdma_ch11sadr::R](pdma_ch11sadr::R) reader structure"]
impl crate::Readable for PDMA_CH11SADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch11sadr::W](pdma_ch11sadr::W) writer structure"]
impl crate::Writable for PDMA_CH11SADR {}
#[doc = "PDMA_CH11SADR"]
pub mod pdma_ch11sadr;
#[doc = "PDMA_CH11DADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch11dadr](pdma_ch11dadr) module"]
pub type PDMA_CH11DADR = crate::Reg<u32, _PDMA_CH11DADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH11DADR;
#[doc = "`read()` method returns [pdma_ch11dadr::R](pdma_ch11dadr::R) reader structure"]
impl crate::Readable for PDMA_CH11DADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch11dadr::W](pdma_ch11dadr::W) writer structure"]
impl crate::Writable for PDMA_CH11DADR {}
#[doc = "PDMA_CH11DADR"]
pub mod pdma_ch11dadr;
#[doc = "PDMA_CH11CADR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch11cadr](pdma_ch11cadr) module"]
pub type PDMA_CH11CADR = crate::Reg<u32, _PDMA_CH11CADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH11CADR;
#[doc = "`read()` method returns [pdma_ch11cadr::R](pdma_ch11cadr::R) reader structure"]
impl crate::Readable for PDMA_CH11CADR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch11cadr::W](pdma_ch11cadr::W) writer structure"]
impl crate::Writable for PDMA_CH11CADR {}
#[doc = "PDMA_CH11CADR"]
pub mod pdma_ch11cadr;
#[doc = "PDMA_CH11TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch11tsr](pdma_ch11tsr) module"]
pub type PDMA_CH11TSR = crate::Reg<u32, _PDMA_CH11TSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH11TSR;
#[doc = "`read()` method returns [pdma_ch11tsr::R](pdma_ch11tsr::R) reader structure"]
impl crate::Readable for PDMA_CH11TSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch11tsr::W](pdma_ch11tsr::W) writer structure"]
impl crate::Writable for PDMA_CH11TSR {}
#[doc = "PDMA_CH11TSR"]
pub mod pdma_ch11tsr;
#[doc = "PDMA_CH11CTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ch11ctsr](pdma_ch11ctsr) module"]
pub type PDMA_CH11CTSR = crate::Reg<u32, _PDMA_CH11CTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_CH11CTSR;
#[doc = "`read()` method returns [pdma_ch11ctsr::R](pdma_ch11ctsr::R) reader structure"]
impl crate::Readable for PDMA_CH11CTSR {}
#[doc = "`write(|w| ..)` method takes [pdma_ch11ctsr::W](pdma_ch11ctsr::W) writer structure"]
impl crate::Writable for PDMA_CH11CTSR {}
#[doc = "PDMA_CH11CTSR"]
pub mod pdma_ch11ctsr;
#[doc = "PDMA_ISR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_isr0](pdma_isr0) module"]
pub type PDMA_ISR0 = crate::Reg<u32, _PDMA_ISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_ISR0;
#[doc = "`read()` method returns [pdma_isr0::R](pdma_isr0::R) reader structure"]
impl crate::Readable for PDMA_ISR0 {}
#[doc = "`write(|w| ..)` method takes [pdma_isr0::W](pdma_isr0::W) writer structure"]
impl crate::Writable for PDMA_ISR0 {}
#[doc = "PDMA_ISR0"]
pub mod pdma_isr0;
#[doc = "PDMA_ISR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_isr1](pdma_isr1) module"]
pub type PDMA_ISR1 = crate::Reg<u32, _PDMA_ISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_ISR1;
#[doc = "`read()` method returns [pdma_isr1::R](pdma_isr1::R) reader structure"]
impl crate::Readable for PDMA_ISR1 {}
#[doc = "`write(|w| ..)` method takes [pdma_isr1::W](pdma_isr1::W) writer structure"]
impl crate::Writable for PDMA_ISR1 {}
#[doc = "PDMA_ISR1"]
pub mod pdma_isr1;
#[doc = "PDMA_ISCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_iscr0](pdma_iscr0) module"]
pub type PDMA_ISCR0 = crate::Reg<u32, _PDMA_ISCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_ISCR0;
#[doc = "`read()` method returns [pdma_iscr0::R](pdma_iscr0::R) reader structure"]
impl crate::Readable for PDMA_ISCR0 {}
#[doc = "`write(|w| ..)` method takes [pdma_iscr0::W](pdma_iscr0::W) writer structure"]
impl crate::Writable for PDMA_ISCR0 {}
#[doc = "PDMA_ISCR0"]
pub mod pdma_iscr0;
#[doc = "PDMA_ISCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_iscr1](pdma_iscr1) module"]
pub type PDMA_ISCR1 = crate::Reg<u32, _PDMA_ISCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_ISCR1;
#[doc = "`read()` method returns [pdma_iscr1::R](pdma_iscr1::R) reader structure"]
impl crate::Readable for PDMA_ISCR1 {}
#[doc = "`write(|w| ..)` method takes [pdma_iscr1::W](pdma_iscr1::W) writer structure"]
impl crate::Writable for PDMA_ISCR1 {}
#[doc = "PDMA_ISCR1"]
pub mod pdma_iscr1;
#[doc = "PDMA_IER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ier0](pdma_ier0) module"]
pub type PDMA_IER0 = crate::Reg<u32, _PDMA_IER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_IER0;
#[doc = "`read()` method returns [pdma_ier0::R](pdma_ier0::R) reader structure"]
impl crate::Readable for PDMA_IER0 {}
#[doc = "`write(|w| ..)` method takes [pdma_ier0::W](pdma_ier0::W) writer structure"]
impl crate::Writable for PDMA_IER0 {}
#[doc = "PDMA_IER0"]
pub mod pdma_ier0;
#[doc = "PDMA_IER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdma_ier1](pdma_ier1) module"]
pub type PDMA_IER1 = crate::Reg<u32, _PDMA_IER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMA_IER1;
#[doc = "`read()` method returns [pdma_ier1::R](pdma_ier1::R) reader structure"]
impl crate::Readable for PDMA_IER1 {}
#[doc = "`write(|w| ..)` method takes [pdma_ier1::W](pdma_ier1::W) writer structure"]
impl crate::Writable for PDMA_IER1 {}
#[doc = "PDMA_IER1"]
pub mod pdma_ier1;
