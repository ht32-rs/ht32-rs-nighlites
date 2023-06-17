#[doc = "Register `FMC_OCMR` reader"]
pub struct R(crate::R<FMC_OCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_OCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_OCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_OCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_OCMR` writer"]
pub struct W(crate::W<FMC_OCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_OCMR_SPEC>;
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
impl From<crate::W<FMC_OCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_OCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMD` reader - CMD"]
pub type CMD_R = crate::FieldReader;
#[doc = "Field `CMD` writer - CMD"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriter<'a, FMC_OCMR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - CMD"]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<0> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC_OCMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_ocmr](index.html) module"]
pub struct FMC_OCMR_SPEC;
impl crate::RegisterSpec for FMC_OCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_ocmr::R](R) reader structure"]
impl crate::Readable for FMC_OCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_ocmr::W](W) writer structure"]
impl crate::Writable for FMC_OCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_OCMR to value 0"]
impl crate::Resettable for FMC_OCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
