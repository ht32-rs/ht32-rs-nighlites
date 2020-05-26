#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_bfsr: [u8; 4usize],
    #[doc = "0x04 - HFSR"]
    pub hfsr: HFSR,
    #[doc = "0x08 - DFSR"]
    pub dfsr: DFSR,
    #[doc = "0x0c - MMFAR"]
    pub mmfar: MMFAR,
    #[doc = "0x10 - BFAR"]
    pub bfar: BFAR,
    #[doc = "0x14 - AFSR"]
    pub afsr: AFSR,
}
impl RegisterBlock {
    #[doc = "0x00 - MFSR"]
    #[inline(always)]
    pub fn mfsr(&self) -> &MFSR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const MFSR) }
    }
    #[doc = "0x00 - MFSR"]
    #[inline(always)]
    pub fn mfsr_mut(&self) -> &mut MFSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut MFSR) }
    }
    #[doc = "0x00 - CFSR"]
    #[inline(always)]
    pub fn cfsr(&self) -> &CFSR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const CFSR) }
    }
    #[doc = "0x00 - CFSR"]
    #[inline(always)]
    pub fn cfsr_mut(&self) -> &mut CFSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut CFSR) }
    }
    #[doc = "0x01 - BFSR"]
    #[inline(always)]
    pub fn bfsr(&self) -> &BFSR {
        unsafe { &*(((self as *const Self) as *const u8).add(1usize) as *const BFSR) }
    }
    #[doc = "0x01 - BFSR"]
    #[inline(always)]
    pub fn bfsr_mut(&self) -> &mut BFSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1usize) as *mut BFSR) }
    }
    #[doc = "0x02 - UFSR"]
    #[inline(always)]
    pub fn ufsr(&self) -> &UFSR {
        unsafe { &*(((self as *const Self) as *const u8).add(2usize) as *const UFSR) }
    }
    #[doc = "0x02 - UFSR"]
    #[inline(always)]
    pub fn ufsr_mut(&self) -> &mut UFSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2usize) as *mut UFSR) }
    }
}
#[doc = "CFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfsr](cfsr) module"]
pub type CFSR = crate::Reg<u32, _CFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFSR;
#[doc = "`read()` method returns [cfsr::R](cfsr::R) reader structure"]
impl crate::Readable for CFSR {}
#[doc = "`write(|w| ..)` method takes [cfsr::W](cfsr::W) writer structure"]
impl crate::Writable for CFSR {}
#[doc = "CFSR"]
pub mod cfsr;
#[doc = "MFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfsr](mfsr) module"]
pub type MFSR = crate::Reg<u8, _MFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MFSR;
#[doc = "`read()` method returns [mfsr::R](mfsr::R) reader structure"]
impl crate::Readable for MFSR {}
#[doc = "`write(|w| ..)` method takes [mfsr::W](mfsr::W) writer structure"]
impl crate::Writable for MFSR {}
#[doc = "MFSR"]
pub mod mfsr;
#[doc = "MMFAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmfar](mmfar) module"]
pub type MMFAR = crate::Reg<u32, _MMFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMFAR;
#[doc = "`read()` method returns [mmfar::R](mmfar::R) reader structure"]
impl crate::Readable for MMFAR {}
#[doc = "`write(|w| ..)` method takes [mmfar::W](mmfar::W) writer structure"]
impl crate::Writable for MMFAR {}
#[doc = "MMFAR"]
pub mod mmfar;
#[doc = "BFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfsr](bfsr) module"]
pub type BFSR = crate::Reg<u8, _BFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFSR;
#[doc = "`read()` method returns [bfsr::R](bfsr::R) reader structure"]
impl crate::Readable for BFSR {}
#[doc = "`write(|w| ..)` method takes [bfsr::W](bfsr::W) writer structure"]
impl crate::Writable for BFSR {}
#[doc = "BFSR"]
pub mod bfsr;
#[doc = "BFAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfar](bfar) module"]
pub type BFAR = crate::Reg<u32, _BFAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFAR;
#[doc = "`read()` method returns [bfar::R](bfar::R) reader structure"]
impl crate::Readable for BFAR {}
#[doc = "`write(|w| ..)` method takes [bfar::W](bfar::W) writer structure"]
impl crate::Writable for BFAR {}
#[doc = "BFAR"]
pub mod bfar;
#[doc = "UFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ufsr](ufsr) module"]
pub type UFSR = crate::Reg<u16, _UFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UFSR;
#[doc = "`read()` method returns [ufsr::R](ufsr::R) reader structure"]
impl crate::Readable for UFSR {}
#[doc = "`write(|w| ..)` method takes [ufsr::W](ufsr::W) writer structure"]
impl crate::Writable for UFSR {}
#[doc = "UFSR"]
pub mod ufsr;
#[doc = "HFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfsr](hfsr) module"]
pub type HFSR = crate::Reg<u32, _HFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFSR;
#[doc = "`read()` method returns [hfsr::R](hfsr::R) reader structure"]
impl crate::Readable for HFSR {}
#[doc = "`write(|w| ..)` method takes [hfsr::W](hfsr::W) writer structure"]
impl crate::Writable for HFSR {}
#[doc = "HFSR"]
pub mod hfsr;
#[doc = "DFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsr](dfsr) module"]
pub type DFSR = crate::Reg<u32, _DFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DFSR;
#[doc = "`read()` method returns [dfsr::R](dfsr::R) reader structure"]
impl crate::Readable for DFSR {}
#[doc = "`write(|w| ..)` method takes [dfsr::W](dfsr::W) writer structure"]
impl crate::Writable for DFSR {}
#[doc = "DFSR"]
pub mod dfsr;
#[doc = "AFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afsr](afsr) module"]
pub type AFSR = crate::Reg<u32, _AFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFSR;
#[doc = "`read()` method returns [afsr::R](afsr::R) reader structure"]
impl crate::Readable for AFSR {}
#[doc = "`write(|w| ..)` method takes [afsr::W](afsr::W) writer structure"]
impl crate::Writable for AFSR {}
#[doc = "AFSR"]
pub mod afsr;
