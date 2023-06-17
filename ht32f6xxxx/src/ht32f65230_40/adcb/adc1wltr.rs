#[doc = "Register `ADC1WLTR` reader"]
pub struct R(crate::R<ADC1WLTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1WLTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1WLTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1WLTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1WLTR` writer"]
pub struct W(crate::W<ADC1WLTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1WLTR_SPEC>;
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
impl From<crate::W<ADC1WLTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1WLTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1LT` reader - AD1LT"]
pub type AD1LT_R = crate::FieldReader<u16>;
#[doc = "Field `AD1LT` writer - AD1LT"]
pub type AD1LT_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1WLTR_SPEC, 12, O, u16>;
impl R {
    #[doc = "Bits 0:11 - AD1LT"]
    #[inline(always)]
    pub fn ad1lt(&self) -> AD1LT_R {
        AD1LT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - AD1LT"]
    #[inline(always)]
    #[must_use]
    pub fn ad1lt(&mut self) -> AD1LT_W<0> {
        AD1LT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1WLTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1wltr](index.html) module"]
pub struct ADC1WLTR_SPEC;
impl crate::RegisterSpec for ADC1WLTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1wltr::R](R) reader structure"]
impl crate::Readable for ADC1WLTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1wltr::W](W) writer structure"]
impl crate::Writable for ADC1WLTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1WLTR to value 0"]
impl crate::Resettable for ADC1WLTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
