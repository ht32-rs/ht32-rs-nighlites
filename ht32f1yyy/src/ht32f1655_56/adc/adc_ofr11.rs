#[doc = "Register `ADC_OFR11` reader"]
pub struct R(crate::R<ADC_OFR11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR11` writer"]
pub struct W(crate::W<ADC_OFR11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR11_SPEC>;
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
impl From<crate::W<ADC_OFR11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF11` reader - ADOF11"]
pub type ADOF11_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF11` writer - ADOF11"]
pub type ADOF11_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR11_SPEC, 12, O, u16>;
#[doc = "Field `ADAL11` reader - ADAL11"]
pub type ADAL11_R = crate::BitReader;
#[doc = "Field `ADAL11` writer - ADAL11"]
pub type ADAL11_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR11_SPEC, O>;
#[doc = "Field `ADOFE11` reader - ADOFE11"]
pub type ADOFE11_R = crate::BitReader;
#[doc = "Field `ADOFE11` writer - ADOFE11"]
pub type ADOFE11_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR11_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF11"]
    #[inline(always)]
    pub fn adof11(&self) -> ADOF11_R {
        ADOF11_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 14 - ADAL11"]
    #[inline(always)]
    pub fn adal11(&self) -> ADAL11_R {
        ADAL11_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADOFE11"]
    #[inline(always)]
    pub fn adofe11(&self) -> ADOFE11_R {
        ADOFE11_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF11"]
    #[inline(always)]
    #[must_use]
    pub fn adof11(&mut self) -> ADOF11_W<0> {
        ADOF11_W::new(self)
    }
    #[doc = "Bit 14 - ADAL11"]
    #[inline(always)]
    #[must_use]
    pub fn adal11(&mut self) -> ADAL11_W<14> {
        ADAL11_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE11"]
    #[inline(always)]
    #[must_use]
    pub fn adofe11(&mut self) -> ADOFE11_W<15> {
        ADOFE11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR11\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr11](index.html) module"]
pub struct ADC_OFR11_SPEC;
impl crate::RegisterSpec for ADC_OFR11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr11::R](R) reader structure"]
impl crate::Readable for ADC_OFR11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr11::W](W) writer structure"]
impl crate::Writable for ADC_OFR11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR11 to value 0"]
impl crate::Resettable for ADC_OFR11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
