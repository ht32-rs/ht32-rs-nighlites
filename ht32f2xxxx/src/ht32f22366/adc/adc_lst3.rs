#[doc = "Register `ADC_LST3` reader"]
pub struct R(crate::R<ADC_LST3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_LST3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_LST3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_LST3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_LST3` writer"]
pub struct W(crate::W<ADC_LST3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_LST3_SPEC>;
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
impl From<crate::W<ADC_LST3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_LST3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSEQ12` reader - ADSEQ12"]
pub type ADSEQ12_R = crate::FieldReader;
#[doc = "Field `ADSEQ12` writer - ADSEQ12"]
pub type ADSEQ12_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST3_SPEC, 5, O>;
#[doc = "Field `ADSEQ13` reader - ADSEQ13"]
pub type ADSEQ13_R = crate::FieldReader;
#[doc = "Field `ADSEQ13` writer - ADSEQ13"]
pub type ADSEQ13_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST3_SPEC, 5, O>;
#[doc = "Field `ADSEQ14` reader - ADSEQ14"]
pub type ADSEQ14_R = crate::FieldReader;
#[doc = "Field `ADSEQ14` writer - ADSEQ14"]
pub type ADSEQ14_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST3_SPEC, 5, O>;
#[doc = "Field `ADSEQ15` reader - ADSEQ15"]
pub type ADSEQ15_R = crate::FieldReader;
#[doc = "Field `ADSEQ15` writer - ADSEQ15"]
pub type ADSEQ15_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST3_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ADSEQ12"]
    #[inline(always)]
    pub fn adseq12(&self) -> ADSEQ12_R {
        ADSEQ12_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ13"]
    #[inline(always)]
    pub fn adseq13(&self) -> ADSEQ13_R {
        ADSEQ13_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ14"]
    #[inline(always)]
    pub fn adseq14(&self) -> ADSEQ14_R {
        ADSEQ14_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ15"]
    #[inline(always)]
    pub fn adseq15(&self) -> ADSEQ15_R {
        ADSEQ15_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ12"]
    #[inline(always)]
    #[must_use]
    pub fn adseq12(&mut self) -> ADSEQ12_W<0> {
        ADSEQ12_W::new(self)
    }
    #[doc = "Bits 8:12 - ADSEQ13"]
    #[inline(always)]
    #[must_use]
    pub fn adseq13(&mut self) -> ADSEQ13_W<8> {
        ADSEQ13_W::new(self)
    }
    #[doc = "Bits 16:20 - ADSEQ14"]
    #[inline(always)]
    #[must_use]
    pub fn adseq14(&mut self) -> ADSEQ14_W<16> {
        ADSEQ14_W::new(self)
    }
    #[doc = "Bits 24:28 - ADSEQ15"]
    #[inline(always)]
    #[must_use]
    pub fn adseq15(&mut self) -> ADSEQ15_W<24> {
        ADSEQ15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_LST3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_lst3](index.html) module"]
pub struct ADC_LST3_SPEC;
impl crate::RegisterSpec for ADC_LST3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_lst3::R](R) reader structure"]
impl crate::Readable for ADC_LST3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_lst3::W](W) writer structure"]
impl crate::Writable for ADC_LST3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_LST3 to value 0"]
impl crate::Resettable for ADC_LST3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
