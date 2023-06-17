#[doc = "Register `ADC_OFR6` reader"]
pub struct R(crate::R<ADC_OFR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR6` writer"]
pub struct W(crate::W<ADC_OFR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR6_SPEC>;
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
impl From<crate::W<ADC_OFR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF6` reader - ADOF6"]
pub type ADOF6_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF6` writer - ADOF6"]
pub type ADOF6_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR6_SPEC, 12, O, u16>;
#[doc = "Field `ADOFE6` reader - ADOFE6"]
pub type ADOFE6_R = crate::BitReader;
#[doc = "Field `ADOFE6` writer - ADOFE6"]
pub type ADOFE6_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR6_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF6"]
    #[inline(always)]
    pub fn adof6(&self) -> ADOF6_R {
        ADOF6_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - ADOFE6"]
    #[inline(always)]
    pub fn adofe6(&self) -> ADOFE6_R {
        ADOFE6_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF6"]
    #[inline(always)]
    #[must_use]
    pub fn adof6(&mut self) -> ADOF6_W<0> {
        ADOF6_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE6"]
    #[inline(always)]
    #[must_use]
    pub fn adofe6(&mut self) -> ADOFE6_W<15> {
        ADOFE6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr6](index.html) module"]
pub struct ADC_OFR6_SPEC;
impl crate::RegisterSpec for ADC_OFR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr6::R](R) reader structure"]
impl crate::Readable for ADC_OFR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr6::W](W) writer structure"]
impl crate::Writable for ADC_OFR6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR6 to value 0"]
impl crate::Resettable for ADC_OFR6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
