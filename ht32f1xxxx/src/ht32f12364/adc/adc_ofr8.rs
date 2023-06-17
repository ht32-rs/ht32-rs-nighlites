#[doc = "Register `ADC_OFR8` reader"]
pub struct R(crate::R<ADC_OFR8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR8` writer"]
pub struct W(crate::W<ADC_OFR8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR8_SPEC>;
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
impl From<crate::W<ADC_OFR8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF8` reader - ADOF8"]
pub type ADOF8_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF8` writer - ADOF8"]
pub type ADOF8_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR8_SPEC, 12, O, u16>;
#[doc = "Field `ADAL8` reader - ADAL8"]
pub type ADAL8_R = crate::BitReader;
#[doc = "Field `ADAL8` writer - ADAL8"]
pub type ADAL8_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR8_SPEC, O>;
#[doc = "Field `ADOFE8` reader - ADOFE8"]
pub type ADOFE8_R = crate::BitReader;
#[doc = "Field `ADOFE8` writer - ADOFE8"]
pub type ADOFE8_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR8_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF8"]
    #[inline(always)]
    pub fn adof8(&self) -> ADOF8_R {
        ADOF8_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL8"]
    #[inline(always)]
    pub fn adal8(&self) -> ADAL8_R {
        ADAL8_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE8"]
    #[inline(always)]
    pub fn adofe8(&self) -> ADOFE8_R {
        ADOFE8_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF8"]
    #[inline(always)]
    #[must_use]
    pub fn adof8(&mut self) -> ADOF8_W<0> {
        ADOF8_W::new(self)
    }
    #[doc = "Bit 14 - ADAL8"]
    #[inline(always)]
    #[must_use]
    pub fn adal8(&mut self) -> ADAL8_W<14> {
        ADAL8_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE8"]
    #[inline(always)]
    #[must_use]
    pub fn adofe8(&mut self) -> ADOFE8_W<15> {
        ADOFE8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr8](index.html) module"]
pub struct ADC_OFR8_SPEC;
impl crate::RegisterSpec for ADC_OFR8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr8::R](R) reader structure"]
impl crate::Readable for ADC_OFR8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr8::W](W) writer structure"]
impl crate::Writable for ADC_OFR8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR8 to value 0"]
impl crate::Resettable for ADC_OFR8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
