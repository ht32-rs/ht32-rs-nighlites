#[doc = "Register `ADC_OFR1` reader"]
pub struct R(crate::R<ADC_OFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR1` writer"]
pub struct W(crate::W<ADC_OFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR1_SPEC>;
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
impl From<crate::W<ADC_OFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF1` reader - ADOF1"]
pub type ADOF1_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF1` writer - ADOF1"]
pub type ADOF1_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR1_SPEC, 12, O, u16>;
#[doc = "Field `ADOFE1` reader - ADOFE1"]
pub type ADOFE1_R = crate::BitReader;
#[doc = "Field `ADOFE1` writer - ADOFE1"]
pub type ADOFE1_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR1_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF1"]
    #[inline(always)]
    pub fn adof1(&self) -> ADOF1_R {
        ADOF1_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - ADOFE1"]
    #[inline(always)]
    pub fn adofe1(&self) -> ADOFE1_R {
        ADOFE1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF1"]
    #[inline(always)]
    #[must_use]
    pub fn adof1(&mut self) -> ADOF1_W<0> {
        ADOF1_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE1"]
    #[inline(always)]
    #[must_use]
    pub fn adofe1(&mut self) -> ADOFE1_W<15> {
        ADOFE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr1](index.html) module"]
pub struct ADC_OFR1_SPEC;
impl crate::RegisterSpec for ADC_OFR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr1::R](R) reader structure"]
impl crate::Readable for ADC_OFR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr1::W](W) writer structure"]
impl crate::Writable for ADC_OFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR1 to value 0"]
impl crate::Resettable for ADC_OFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
