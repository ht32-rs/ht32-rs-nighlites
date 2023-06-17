#[doc = "Register `ADC_OFR0` reader"]
pub struct R(crate::R<ADC_OFR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR0` writer"]
pub struct W(crate::W<ADC_OFR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR0_SPEC>;
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
impl From<crate::W<ADC_OFR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF0` reader - ADOF0"]
pub type ADOF0_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF0` writer - ADOF0"]
pub type ADOF0_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR0_SPEC, 12, O, u16>;
#[doc = "Field `ADOFE0` reader - ADOFE0"]
pub type ADOFE0_R = crate::BitReader;
#[doc = "Field `ADOFE0` writer - ADOFE0"]
pub type ADOFE0_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR0_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF0"]
    #[inline(always)]
    pub fn adof0(&self) -> ADOF0_R {
        ADOF0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - ADOFE0"]
    #[inline(always)]
    pub fn adofe0(&self) -> ADOFE0_R {
        ADOFE0_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF0"]
    #[inline(always)]
    #[must_use]
    pub fn adof0(&mut self) -> ADOF0_W<0> {
        ADOF0_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE0"]
    #[inline(always)]
    #[must_use]
    pub fn adofe0(&mut self) -> ADOFE0_W<15> {
        ADOFE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr0](index.html) module"]
pub struct ADC_OFR0_SPEC;
impl crate::RegisterSpec for ADC_OFR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr0::R](R) reader structure"]
impl crate::Readable for ADC_OFR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr0::W](W) writer structure"]
impl crate::Writable for ADC_OFR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR0 to value 0"]
impl crate::Resettable for ADC_OFR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
