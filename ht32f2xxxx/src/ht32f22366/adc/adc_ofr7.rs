#[doc = "Register `ADC_OFR7` reader"]
pub struct R(crate::R<ADC_OFR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR7` writer"]
pub struct W(crate::W<ADC_OFR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR7_SPEC>;
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
impl From<crate::W<ADC_OFR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF7` reader - ADOF7"]
pub type ADOF7_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF7` writer - ADOF7"]
pub type ADOF7_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR7_SPEC, 12, O, u16>;
#[doc = "Field `ADAL7` reader - ADAL7"]
pub type ADAL7_R = crate::BitReader;
#[doc = "Field `ADAL7` writer - ADAL7"]
pub type ADAL7_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR7_SPEC, O>;
#[doc = "Field `ADOFE7` reader - ADOFE7"]
pub type ADOFE7_R = crate::BitReader;
#[doc = "Field `ADOFE7` writer - ADOFE7"]
pub type ADOFE7_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR7_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF7"]
    #[inline(always)]
    pub fn adof7(&self) -> ADOF7_R {
        ADOF7_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL7"]
    #[inline(always)]
    pub fn adal7(&self) -> ADAL7_R {
        ADAL7_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE7"]
    #[inline(always)]
    pub fn adofe7(&self) -> ADOFE7_R {
        ADOFE7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF7"]
    #[inline(always)]
    #[must_use]
    pub fn adof7(&mut self) -> ADOF7_W<0> {
        ADOF7_W::new(self)
    }
    #[doc = "Bit 14 - ADAL7"]
    #[inline(always)]
    #[must_use]
    pub fn adal7(&mut self) -> ADAL7_W<14> {
        ADAL7_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE7"]
    #[inline(always)]
    #[must_use]
    pub fn adofe7(&mut self) -> ADOFE7_W<15> {
        ADOFE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr7](index.html) module"]
pub struct ADC_OFR7_SPEC;
impl crate::RegisterSpec for ADC_OFR7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr7::R](R) reader structure"]
impl crate::Readable for ADC_OFR7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr7::W](W) writer structure"]
impl crate::Writable for ADC_OFR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR7 to value 0"]
impl crate::Resettable for ADC_OFR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
