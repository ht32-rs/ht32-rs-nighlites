#[doc = "Register `PFRR` reader"]
pub struct R(crate::R<PFRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFRR` writer"]
pub struct W(crate::W<PFRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFRR_SPEC>;
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
impl From<crate::W<PFRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFRST0` reader - PFRST0"]
pub type PFRST0_R = crate::BitReader;
#[doc = "Field `PFRST0` writer - PFRST0"]
pub type PFRST0_W<'a, const O: u8> = crate::BitWriter<'a, PFRR_SPEC, O>;
#[doc = "Field `PFRST1` reader - PFRST1"]
pub type PFRST1_R = crate::BitReader;
#[doc = "Field `PFRST1` writer - PFRST1"]
pub type PFRST1_W<'a, const O: u8> = crate::BitWriter<'a, PFRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PFRST0"]
    #[inline(always)]
    pub fn pfrst0(&self) -> PFRST0_R {
        PFRST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PFRST1"]
    #[inline(always)]
    pub fn pfrst1(&self) -> PFRST1_R {
        PFRST1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PFRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pfrst0(&mut self) -> PFRST0_W<0> {
        PFRST0_W::new(self)
    }
    #[doc = "Bit 1 - PFRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pfrst1(&mut self) -> PFRST1_W<1> {
        PFRST1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfrr](index.html) module"]
pub struct PFRR_SPEC;
impl crate::RegisterSpec for PFRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfrr::R](R) reader structure"]
impl crate::Readable for PFRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfrr::W](W) writer structure"]
impl crate::Writable for PFRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFRR to value 0"]
impl crate::Resettable for PFRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
