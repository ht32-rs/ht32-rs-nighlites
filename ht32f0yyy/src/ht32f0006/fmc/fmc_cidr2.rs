#[doc = "Register `FMC_CIDR2` reader"]
pub struct R(crate::R<FMC_CIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CIDR2` writer"]
pub struct W(crate::W<FMC_CIDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CIDR2_SPEC>;
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
impl From<crate::W<FMC_CIDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CIDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CIDn` reader - CIDn"]
pub type CIDN_R = crate::FieldReader<u32>;
#[doc = "Field `CIDn` writer - CIDn"]
pub type CIDN_W<'a, const O: u8> = crate::FieldWriter<'a, FMC_CIDR2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - CIDn"]
    #[inline(always)]
    pub fn cidn(&self) -> CIDN_R {
        CIDN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CIDn"]
    #[inline(always)]
    #[must_use]
    pub fn cidn(&mut self) -> CIDN_W<0> {
        CIDN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC_CIDR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_cidr2](index.html) module"]
pub struct FMC_CIDR2_SPEC;
impl crate::RegisterSpec for FMC_CIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_cidr2::R](R) reader structure"]
impl crate::Readable for FMC_CIDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_cidr2::W](W) writer structure"]
impl crate::Writable for FMC_CIDR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CIDR2 to value 0"]
impl crate::Resettable for FMC_CIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
