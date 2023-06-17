#[doc = "Register `FMC_CPSR` reader"]
pub struct R(crate::R<FMC_CPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_CPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_CPSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_CPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMC_CPSR` writer"]
pub struct W(crate::W<FMC_CPSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMC_CPSR_SPEC>;
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
impl From<crate::W<FMC_CPSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMC_CPSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPSB` reader - CPSB"]
pub type CPSB_R = crate::BitReader;
#[doc = "Field `CPSB` writer - CPSB"]
pub type CPSB_W<'a, const O: u8> = crate::BitWriter<'a, FMC_CPSR_SPEC, O>;
#[doc = "Field `OBPSB` reader - OBPSB"]
pub type OBPSB_R = crate::BitReader;
#[doc = "Field `OBPSB` writer - OBPSB"]
pub type OBPSB_W<'a, const O: u8> = crate::BitWriter<'a, FMC_CPSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CPSB"]
    #[inline(always)]
    pub fn cpsb(&self) -> CPSB_R {
        CPSB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OBPSB"]
    #[inline(always)]
    pub fn obpsb(&self) -> OBPSB_R {
        OBPSB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPSB"]
    #[inline(always)]
    #[must_use]
    pub fn cpsb(&mut self) -> CPSB_W<0> {
        CPSB_W::new(self)
    }
    #[doc = "Bit 1 - OBPSB"]
    #[inline(always)]
    #[must_use]
    pub fn obpsb(&mut self) -> OBPSB_W<1> {
        OBPSB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMC_CPSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_cpsr](index.html) module"]
pub struct FMC_CPSR_SPEC;
impl crate::RegisterSpec for FMC_CPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_cpsr::R](R) reader structure"]
impl crate::Readable for FMC_CPSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fmc_cpsr::W](W) writer structure"]
impl crate::Writable for FMC_CPSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMC_CPSR to value 0"]
impl crate::Resettable for FMC_CPSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
