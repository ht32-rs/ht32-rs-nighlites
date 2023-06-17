#[doc = "Register `ADC_OFR4` reader"]
pub struct R(crate::R<ADC_OFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR4` writer"]
pub struct W(crate::W<ADC_OFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR4_SPEC>;
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
impl From<crate::W<ADC_OFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF4` reader - ADOF4"]
pub type ADOF4_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF4` writer - ADOF4"]
pub type ADOF4_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR4_SPEC, 12, O, u16>;
#[doc = "Field `ADAL4` reader - ADAL4"]
pub type ADAL4_R = crate::BitReader;
#[doc = "Field `ADAL4` writer - ADAL4"]
pub type ADAL4_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR4_SPEC, O>;
#[doc = "Field `ADOFE4` reader - ADOFE4"]
pub type ADOFE4_R = crate::BitReader;
#[doc = "Field `ADOFE4` writer - ADOFE4"]
pub type ADOFE4_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR4_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF4"]
    #[inline(always)]
    pub fn adof4(&self) -> ADOF4_R {
        ADOF4_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL4"]
    #[inline(always)]
    pub fn adal4(&self) -> ADAL4_R {
        ADAL4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE4"]
    #[inline(always)]
    pub fn adofe4(&self) -> ADOFE4_R {
        ADOFE4_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF4"]
    #[inline(always)]
    #[must_use]
    pub fn adof4(&mut self) -> ADOF4_W<0> {
        ADOF4_W::new(self)
    }
    #[doc = "Bit 14 - ADAL4"]
    #[inline(always)]
    #[must_use]
    pub fn adal4(&mut self) -> ADAL4_W<14> {
        ADAL4_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE4"]
    #[inline(always)]
    #[must_use]
    pub fn adofe4(&mut self) -> ADOFE4_W<15> {
        ADOFE4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr4](index.html) module"]
pub struct ADC_OFR4_SPEC;
impl crate::RegisterSpec for ADC_OFR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr4::R](R) reader structure"]
impl crate::Readable for ADC_OFR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr4::W](W) writer structure"]
impl crate::Writable for ADC_OFR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR4 to value 0"]
impl crate::Resettable for ADC_OFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
