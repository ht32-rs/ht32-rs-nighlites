#[doc = "Register `ADC_VREFVALR` reader"]
pub struct R(crate::R<ADC_VREFVALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_VREFVALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_VREFVALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_VREFVALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_VREFVALR` writer"]
pub struct W(crate::W<ADC_VREFVALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_VREFVALR_SPEC>;
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
impl From<crate::W<ADC_VREFVALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_VREFVALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VREFVAL` reader - VREFEN"]
pub type VREFVAL_R = crate::FieldReader;
#[doc = "Field `VREFVAL` writer - VREFEN"]
pub type VREFVAL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_VREFVALR_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - VREFEN"]
    #[inline(always)]
    pub fn vrefval(&self) -> VREFVAL_R {
        VREFVAL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - VREFEN"]
    #[inline(always)]
    #[must_use]
    pub fn vrefval(&mut self) -> VREFVAL_W<0> {
        VREFVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_VREFVALR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_vrefvalr](index.html) module"]
pub struct ADC_VREFVALR_SPEC;
impl crate::RegisterSpec for ADC_VREFVALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_vrefvalr::R](R) reader structure"]
impl crate::Readable for ADC_VREFVALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_vrefvalr::W](W) writer structure"]
impl crate::Writable for ADC_VREFVALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_VREFVALR to value 0"]
impl crate::Resettable for ADC_VREFVALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
