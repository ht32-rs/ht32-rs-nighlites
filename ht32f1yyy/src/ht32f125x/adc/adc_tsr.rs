#[doc = "Register `ADC_TSR` reader"]
pub struct R(crate::R<ADC_TSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_TSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_TSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_TSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_TSR` writer"]
pub struct W(crate::W<ADC_TSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_TSR_SPEC>;
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
impl From<crate::W<ADC_TSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_TSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSC` reader - ADSC"]
pub type ADSC_R = crate::BitReader;
#[doc = "Field `ADSC` writer - ADSC"]
pub type ADSC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_TSR_SPEC, O>;
#[doc = "Field `ADEXTIS` reader - ADEXTIS"]
pub type ADEXTIS_R = crate::FieldReader;
#[doc = "Field `ADEXTIS` writer - ADEXTIS"]
pub type ADEXTIS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_TSR_SPEC, 4, O>;
#[doc = "Field `GPTMS` reader - GPTMS"]
pub type GPTMS_R = crate::FieldReader;
#[doc = "Field `GPTMS` writer - GPTMS"]
pub type GPTMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_TSR_SPEC, 3, O>;
#[doc = "Field `GPTME` reader - GPTME"]
pub type GPTME_R = crate::FieldReader;
#[doc = "Field `GPTME` writer - GPTME"]
pub type GPTME_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_TSR_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    pub fn adsc(&self) -> ADSC_R {
        ADSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    pub fn adextis(&self) -> ADEXTIS_R {
        ADEXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - GPTMS"]
    #[inline(always)]
    pub fn gptms(&self) -> GPTMS_R {
        GPTMS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - GPTME"]
    #[inline(always)]
    pub fn gptme(&self) -> GPTME_R {
        GPTME_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADSC"]
    #[inline(always)]
    #[must_use]
    pub fn adsc(&mut self) -> ADSC_W<0> {
        ADSC_W::new(self)
    }
    #[doc = "Bits 8:11 - ADEXTIS"]
    #[inline(always)]
    #[must_use]
    pub fn adextis(&mut self) -> ADEXTIS_W<8> {
        ADEXTIS_W::new(self)
    }
    #[doc = "Bits 16:18 - GPTMS"]
    #[inline(always)]
    #[must_use]
    pub fn gptms(&mut self) -> GPTMS_W<16> {
        GPTMS_W::new(self)
    }
    #[doc = "Bits 24:26 - GPTME"]
    #[inline(always)]
    #[must_use]
    pub fn gptme(&mut self) -> GPTME_W<24> {
        GPTME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_TSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_tsr](index.html) module"]
pub struct ADC_TSR_SPEC;
impl crate::RegisterSpec for ADC_TSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_tsr::R](R) reader structure"]
impl crate::Readable for ADC_TSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_tsr::W](W) writer structure"]
impl crate::Writable for ADC_TSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_TSR to value 0"]
impl crate::Resettable for ADC_TSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
