#[doc = "Register `ADC_OFR13` reader"]
pub struct R(crate::R<ADC_OFR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR13` writer"]
pub struct W(crate::W<ADC_OFR13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR13_SPEC>;
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
impl From<crate::W<ADC_OFR13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF13` reader - ADOF13"]
pub type ADOF13_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF13` writer - ADOF13"]
pub type ADOF13_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR13_SPEC, 12, O, u16>;
#[doc = "Field `ADAL13` reader - ADAL13"]
pub type ADAL13_R = crate::BitReader;
#[doc = "Field `ADAL13` writer - ADAL13"]
pub type ADAL13_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR13_SPEC, O>;
#[doc = "Field `ADOFE13` reader - ADOFE13"]
pub type ADOFE13_R = crate::BitReader;
#[doc = "Field `ADOFE13` writer - ADOFE13"]
pub type ADOFE13_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR13_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF13"]
    #[inline(always)]
    pub fn adof13(&self) -> ADOF13_R {
        ADOF13_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL13"]
    #[inline(always)]
    pub fn adal13(&self) -> ADAL13_R {
        ADAL13_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE13"]
    #[inline(always)]
    pub fn adofe13(&self) -> ADOFE13_R {
        ADOFE13_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF13"]
    #[inline(always)]
    #[must_use]
    pub fn adof13(&mut self) -> ADOF13_W<0> {
        ADOF13_W::new(self)
    }
    #[doc = "Bit 14 - ADAL13"]
    #[inline(always)]
    #[must_use]
    pub fn adal13(&mut self) -> ADAL13_W<14> {
        ADAL13_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE13"]
    #[inline(always)]
    #[must_use]
    pub fn adofe13(&mut self) -> ADOFE13_W<15> {
        ADOFE13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr13](index.html) module"]
pub struct ADC_OFR13_SPEC;
impl crate::RegisterSpec for ADC_OFR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr13::R](R) reader structure"]
impl crate::Readable for ADC_OFR13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr13::W](W) writer structure"]
impl crate::Writable for ADC_OFR13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR13 to value 0"]
impl crate::Resettable for ADC_OFR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
