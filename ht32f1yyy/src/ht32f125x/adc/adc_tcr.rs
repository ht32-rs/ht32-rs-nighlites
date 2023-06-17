#[doc = "Register `ADC_TCR` reader"]
pub struct R(crate::R<ADC_TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_TCR` writer"]
pub struct W(crate::W<ADC_TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_TCR_SPEC>;
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
impl From<crate::W<ADC_TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSW` reader - ADSW"]
pub type ADSW_R = crate::BitReader;
#[doc = "Field `ADSW` writer - ADSW"]
pub type ADSW_W<'a, const O: u8> = crate::BitWriter<'a, ADC_TCR_SPEC, O>;
#[doc = "Field `ADEXTI` reader - ADEXTI"]
pub type ADEXTI_R = crate::BitReader;
#[doc = "Field `ADEXTI` writer - ADEXTI"]
pub type ADEXTI_W<'a, const O: u8> = crate::BitWriter<'a, ADC_TCR_SPEC, O>;
#[doc = "Field `GPTM` reader - GPTM"]
pub type GPTM_R = crate::BitReader;
#[doc = "Field `GPTM` writer - GPTM"]
pub type GPTM_W<'a, const O: u8> = crate::BitWriter<'a, ADC_TCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    pub fn adsw(&self) -> ADSW_R {
        ADSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    pub fn adexti(&self) -> ADEXTI_R {
        ADEXTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPTM"]
    #[inline(always)]
    pub fn gptm(&self) -> GPTM_R {
        GPTM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADSW"]
    #[inline(always)]
    #[must_use]
    pub fn adsw(&mut self) -> ADSW_W<0> {
        ADSW_W::new(self)
    }
    #[doc = "Bit 1 - ADEXTI"]
    #[inline(always)]
    #[must_use]
    pub fn adexti(&mut self) -> ADEXTI_W<1> {
        ADEXTI_W::new(self)
    }
    #[doc = "Bit 2 - GPTM"]
    #[inline(always)]
    #[must_use]
    pub fn gptm(&mut self) -> GPTM_W<2> {
        GPTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_tcr](index.html) module"]
pub struct ADC_TCR_SPEC;
impl crate::RegisterSpec for ADC_TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_tcr::R](R) reader structure"]
impl crate::Readable for ADC_TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_tcr::W](W) writer structure"]
impl crate::Writable for ADC_TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_TCR to value 0"]
impl crate::Resettable for ADC_TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
