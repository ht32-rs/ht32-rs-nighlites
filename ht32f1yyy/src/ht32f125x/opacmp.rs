#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - OPACR0"]
    pub opacr0: OPACR0,
    #[doc = "0x04 - OFVCR0"]
    pub ofvcr0: OFVCR0,
    #[doc = "0x08 - CMPIER0"]
    pub cmpier0: CMPIER0,
    #[doc = "0x0c - CMPRSR0"]
    pub cmprsr0: CMPRSR0,
    #[doc = "0x10 - CMPISR0"]
    pub cmpisr0: CMPISR0,
    #[doc = "0x14 - CMPICLR0"]
    pub cmpiclr0: CMPICLR0,
    _reserved6: [u8; 232usize],
    #[doc = "0x100 - OPACR1"]
    pub opacr1: OPACR1,
    #[doc = "0x104 - OFVCR1"]
    pub ofvcr1: OFVCR1,
    #[doc = "0x108 - CMPIER1"]
    pub cmpier1: CMPIER1,
    #[doc = "0x10c - CMPRSR1"]
    pub cmprsr1: CMPRSR1,
    #[doc = "0x110 - CMPISR1"]
    pub cmpisr1: CMPISR1,
    #[doc = "0x114 - CMPICLR1"]
    pub cmpiclr1: CMPICLR1,
}
#[doc = "OPACR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr0](opacr0) module"]
pub type OPACR0 = crate::Reg<u32, _OPACR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACR0;
#[doc = "`read()` method returns [opacr0::R](opacr0::R) reader structure"]
impl crate::Readable for OPACR0 {}
#[doc = "`write(|w| ..)` method takes [opacr0::W](opacr0::W) writer structure"]
impl crate::Writable for OPACR0 {}
#[doc = "OPACR0"]
pub mod opacr0;
#[doc = "OFVCR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofvcr0](ofvcr0) module"]
pub type OFVCR0 = crate::Reg<u32, _OFVCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFVCR0;
#[doc = "`read()` method returns [ofvcr0::R](ofvcr0::R) reader structure"]
impl crate::Readable for OFVCR0 {}
#[doc = "`write(|w| ..)` method takes [ofvcr0::W](ofvcr0::W) writer structure"]
impl crate::Writable for OFVCR0 {}
#[doc = "OFVCR0"]
pub mod ofvcr0;
#[doc = "CMPIER0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpier0](cmpier0) module"]
pub type CMPIER0 = crate::Reg<u32, _CMPIER0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPIER0;
#[doc = "`read()` method returns [cmpier0::R](cmpier0::R) reader structure"]
impl crate::Readable for CMPIER0 {}
#[doc = "`write(|w| ..)` method takes [cmpier0::W](cmpier0::W) writer structure"]
impl crate::Writable for CMPIER0 {}
#[doc = "CMPIER0"]
pub mod cmpier0;
#[doc = "CMPRSR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprsr0](cmprsr0) module"]
pub type CMPRSR0 = crate::Reg<u32, _CMPRSR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRSR0;
#[doc = "`read()` method returns [cmprsr0::R](cmprsr0::R) reader structure"]
impl crate::Readable for CMPRSR0 {}
#[doc = "`write(|w| ..)` method takes [cmprsr0::W](cmprsr0::W) writer structure"]
impl crate::Writable for CMPRSR0 {}
#[doc = "CMPRSR0"]
pub mod cmprsr0;
#[doc = "CMPISR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpisr0](cmpisr0) module"]
pub type CMPISR0 = crate::Reg<u32, _CMPISR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPISR0;
#[doc = "`read()` method returns [cmpisr0::R](cmpisr0::R) reader structure"]
impl crate::Readable for CMPISR0 {}
#[doc = "`write(|w| ..)` method takes [cmpisr0::W](cmpisr0::W) writer structure"]
impl crate::Writable for CMPISR0 {}
#[doc = "CMPISR0"]
pub mod cmpisr0;
#[doc = "CMPICLR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpiclr0](cmpiclr0) module"]
pub type CMPICLR0 = crate::Reg<u32, _CMPICLR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPICLR0;
#[doc = "`read()` method returns [cmpiclr0::R](cmpiclr0::R) reader structure"]
impl crate::Readable for CMPICLR0 {}
#[doc = "`write(|w| ..)` method takes [cmpiclr0::W](cmpiclr0::W) writer structure"]
impl crate::Writable for CMPICLR0 {}
#[doc = "CMPICLR0"]
pub mod cmpiclr0;
#[doc = "OPACR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr1](opacr1) module"]
pub type OPACR1 = crate::Reg<u32, _OPACR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OPACR1;
#[doc = "`read()` method returns [opacr1::R](opacr1::R) reader structure"]
impl crate::Readable for OPACR1 {}
#[doc = "`write(|w| ..)` method takes [opacr1::W](opacr1::W) writer structure"]
impl crate::Writable for OPACR1 {}
#[doc = "OPACR1"]
pub mod opacr1;
#[doc = "OFVCR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofvcr1](ofvcr1) module"]
pub type OFVCR1 = crate::Reg<u32, _OFVCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OFVCR1;
#[doc = "`read()` method returns [ofvcr1::R](ofvcr1::R) reader structure"]
impl crate::Readable for OFVCR1 {}
#[doc = "`write(|w| ..)` method takes [ofvcr1::W](ofvcr1::W) writer structure"]
impl crate::Writable for OFVCR1 {}
#[doc = "OFVCR1"]
pub mod ofvcr1;
#[doc = "CMPIER1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpier1](cmpier1) module"]
pub type CMPIER1 = crate::Reg<u32, _CMPIER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPIER1;
#[doc = "`read()` method returns [cmpier1::R](cmpier1::R) reader structure"]
impl crate::Readable for CMPIER1 {}
#[doc = "`write(|w| ..)` method takes [cmpier1::W](cmpier1::W) writer structure"]
impl crate::Writable for CMPIER1 {}
#[doc = "CMPIER1"]
pub mod cmpier1;
#[doc = "CMPRSR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmprsr1](cmprsr1) module"]
pub type CMPRSR1 = crate::Reg<u32, _CMPRSR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPRSR1;
#[doc = "`read()` method returns [cmprsr1::R](cmprsr1::R) reader structure"]
impl crate::Readable for CMPRSR1 {}
#[doc = "`write(|w| ..)` method takes [cmprsr1::W](cmprsr1::W) writer structure"]
impl crate::Writable for CMPRSR1 {}
#[doc = "CMPRSR1"]
pub mod cmprsr1;
#[doc = "CMPISR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpisr1](cmpisr1) module"]
pub type CMPISR1 = crate::Reg<u32, _CMPISR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPISR1;
#[doc = "`read()` method returns [cmpisr1::R](cmpisr1::R) reader structure"]
impl crate::Readable for CMPISR1 {}
#[doc = "`write(|w| ..)` method takes [cmpisr1::W](cmpisr1::W) writer structure"]
impl crate::Writable for CMPISR1 {}
#[doc = "CMPISR1"]
pub mod cmpisr1;
#[doc = "CMPICLR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpiclr1](cmpiclr1) module"]
pub type CMPICLR1 = crate::Reg<u32, _CMPICLR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMPICLR1;
#[doc = "`read()` method returns [cmpiclr1::R](cmpiclr1::R) reader structure"]
impl crate::Readable for CMPICLR1 {}
#[doc = "`write(|w| ..)` method takes [cmpiclr1::W](cmpiclr1::W) writer structure"]
impl crate::Writable for CMPICLR1 {}
#[doc = "CMPICLR1"]
pub mod cmpiclr1;
