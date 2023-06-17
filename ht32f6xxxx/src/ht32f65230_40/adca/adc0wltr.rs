#[doc = "Register `ADC0WLTR` reader"]
pub struct R(crate::R<ADC0WLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0WLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0WLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0WLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0WLTR` writer"]
pub struct W(crate::W<ADC0WLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0WLTR_SPEC>;
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
impl From<crate::W<ADC0WLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0WLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0LT` reader - AD0LT"]
pub type AD0LT_R = crate::FieldReader<u16>;
#[doc = "Field `AD0LT` writer - AD0LT"]
pub type AD0LT_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0WLTR_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - AD0LT"]
    #[inline(always)]
    pub fn ad0lt(&self) -> AD0LT_R {
        AD0LT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - AD0LT"]
    #[inline(always)]
    #[must_use]
    pub fn ad0lt(&mut self) -> AD0LT_W<0> {
        AD0LT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0WLTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0wltr](index.html) module"]
pub struct ADC0WLTR_SPEC;
impl crate::RegisterSpec for ADC0WLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0wltr::R](R) reader structure"]
impl crate::Readable for ADC0WLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0wltr::W](W) writer structure"]
impl crate::Writable for ADC0WLTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0WLTR to value 0"]
impl crate::Resettable for ADC0WLTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
