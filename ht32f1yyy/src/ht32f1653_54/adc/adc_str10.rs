#[doc = "Register `ADC_STR10` reader"]
pub struct R(crate::R<ADC_STR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_STR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_STR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_STR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_STR10` writer"]
pub struct W(crate::W<ADC_STR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_STR10_SPEC>;
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
impl From<crate::W<ADC_STR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_STR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADST10` reader - ADST10"]
pub type ADST10_R = crate::FieldReader;
#[doc = "Field `ADST10` writer - ADST10"]
pub type ADST10_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_STR10_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADST10"]
    #[inline(always)]
    pub fn adst10(&self) -> ADST10_R {
        ADST10_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST10"]
    #[inline(always)]
    #[must_use]
    pub fn adst10(&mut self) -> ADST10_W<0> {
        ADST10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_STR10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str10](index.html) module"]
pub struct ADC_STR10_SPEC;
impl crate::RegisterSpec for ADC_STR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_str10::R](R) reader structure"]
impl crate::Readable for ADC_STR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_str10::W](W) writer structure"]
impl crate::Writable for ADC_STR10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_STR10 to value 0"]
impl crate::Resettable for ADC_STR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
