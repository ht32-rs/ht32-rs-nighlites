#[doc = "Register `ADC1WUTR` reader"]
pub struct R(crate::R<ADC1WUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1WUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1WUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1WUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1WUTR` writer"]
pub struct W(crate::W<ADC1WUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1WUTR_SPEC>;
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
impl From<crate::W<ADC1WUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1WUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1UT` reader - AD1UT"]
pub type AD1UT_R = crate::FieldReader<u16>;
#[doc = "Field `AD1UT` writer - AD1UT"]
pub type AD1UT_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1WUTR_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - AD1UT"]
    #[inline(always)]
    pub fn ad1ut(&self) -> AD1UT_R {
        AD1UT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - AD1UT"]
    #[inline(always)]
    #[must_use]
    pub fn ad1ut(&mut self) -> AD1UT_W<0> {
        AD1UT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1WUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1wutr](index.html) module"]
pub struct ADC1WUTR_SPEC;
impl crate::RegisterSpec for ADC1WUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1wutr::R](R) reader structure"]
impl crate::Readable for ADC1WUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1wutr::W](W) writer structure"]
impl crate::Writable for ADC1WUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1WUTR to value 0"]
impl crate::Resettable for ADC1WUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
