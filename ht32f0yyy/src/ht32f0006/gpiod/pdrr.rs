#[doc = "Register `PDRR` reader"]
pub struct R(crate::R<PDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDRR` writer"]
pub struct W(crate::W<PDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRR_SPEC>;
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
impl From<crate::W<PDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDRST` reader - PDRST"]
pub type PDRST_R = crate::FieldReader;
#[doc = "Field `PDRST` writer - PDRST"]
pub type PDRST_W<'a, const O: u8> = crate::FieldWriter<'a, PDRR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDRST"]
    #[inline(always)]
    pub fn pdrst(&self) -> PDRST_R {
        PDRST_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDRST"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst(&mut self) -> PDRST_W<0> {
        PDRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdrr](index.html) module"]
pub struct PDRR_SPEC;
impl crate::RegisterSpec for PDRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdrr::R](R) reader structure"]
impl crate::Readable for PDRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdrr::W](W) writer structure"]
impl crate::Writable for PDRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDRR to value 0"]
impl crate::Resettable for PDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
