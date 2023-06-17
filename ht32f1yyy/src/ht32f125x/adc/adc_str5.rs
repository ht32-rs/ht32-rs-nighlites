#[doc = "Register `ADC_STR5` reader"]
pub struct R(crate::R<ADC_STR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_STR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_STR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_STR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_STR5` writer"]
pub struct W(crate::W<ADC_STR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_STR5_SPEC>;
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
impl From<crate::W<ADC_STR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_STR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADST5` reader - ADST5"]
pub type ADST5_R = crate::FieldReader;
#[doc = "Field `ADST5` writer - ADST5"]
pub type ADST5_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_STR5_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADST5"]
    #[inline(always)]
    pub fn adst5(&self) -> ADST5_R {
        ADST5_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST5"]
    #[inline(always)]
    #[must_use]
    pub fn adst5(&mut self) -> ADST5_W<0> {
        ADST5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_STR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str5](index.html) module"]
pub struct ADC_STR5_SPEC;
impl crate::RegisterSpec for ADC_STR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_str5::R](R) reader structure"]
impl crate::Readable for ADC_STR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_str5::W](W) writer structure"]
impl crate::Writable for ADC_STR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_STR5 to value 0"]
impl crate::Resettable for ADC_STR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
