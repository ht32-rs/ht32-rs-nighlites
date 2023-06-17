#[doc = "Register `FMC_SBVT2` reader"]
pub struct R(crate::R<FMC_SBVT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_SBVT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_SBVT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_SBVT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_SBVT2` writer"]
pub struct W(crate::W<FMC_SBVT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_SBVT2_SPEC>;
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
impl From<crate::W<FMC_SBVT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_SBVT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBVT` reader - SBVT"]
pub type SBVT_R = crate::FieldReader<u32>;
#[doc = "Field `SBVT` writer - SBVT"]
pub type SBVT_W<'a, const O: u8> = crate::FieldWriter<'a, FMC_SBVT2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - SBVT"]
    #[inline(always)]
    pub fn sbvt(&self) -> SBVT_R {
        SBVT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SBVT"]
    #[inline(always)]
    #[must_use]
    pub fn sbvt(&mut self) -> SBVT_W<0> {
        SBVT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC_SBVT2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sbvt2](index.html) module"]
pub struct FMC_SBVT2_SPEC;
impl crate::RegisterSpec for FMC_SBVT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_sbvt2::R](R) reader structure"]
impl crate::Readable for FMC_SBVT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_sbvt2::W](W) writer structure"]
impl crate::Writable for FMC_SBVT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_SBVT2 to value 0"]
impl crate::Resettable for FMC_SBVT2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
