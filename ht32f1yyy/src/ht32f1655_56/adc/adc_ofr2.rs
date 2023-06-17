#[doc = "Register `ADC_OFR2` reader"]
pub struct R(crate::R<ADC_OFR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR2` writer"]
pub struct W(crate::W<ADC_OFR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR2_SPEC>;
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
impl From<crate::W<ADC_OFR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF2` reader - ADOF2"]
pub type ADOF2_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF2` writer - ADOF2"]
pub type ADOF2_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR2_SPEC, 12, O, u16>;
#[doc = "Field `ADAL2` reader - ADAL2"]
pub type ADAL2_R = crate::BitReader;
#[doc = "Field `ADAL2` writer - ADAL2"]
pub type ADAL2_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR2_SPEC, O>;
#[doc = "Field `ADOFE2` reader - ADOFE2"]
pub type ADOFE2_R = crate::BitReader;
#[doc = "Field `ADOFE2` writer - ADOFE2"]
pub type ADOFE2_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR2_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF2"]
    #[inline(always)]
    pub fn adof2(&self) -> ADOF2_R {
        ADOF2_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL2"]
    #[inline(always)]
    pub fn adal2(&self) -> ADAL2_R {
        ADAL2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE2"]
    #[inline(always)]
    pub fn adofe2(&self) -> ADOFE2_R {
        ADOFE2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF2"]
    #[inline(always)]
    #[must_use]
    pub fn adof2(&mut self) -> ADOF2_W<0> {
        ADOF2_W::new(self)
    }
    #[doc = "Bit 14 - ADAL2"]
    #[inline(always)]
    #[must_use]
    pub fn adal2(&mut self) -> ADAL2_W<14> {
        ADAL2_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE2"]
    #[inline(always)]
    #[must_use]
    pub fn adofe2(&mut self) -> ADOFE2_W<15> {
        ADOFE2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr2](index.html) module"]
pub struct ADC_OFR2_SPEC;
impl crate::RegisterSpec for ADC_OFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr2::R](R) reader structure"]
impl crate::Readable for ADC_OFR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr2::W](W) writer structure"]
impl crate::Writable for ADC_OFR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR2 to value 0"]
impl crate::Resettable for ADC_OFR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
