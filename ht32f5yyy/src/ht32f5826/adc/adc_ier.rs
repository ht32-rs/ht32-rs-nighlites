#[doc = "Register `ADC_IER` reader"]
pub struct R(crate::R<ADC_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_IER` writer"]
pub struct W(crate::W<ADC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_IER_SPEC>;
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
impl From<crate::W<ADC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADIES` reader - ADIES"]
pub type ADIES_R = crate::BitReader;
#[doc = "Field `ADIES` writer - ADIES"]
pub type ADIES_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IER_SPEC, O>;
#[doc = "Field `ADIEG` reader - ADIEG"]
pub type ADIEG_R = crate::BitReader;
#[doc = "Field `ADIEG` writer - ADIEG"]
pub type ADIEG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IER_SPEC, O>;
#[doc = "Field `ADIEC` reader - ADIEC"]
pub type ADIEC_R = crate::BitReader;
#[doc = "Field `ADIEC` writer - ADIEC"]
pub type ADIEC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IER_SPEC, O>;
#[doc = "Field `ADIEL` reader - ADIEL"]
pub type ADIEL_R = crate::BitReader;
#[doc = "Field `ADIEL` writer - ADIEL"]
pub type ADIEL_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IER_SPEC, O>;
#[doc = "Field `ADIEU` reader - ADIEU"]
pub type ADIEU_R = crate::BitReader;
#[doc = "Field `ADIEU` writer - ADIEU"]
pub type ADIEU_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IER_SPEC, O>;
#[doc = "Field `ADIEO` reader - ADIEO"]
pub type ADIEO_R = crate::BitReader;
#[doc = "Field `ADIEO` writer - ADIEO"]
pub type ADIEO_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADIES"]
    #[inline(always)]
    pub fn adies(&self) -> ADIES_R {
        ADIES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADIEG"]
    #[inline(always)]
    pub fn adieg(&self) -> ADIEG_R {
        ADIEG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADIEC"]
    #[inline(always)]
    pub fn adiec(&self) -> ADIEC_R {
        ADIEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ADIEL"]
    #[inline(always)]
    pub fn adiel(&self) -> ADIEL_R {
        ADIEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADIEU"]
    #[inline(always)]
    pub fn adieu(&self) -> ADIEU_R {
        ADIEU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADIEO"]
    #[inline(always)]
    pub fn adieo(&self) -> ADIEO_R {
        ADIEO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIES"]
    #[inline(always)]
    #[must_use]
    pub fn adies(&mut self) -> ADIES_W<0> {
        ADIES_W::new(self)
    }
    #[doc = "Bit 1 - ADIEG"]
    #[inline(always)]
    #[must_use]
    pub fn adieg(&mut self) -> ADIEG_W<1> {
        ADIEG_W::new(self)
    }
    #[doc = "Bit 2 - ADIEC"]
    #[inline(always)]
    #[must_use]
    pub fn adiec(&mut self) -> ADIEC_W<2> {
        ADIEC_W::new(self)
    }
    #[doc = "Bit 16 - ADIEL"]
    #[inline(always)]
    #[must_use]
    pub fn adiel(&mut self) -> ADIEL_W<16> {
        ADIEL_W::new(self)
    }
    #[doc = "Bit 17 - ADIEU"]
    #[inline(always)]
    #[must_use]
    pub fn adieu(&mut self) -> ADIEU_W<17> {
        ADIEU_W::new(self)
    }
    #[doc = "Bit 24 - ADIEO"]
    #[inline(always)]
    #[must_use]
    pub fn adieo(&mut self) -> ADIEO_W<24> {
        ADIEO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_IMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_ier](index.html) module"]
pub struct ADC_IER_SPEC;
impl crate::RegisterSpec for ADC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_ier::R](R) reader structure"]
impl crate::Readable for ADC_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_ier::W](W) writer structure"]
impl crate::Writable for ADC_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_IER to value 0"]
impl crate::Resettable for ADC_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
