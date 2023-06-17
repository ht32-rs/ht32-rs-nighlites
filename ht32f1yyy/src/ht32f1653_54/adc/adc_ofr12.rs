#[doc = "Register `ADC_OFR12` reader"]
pub struct R(crate::R<ADC_OFR12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR12` writer"]
pub struct W(crate::W<ADC_OFR12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR12_SPEC>;
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
impl From<crate::W<ADC_OFR12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF12` reader - ADOF12"]
pub type ADOF12_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF12` writer - ADOF12"]
pub type ADOF12_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR12_SPEC, 12, O, u16>;
#[doc = "Field `ADAL12` reader - ADAL12"]
pub type ADAL12_R = crate::BitReader;
#[doc = "Field `ADAL12` writer - ADAL12"]
pub type ADAL12_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR12_SPEC, O>;
#[doc = "Field `ADOFE12` reader - ADOFE12"]
pub type ADOFE12_R = crate::BitReader;
#[doc = "Field `ADOFE12` writer - ADOFE12"]
pub type ADOFE12_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR12_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF12"]
    #[inline(always)]
    pub fn adof12(&self) -> ADOF12_R {
        ADOF12_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL12"]
    #[inline(always)]
    pub fn adal12(&self) -> ADAL12_R {
        ADAL12_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE12"]
    #[inline(always)]
    pub fn adofe12(&self) -> ADOFE12_R {
        ADOFE12_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF12"]
    #[inline(always)]
    #[must_use]
    pub fn adof12(&mut self) -> ADOF12_W<0> {
        ADOF12_W::new(self)
    }
    #[doc = "Bit 14 - ADAL12"]
    #[inline(always)]
    #[must_use]
    pub fn adal12(&mut self) -> ADAL12_W<14> {
        ADAL12_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE12"]
    #[inline(always)]
    #[must_use]
    pub fn adofe12(&mut self) -> ADOFE12_W<15> {
        ADOFE12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR12\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr12](index.html) module"]
pub struct ADC_OFR12_SPEC;
impl crate::RegisterSpec for ADC_OFR12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr12::R](R) reader structure"]
impl crate::Readable for ADC_OFR12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr12::W](W) writer structure"]
impl crate::Writable for ADC_OFR12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR12 to value 0"]
impl crate::Resettable for ADC_OFR12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
