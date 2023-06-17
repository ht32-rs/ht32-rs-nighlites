#[doc = "Register `ADC_UTR` reader"]
pub struct R(crate::R<ADC_UTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_UTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_UTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_UTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_UTR` writer"]
pub struct W(crate::W<ADC_UTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_UTR_SPEC>;
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
impl From<crate::W<ADC_UTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_UTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADUT` reader - ADUT"]
pub type ADUT_R = crate::FieldReader<u16>;
#[doc = "Field `ADUT` writer - ADUT"]
pub type ADUT_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_UTR_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - ADUT"]
    #[inline(always)]
    pub fn adut(&self) -> ADUT_R {
        ADUT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADUT"]
    #[inline(always)]
    #[must_use]
    pub fn adut(&mut self) -> ADUT_W<0> {
        ADUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_UTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_utr](index.html) module"]
pub struct ADC_UTR_SPEC;
impl crate::RegisterSpec for ADC_UTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_utr::R](R) reader structure"]
impl crate::Readable for ADC_UTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_utr::W](W) writer structure"]
impl crate::Writable for ADC_UTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_UTR to value 0"]
impl crate::Resettable for ADC_UTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
