#[doc = "Register `ADC_STR12` reader"]
pub struct R(crate::R<ADC_STR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_STR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_STR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_STR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_STR12` writer"]
pub struct W(crate::W<ADC_STR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_STR12_SPEC>;
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
impl From<crate::W<ADC_STR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_STR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADST12` reader - ADST12"]
pub type ADST12_R = crate::FieldReader;
#[doc = "Field `ADST12` writer - ADST12"]
pub type ADST12_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_STR12_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADST12"]
    #[inline(always)]
    pub fn adst12(&self) -> ADST12_R {
        ADST12_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST12"]
    #[inline(always)]
    #[must_use]
    pub fn adst12(&mut self) -> ADST12_W<0> {
        ADST12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_STR12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_str12](index.html) module"]
pub struct ADC_STR12_SPEC;
impl crate::RegisterSpec for ADC_STR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_str12::R](R) reader structure"]
impl crate::Readable for ADC_STR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_str12::W](W) writer structure"]
impl crate::Writable for ADC_STR12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_STR12 to value 0"]
impl crate::Resettable for ADC_STR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
