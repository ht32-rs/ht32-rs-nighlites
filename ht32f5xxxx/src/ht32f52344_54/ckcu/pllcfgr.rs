#[doc = "Register `PLLCFGR` reader"]
pub struct R(crate::R<PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLCFGR` writer"]
pub struct W(crate::W<PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBPOTD` reader - USBPOTD"]
pub type USBPOTD_R = crate::FieldReader;
#[doc = "Field `USBPOTD` writer - USBPOTD"]
pub type USBPOTD_W<'a, const O: u8> = crate::FieldWriter<'a, PLLCFGR_SPEC, 2, O>;
#[doc = "Field `USBPFBD` reader - USBPFBD"]
pub type USBPFBD_R = crate::FieldReader;
#[doc = "Field `USBPFBD` writer - USBPFBD"]
pub type USBPFBD_W<'a, const O: u8> = crate::FieldWriter<'a, PLLCFGR_SPEC, 5, O>;
#[doc = "Field `POTD` reader - POTD"]
pub type POTD_R = crate::FieldReader;
#[doc = "Field `POTD` writer - POTD"]
pub type POTD_W<'a, const O: u8> = crate::FieldWriter<'a, PLLCFGR_SPEC, 2, O>;
#[doc = "Field `PFBD` reader - PFBD"]
pub type PFBD_R = crate::FieldReader;
#[doc = "Field `PFBD` writer - PFBD"]
pub type PFBD_W<'a, const O: u8> = crate::FieldWriter<'a, PLLCFGR_SPEC, 4, O>;
#[doc = "Field `REFDIV` reader - REFDIV"]
pub type REFDIV_R = crate::BitReader;
#[doc = "Field `REFDIV` writer - REFDIV"]
pub type REFDIV_W<'a, const O: u8> = crate::BitWriter<'a, PLLCFGR_SPEC, O>;
impl R {
    #[doc = "Bits 5:6 - USBPOTD"]
    #[inline(always)]
    pub fn usbpotd(&self) -> USBPOTD_R {
        USBPOTD_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 7:11 - USBPFBD"]
    #[inline(always)]
    pub fn usbpfbd(&self) -> USBPFBD_R {
        USBPFBD_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 21:22 - POTD"]
    #[inline(always)]
    pub fn potd(&self) -> POTD_R {
        POTD_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:26 - PFBD"]
    #[inline(always)]
    pub fn pfbd(&self) -> PFBD_R {
        PFBD_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - REFDIV"]
    #[inline(always)]
    pub fn refdiv(&self) -> REFDIV_R {
        REFDIV_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 5:6 - USBPOTD"]
    #[inline(always)]
    #[must_use]
    pub fn usbpotd(&mut self) -> USBPOTD_W<5> {
        USBPOTD_W::new(self)
    }
    #[doc = "Bits 7:11 - USBPFBD"]
    #[inline(always)]
    #[must_use]
    pub fn usbpfbd(&mut self) -> USBPFBD_W<7> {
        USBPFBD_W::new(self)
    }
    #[doc = "Bits 21:22 - POTD"]
    #[inline(always)]
    #[must_use]
    pub fn potd(&mut self) -> POTD_W<21> {
        POTD_W::new(self)
    }
    #[doc = "Bits 23:26 - PFBD"]
    #[inline(always)]
    #[must_use]
    pub fn pfbd(&mut self) -> PFBD_W<23> {
        PFBD_W::new(self)
    }
    #[doc = "Bit 28 - REFDIV"]
    #[inline(always)]
    #[must_use]
    pub fn refdiv(&mut self) -> REFDIV_W<28> {
        REFDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfgr](index.html) module"]
pub struct PLLCFGR_SPEC;
impl crate::RegisterSpec for PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pllcfgr::R](R) reader structure"]
impl crate::Readable for PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pllcfgr::W](W) writer structure"]
impl crate::Writable for PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLLCFGR to value 0"]
impl crate::Resettable for PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
