#[doc = "Register `FMC_VMCR` reader"]
pub struct R(crate::R<FMC_VMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_VMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_VMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_VMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_VMCR` writer"]
pub struct W(crate::W<FMC_VMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_VMCR_SPEC>;
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
impl From<crate::W<FMC_VMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_VMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VMCB` reader - VMCB"]
pub type VMCB_R = crate::BitReader;
#[doc = "Field `VMCB` writer - VMCB"]
pub type VMCB_W<'a, const O: u8> = crate::BitWriter<'a, FMC_VMCR_SPEC, O>;
impl R {
    #[doc = "Bit 1 - VMCB"]
    #[inline(always)]
    pub fn vmcb(&self) -> VMCB_R {
        VMCB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VMCB"]
    #[inline(always)]
    #[must_use]
    pub fn vmcb(&mut self) -> VMCB_W<1> {
        VMCB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC_VMCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_vmcr](index.html) module"]
pub struct FMC_VMCR_SPEC;
impl crate::RegisterSpec for FMC_VMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_vmcr::R](R) reader structure"]
impl crate::Readable for FMC_VMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_vmcr::W](W) writer structure"]
impl crate::Writable for FMC_VMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_VMCR to value 0"]
impl crate::Resettable for FMC_VMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
