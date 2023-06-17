#[doc = "Register `ADC_STR3` reader"]
pub struct R(crate::R<ADC_STR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_STR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_STR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_STR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_STR3` writer"]
pub struct W(crate::W<ADC_STR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_STR3_SPEC>;
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
impl From<crate::W<ADC_STR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_STR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADST3` reader - ADST3"]
pub type ADST3_R = crate::FieldReader;
#[doc = "Field `ADST3` writer - ADST3"]
pub type ADST3_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_STR3_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADST3"]
    #[inline(always)]
    pub fn adst3(&self) -> ADST3_R {
        ADST3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST3"]
    #[inline(always)]
    #[must_use]
    pub fn adst3(&mut self) -> ADST3_W<0> {
        ADST3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_STR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str3](index.html) module"]
pub struct ADC_STR3_SPEC;
impl crate::RegisterSpec for ADC_STR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_str3::R](R) reader structure"]
impl crate::Readable for ADC_STR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_str3::W](W) writer structure"]
impl crate::Writable for ADC_STR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_STR3 to value 0"]
impl crate::Resettable for ADC_STR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
