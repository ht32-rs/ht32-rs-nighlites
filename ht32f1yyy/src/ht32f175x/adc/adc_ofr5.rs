#[doc = "Register `ADC_OFR5` reader"]
pub struct R(crate::R<ADC_OFR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR5` writer"]
pub struct W(crate::W<ADC_OFR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR5_SPEC>;
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
impl From<crate::W<ADC_OFR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF5` reader - ADOF5"]
pub type ADOF5_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF5` writer - ADOF5"]
pub type ADOF5_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR5_SPEC, 12, O, u16>;
#[doc = "Field `ADOFE5` reader - ADOFE5"]
pub type ADOFE5_R = crate::BitReader;
#[doc = "Field `ADOFE5` writer - ADOFE5"]
pub type ADOFE5_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR5_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF5"]
    #[inline(always)]
    pub fn adof5(&self) -> ADOF5_R {
        ADOF5_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - ADOFE5"]
    #[inline(always)]
    pub fn adofe5(&self) -> ADOFE5_R {
        ADOFE5_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF5"]
    #[inline(always)]
    #[must_use]
    pub fn adof5(&mut self) -> ADOF5_W<0> {
        ADOF5_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE5"]
    #[inline(always)]
    #[must_use]
    pub fn adofe5(&mut self) -> ADOFE5_W<15> {
        ADOFE5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr5](index.html) module"]
pub struct ADC_OFR5_SPEC;
impl crate::RegisterSpec for ADC_OFR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr5::R](R) reader structure"]
impl crate::Readable for ADC_OFR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr5::W](W) writer structure"]
impl crate::Writable for ADC_OFR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR5 to value 0"]
impl crate::Resettable for ADC_OFR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
