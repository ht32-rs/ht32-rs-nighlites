#[doc = "Register `ADC_LTR` reader"]
pub struct R(crate::R<ADC_LTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_LTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_LTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_LTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_LTR` writer"]
pub struct W(crate::W<ADC_LTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_LTR_SPEC>;
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
impl From<crate::W<ADC_LTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_LTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADLT` reader - ADLT"]
pub type ADLT_R = crate::FieldReader<u16>;
#[doc = "Field `ADLT` writer - ADLT"]
pub type ADLT_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LTR_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - ADLT"]
    #[inline(always)]
    pub fn adlt(&self) -> ADLT_R {
        ADLT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADLT"]
    #[inline(always)]
    #[must_use]
    pub fn adlt(&mut self) -> ADLT_W<0> {
        ADLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_LTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ltr](index.html) module"]
pub struct ADC_LTR_SPEC;
impl crate::RegisterSpec for ADC_LTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ltr::R](R) reader structure"]
impl crate::Readable for ADC_LTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ltr::W](W) writer structure"]
impl crate::Writable for ADC_LTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_LTR to value 0"]
impl crate::Resettable for ADC_LTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
