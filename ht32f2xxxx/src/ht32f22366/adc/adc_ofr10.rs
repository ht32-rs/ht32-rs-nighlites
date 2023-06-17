#[doc = "Register `ADC_OFR10` reader"]
pub struct R(crate::R<ADC_OFR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR10` writer"]
pub struct W(crate::W<ADC_OFR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR10_SPEC>;
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
impl From<crate::W<ADC_OFR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF10` reader - ADOF10"]
pub type ADOF10_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF10` writer - ADOF10"]
pub type ADOF10_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR10_SPEC, 12, O, u16>;
#[doc = "Field `ADAL10` reader - ADAL10"]
pub type ADAL10_R = crate::BitReader;
#[doc = "Field `ADAL10` writer - ADAL10"]
pub type ADAL10_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR10_SPEC, O>;
#[doc = "Field `ADOFE10` reader - ADOFE10"]
pub type ADOFE10_R = crate::BitReader;
#[doc = "Field `ADOFE10` writer - ADOFE10"]
pub type ADOFE10_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR10_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF10"]
    #[inline(always)]
    pub fn adof10(&self) -> ADOF10_R {
        ADOF10_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL10"]
    #[inline(always)]
    pub fn adal10(&self) -> ADAL10_R {
        ADAL10_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE10"]
    #[inline(always)]
    pub fn adofe10(&self) -> ADOFE10_R {
        ADOFE10_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF10"]
    #[inline(always)]
    #[must_use]
    pub fn adof10(&mut self) -> ADOF10_W<0> {
        ADOF10_W::new(self)
    }
    #[doc = "Bit 14 - ADAL10"]
    #[inline(always)]
    #[must_use]
    pub fn adal10(&mut self) -> ADAL10_W<14> {
        ADAL10_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE10"]
    #[inline(always)]
    #[must_use]
    pub fn adofe10(&mut self) -> ADOFE10_W<15> {
        ADOFE10_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR10\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr10](index.html) module"]
pub struct ADC_OFR10_SPEC;
impl crate::RegisterSpec for ADC_OFR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr10::R](R) reader structure"]
impl crate::Readable for ADC_OFR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr10::W](W) writer structure"]
impl crate::Writable for ADC_OFR10_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR10 to value 0"]
impl crate::Resettable for ADC_OFR10_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
