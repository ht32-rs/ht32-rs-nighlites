#[doc = "Register `ADC_OFR9` reader"]
pub struct R(crate::R<ADC_OFR9_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR9_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR9_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR9_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR9` writer"]
pub struct W(crate::W<ADC_OFR9_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR9_SPEC>;
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
impl From<crate::W<ADC_OFR9_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR9_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF9` reader - ADOF9"]
pub type ADOF9_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF9` writer - ADOF9"]
pub type ADOF9_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR9_SPEC, 12, O, u16>;
#[doc = "Field `ADAL9` reader - ADAL9"]
pub type ADAL9_R = crate::BitReader;
#[doc = "Field `ADAL9` writer - ADAL9"]
pub type ADAL9_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR9_SPEC, O>;
#[doc = "Field `ADOFE9` reader - ADOFE9"]
pub type ADOFE9_R = crate::BitReader;
#[doc = "Field `ADOFE9` writer - ADOFE9"]
pub type ADOFE9_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR9_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF9"]
    #[inline(always)]
    pub fn adof9(&self) -> ADOF9_R {
        ADOF9_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL9"]
    #[inline(always)]
    pub fn adal9(&self) -> ADAL9_R {
        ADAL9_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE9"]
    #[inline(always)]
    pub fn adofe9(&self) -> ADOFE9_R {
        ADOFE9_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF9"]
    #[inline(always)]
    #[must_use]
    pub fn adof9(&mut self) -> ADOF9_W<0> {
        ADOF9_W::new(self)
    }
    #[doc = "Bit 14 - ADAL9"]
    #[inline(always)]
    #[must_use]
    pub fn adal9(&mut self) -> ADAL9_W<14> {
        ADAL9_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE9"]
    #[inline(always)]
    #[must_use]
    pub fn adofe9(&mut self) -> ADOFE9_W<15> {
        ADOFE9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR9\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr9](index.html) module"]
pub struct ADC_OFR9_SPEC;
impl crate::RegisterSpec for ADC_OFR9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr9::R](R) reader structure"]
impl crate::Readable for ADC_OFR9_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr9::W](W) writer structure"]
impl crate::Writable for ADC_OFR9_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR9 to value 0"]
impl crate::Resettable for ADC_OFR9_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
