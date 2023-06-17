#[doc = "Register `ADC_OFR14` reader"]
pub struct R(crate::R<ADC_OFR14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR14` writer"]
pub struct W(crate::W<ADC_OFR14_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR14_SPEC>;
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
impl From<crate::W<ADC_OFR14_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR14_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF14` reader - ADOF14"]
pub type ADOF14_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF14` writer - ADOF14"]
pub type ADOF14_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR14_SPEC, 12, O, u16>;
#[doc = "Field `ADAL14` reader - ADAL14"]
pub type ADAL14_R = crate::BitReader;
#[doc = "Field `ADAL14` writer - ADAL14"]
pub type ADAL14_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR14_SPEC, O>;
#[doc = "Field `ADOFE14` reader - ADOFE14"]
pub type ADOFE14_R = crate::BitReader;
#[doc = "Field `ADOFE14` writer - ADOFE14"]
pub type ADOFE14_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR14_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF14"]
    #[inline(always)]
    pub fn adof14(&self) -> ADOF14_R {
        ADOF14_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL14"]
    #[inline(always)]
    pub fn adal14(&self) -> ADAL14_R {
        ADAL14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE14"]
    #[inline(always)]
    pub fn adofe14(&self) -> ADOFE14_R {
        ADOFE14_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF14"]
    #[inline(always)]
    #[must_use]
    pub fn adof14(&mut self) -> ADOF14_W<0> {
        ADOF14_W::new(self)
    }
    #[doc = "Bit 14 - ADAL14"]
    #[inline(always)]
    #[must_use]
    pub fn adal14(&mut self) -> ADAL14_W<14> {
        ADAL14_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE14"]
    #[inline(always)]
    #[must_use]
    pub fn adofe14(&mut self) -> ADOFE14_W<15> {
        ADOFE14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR14\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr14](index.html) module"]
pub struct ADC_OFR14_SPEC;
impl crate::RegisterSpec for ADC_OFR14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr14::R](R) reader structure"]
impl crate::Readable for ADC_OFR14_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr14::W](W) writer structure"]
impl crate::Writable for ADC_OFR14_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR14 to value 0"]
impl crate::Resettable for ADC_OFR14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
