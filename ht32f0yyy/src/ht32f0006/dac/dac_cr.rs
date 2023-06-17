#[doc = "Register `DAC_CR` reader"]
pub struct R(crate::R<DAC_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_CR` writer"]
pub struct W(crate::W<DAC_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CR_SPEC>;
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
impl From<crate::W<DAC_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELWR` reader - SELWR"]
pub type SELWR_R = crate::BitReader;
#[doc = "Field `SELWR` writer - SELWR"]
pub type SELWR_W<'a, const O: u8> = crate::BitWriter<'a, DAC_CR_SPEC, O>;
#[doc = "Field `SELWL` reader - SELWL"]
pub type SELWL_R = crate::BitReader;
#[doc = "Field `SELWL` writer - SELWL"]
pub type SELWL_W<'a, const O: u8> = crate::BitWriter<'a, DAC_CR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SELWR"]
    #[inline(always)]
    pub fn selwr(&self) -> SELWR_R {
        SELWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - SELWL"]
    #[inline(always)]
    pub fn selwl(&self) -> SELWL_R {
        SELWL_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SELWR"]
    #[inline(always)]
    #[must_use]
    pub fn selwr(&mut self) -> SELWR_W<0> {
        SELWR_W::new(self)
    }
    #[doc = "Bit 8 - SELWL"]
    #[inline(always)]
    #[must_use]
    pub fn selwl(&mut self) -> SELWL_W<8> {
        SELWL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC_CR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cr](index.html) module"]
pub struct DAC_CR_SPEC;
impl crate::RegisterSpec for DAC_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_cr::R](R) reader structure"]
impl crate::Readable for DAC_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_cr::W](W) writer structure"]
impl crate::Writable for DAC_CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_CR to value 0"]
impl crate::Resettable for DAC_CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
