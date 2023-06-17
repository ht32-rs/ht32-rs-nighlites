#[doc = "Register `ADC_OFR15` reader"]
pub struct R(crate::R<ADC_OFR15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR15` writer"]
pub struct W(crate::W<ADC_OFR15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR15_SPEC>;
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
impl From<crate::W<ADC_OFR15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF15` reader - ADOF15"]
pub type ADOF15_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF15` writer - ADOF15"]
pub type ADOF15_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR15_SPEC, 12, O, u16>;
#[doc = "Field `ADAL15` reader - ADAL15"]
pub type ADAL15_R = crate::BitReader;
#[doc = "Field `ADAL15` writer - ADAL15"]
pub type ADAL15_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR15_SPEC, O>;
#[doc = "Field `ADOFE15` reader - ADOFE15"]
pub type ADOFE15_R = crate::BitReader;
#[doc = "Field `ADOFE15` writer - ADOFE15"]
pub type ADOFE15_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR15_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF15"]
    #[inline(always)]
    pub fn adof15(&self) -> ADOF15_R {
        ADOF15_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL15"]
    #[inline(always)]
    pub fn adal15(&self) -> ADAL15_R {
        ADAL15_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE15"]
    #[inline(always)]
    pub fn adofe15(&self) -> ADOFE15_R {
        ADOFE15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF15"]
    #[inline(always)]
    #[must_use]
    pub fn adof15(&mut self) -> ADOF15_W<0> {
        ADOF15_W::new(self)
    }
    #[doc = "Bit 14 - ADAL15"]
    #[inline(always)]
    #[must_use]
    pub fn adal15(&mut self) -> ADAL15_W<14> {
        ADAL15_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE15"]
    #[inline(always)]
    #[must_use]
    pub fn adofe15(&mut self) -> ADOFE15_W<15> {
        ADOFE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR15\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr15](index.html) module"]
pub struct ADC_OFR15_SPEC;
impl crate::RegisterSpec for ADC_OFR15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr15::R](R) reader structure"]
impl crate::Readable for ADC_OFR15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr15::W](W) writer structure"]
impl crate::Writable for ADC_OFR15_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR15 to value 0"]
impl crate::Resettable for ADC_OFR15_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
