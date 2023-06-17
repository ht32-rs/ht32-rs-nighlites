#[doc = "Register `FMC_PSSR` reader"]
pub struct R(crate::R<FMC_PSSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_PSSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_PSSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_PSSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_PSSR` writer"]
pub struct W(crate::W<FMC_PSSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_PSSR_SPEC>;
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
impl From<crate::W<FMC_PSSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_PSSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSSB` reader - PSSB"]
pub type PSSB_R = crate::FieldReader<u32>;
#[doc = "Field `PSSB` writer - PSSB"]
pub type PSSB_W<'a, const O: u8> = crate::FieldWriter<'a, FMC_PSSR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - PSSB"]
    #[inline(always)]
    pub fn pssb(&self) -> PSSB_R {
        PSSB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PSSB"]
    #[inline(always)]
    #[must_use]
    pub fn pssb(&mut self) -> PSSB_W<0> {
        PSSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC_PSSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_pssr](index.html) module"]
pub struct FMC_PSSR_SPEC;
impl crate::RegisterSpec for FMC_PSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_pssr::R](R) reader structure"]
impl crate::Readable for FMC_PSSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_pssr::W](W) writer structure"]
impl crate::Writable for FMC_PSSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_PSSR to value 0"]
impl crate::Resettable for FMC_PSSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
