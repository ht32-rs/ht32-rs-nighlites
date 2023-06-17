#[doc = "Register `ADC_OFR3` reader"]
pub struct R(crate::R<ADC_OFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_OFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_OFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_OFR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_OFR3` writer"]
pub struct W(crate::W<ADC_OFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_OFR3_SPEC>;
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
impl From<crate::W<ADC_OFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_OFR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADOF3` reader - ADOF3"]
pub type ADOF3_R = crate::FieldReader<u16>;
#[doc = "Field `ADOF3` writer - ADOF3"]
pub type ADOF3_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_OFR3_SPEC, 12, O, u16>;
#[doc = "Field `ADOFE3` reader - ADOFE3"]
pub type ADOFE3_R = crate::BitReader;
#[doc = "Field `ADOFE3` writer - ADOFE3"]
pub type ADOFE3_W<'a, const O: u8> = crate::BitWriter<'a, ADC_OFR3_SPEC, O>;
impl R {
    #[doc = "Bits 0:11 - ADOF3"]
    #[inline(always)]
    pub fn adof3(&self) -> ADOF3_R {
        ADOF3_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - ADOFE3"]
    #[inline(always)]
    pub fn adofe3(&self) -> ADOFE3_R {
        ADOFE3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ADOF3"]
    #[inline(always)]
    #[must_use]
    pub fn adof3(&mut self) -> ADOF3_W<0> {
        ADOF3_W::new(self)
    }
    #[doc = "Bit 15 - ADOFE3"]
    #[inline(always)]
    #[must_use]
    pub fn adofe3(&mut self) -> ADOFE3_W<15> {
        ADOFE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_OFR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ofr3](index.html) module"]
pub struct ADC_OFR3_SPEC;
impl crate::RegisterSpec for ADC_OFR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ofr3::R](R) reader structure"]
impl crate::Readable for ADC_OFR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ofr3::W](W) writer structure"]
impl crate::Writable for ADC_OFR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_OFR3 to value 0"]
impl crate::Resettable for ADC_OFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
