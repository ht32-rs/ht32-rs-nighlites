#[doc = "Register `SCTM_EVGR` reader"]
pub struct R(crate::R<SCTM_EVGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCTM_EVGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCTM_EVGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCTM_EVGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCTM_EVGR` writer"]
pub struct W(crate::W<SCTM_EVGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCTM_EVGR_SPEC>;
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
impl From<crate::W<SCTM_EVGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCTM_EVGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHCCG` reader - CHCCG"]
pub type CHCCG_R = crate::BitReader;
#[doc = "Field `CHCCG` writer - CHCCG"]
pub type CHCCG_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_EVGR_SPEC, O>;
#[doc = "Field `UEVG` reader - UEVG"]
pub type UEVG_R = crate::BitReader;
#[doc = "Field `UEVG` writer - UEVG"]
pub type UEVG_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_EVGR_SPEC, O>;
#[doc = "Field `TEVG` reader - TEVG"]
pub type TEVG_R = crate::BitReader;
#[doc = "Field `TEVG` writer - TEVG"]
pub type TEVG_W<'a, const O: u8> = crate::BitWriter<'a, SCTM_EVGR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CHCCG"]
    #[inline(always)]
    pub fn chccg(&self) -> CHCCG_R {
        CHCCG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    pub fn uevg(&self) -> UEVG_R {
        UEVG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    pub fn tevg(&self) -> TEVG_R {
        TEVG_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHCCG"]
    #[inline(always)]
    #[must_use]
    pub fn chccg(&mut self) -> CHCCG_W<0> {
        CHCCG_W::new(self)
    }
    #[doc = "Bit 8 - UEVG"]
    #[inline(always)]
    #[must_use]
    pub fn uevg(&mut self) -> UEVG_W<8> {
        UEVG_W::new(self)
    }
    #[doc = "Bit 10 - TEVG"]
    #[inline(always)]
    #[must_use]
    pub fn tevg(&mut self) -> TEVG_W<10> {
        TEVG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCTM_EVGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sctm_evgr](index.html) module"]
pub struct SCTM_EVGR_SPEC;
impl crate::RegisterSpec for SCTM_EVGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sctm_evgr::R](R) reader structure"]
impl crate::Readable for SCTM_EVGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sctm_evgr::W](W) writer structure"]
impl crate::Writable for SCTM_EVGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCTM_EVGR to value 0"]
impl crate::Resettable for SCTM_EVGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
