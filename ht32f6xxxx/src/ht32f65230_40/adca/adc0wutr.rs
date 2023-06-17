#[doc = "Register `ADC0WUTR` reader"]
pub struct R(crate::R<ADC0WUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0WUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0WUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0WUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0WUTR` writer"]
pub struct W(crate::W<ADC0WUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0WUTR_SPEC>;
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
impl From<crate::W<ADC0WUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0WUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0UT` reader - AD0UT"]
pub type AD0UT_R = crate::FieldReader<u16>;
#[doc = "Field `AD0UT` writer - AD0UT"]
pub type AD0UT_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0WUTR_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - AD0UT"]
    #[inline(always)]
    pub fn ad0ut(&self) -> AD0UT_R {
        AD0UT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - AD0UT"]
    #[inline(always)]
    #[must_use]
    pub fn ad0ut(&mut self) -> AD0UT_W<0> {
        AD0UT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0WUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0wutr](index.html) module"]
pub struct ADC0WUTR_SPEC;
impl crate::RegisterSpec for ADC0WUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0wutr::R](R) reader structure"]
impl crate::Readable for ADC0WUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0wutr::W](W) writer structure"]
impl crate::Writable for ADC0WUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0WUTR to value 0"]
impl crate::Resettable for ADC0WUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
