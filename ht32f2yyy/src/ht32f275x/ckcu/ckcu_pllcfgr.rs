#[doc = "Register `CKCU_PLLCFGR` reader"]
pub struct R(crate::R<CKCU_PLLCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_PLLCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_PLLCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_PLLCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_PLLCFGR` writer"]
pub struct W(crate::W<CKCU_PLLCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_PLLCFGR_SPEC>;
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
impl From<crate::W<CKCU_PLLCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_PLLCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POTD` reader - POTD"]
pub type POTD_R = crate::FieldReader;
#[doc = "Field `POTD` writer - POTD"]
pub type POTD_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_PLLCFGR_SPEC, 2, O>;
#[doc = "Field `PFBD` reader - PFBD"]
pub type PFBD_R = crate::FieldReader;
#[doc = "Field `PFBD` writer - PFBD"]
pub type PFBD_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_PLLCFGR_SPEC, 6, O>;
impl R {
    #[doc = "Bits 21:22 - POTD"]
    #[inline(always)]
    pub fn potd(&self) -> POTD_R {
        POTD_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:28 - PFBD"]
    #[inline(always)]
    pub fn pfbd(&self) -> PFBD_R {
        PFBD_R::new(((self.bits >> 23) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 21:22 - POTD"]
    #[inline(always)]
    #[must_use]
    pub fn potd(&mut self) -> POTD_W<21> {
        POTD_W::new(self)
    }
    #[doc = "Bits 23:28 - PFBD"]
    #[inline(always)]
    #[must_use]
    pub fn pfbd(&mut self) -> PFBD_W<23> {
        PFBD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_PLLCFGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_pllcfgr](index.html) module"]
pub struct CKCU_PLLCFGR_SPEC;
impl crate::RegisterSpec for CKCU_PLLCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_pllcfgr::R](R) reader structure"]
impl crate::Readable for CKCU_PLLCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_pllcfgr::W](W) writer structure"]
impl crate::Writable for CKCU_PLLCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_PLLCFGR to value 0"]
impl crate::Resettable for CKCU_PLLCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
