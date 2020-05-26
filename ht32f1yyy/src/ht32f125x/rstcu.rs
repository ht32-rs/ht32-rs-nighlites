#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RSTCU_GRSR"]
    pub rstcu_grsr: RSTCU_GRSR,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - RSTCU_APBPRSTR0"]
    pub rstcu_apbprstr0: RSTCU_APBPRSTR0,
    #[doc = "0x0c - RSTCU_APBPRSTR1"]
    pub rstcu_apbprstr1: RSTCU_APBPRSTR1,
}
#[doc = "RSTCU_GRSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcu_grsr](rstcu_grsr) module"]
pub type RSTCU_GRSR = crate::Reg<u32, _RSTCU_GRSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCU_GRSR;
#[doc = "`read()` method returns [rstcu_grsr::R](rstcu_grsr::R) reader structure"]
impl crate::Readable for RSTCU_GRSR {}
#[doc = "`write(|w| ..)` method takes [rstcu_grsr::W](rstcu_grsr::W) writer structure"]
impl crate::Writable for RSTCU_GRSR {}
#[doc = "RSTCU_GRSR"]
pub mod rstcu_grsr;
#[doc = "RSTCU_APBPRSTR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcu_apbprstr0](rstcu_apbprstr0) module"]
pub type RSTCU_APBPRSTR0 = crate::Reg<u32, _RSTCU_APBPRSTR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCU_APBPRSTR0;
#[doc = "`read()` method returns [rstcu_apbprstr0::R](rstcu_apbprstr0::R) reader structure"]
impl crate::Readable for RSTCU_APBPRSTR0 {}
#[doc = "`write(|w| ..)` method takes [rstcu_apbprstr0::W](rstcu_apbprstr0::W) writer structure"]
impl crate::Writable for RSTCU_APBPRSTR0 {}
#[doc = "RSTCU_APBPRSTR0"]
pub mod rstcu_apbprstr0;
#[doc = "RSTCU_APBPRSTR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcu_apbprstr1](rstcu_apbprstr1) module"]
pub type RSTCU_APBPRSTR1 = crate::Reg<u32, _RSTCU_APBPRSTR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCU_APBPRSTR1;
#[doc = "`read()` method returns [rstcu_apbprstr1::R](rstcu_apbprstr1::R) reader structure"]
impl crate::Readable for RSTCU_APBPRSTR1 {}
#[doc = "`write(|w| ..)` method takes [rstcu_apbprstr1::W](rstcu_apbprstr1::W) writer structure"]
impl crate::Writable for RSTCU_APBPRSTR1 {}
#[doc = "RSTCU_APBPRSTR1"]
pub mod rstcu_apbprstr1;
