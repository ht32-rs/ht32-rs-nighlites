#[doc = "Register `ADC_IMR` reader"]
pub struct R(crate::R<ADC_IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_IMR` writer"]
pub struct W(crate::W<ADC_IMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_IMR_SPEC>;
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
impl From<crate::W<ADC_IMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_IMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADIMS` reader - ADIMS"]
pub type ADIMS_R = crate::BitReader;
#[doc = "Field `ADIMS` writer - ADIMS"]
pub type ADIMS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMR_SPEC, O>;
#[doc = "Field `ADIMG` reader - ADIMG"]
pub type ADIMG_R = crate::BitReader;
#[doc = "Field `ADIMG` writer - ADIMG"]
pub type ADIMG_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMR_SPEC, O>;
#[doc = "Field `ADIMC` reader - ADIMC"]
pub type ADIMC_R = crate::BitReader;
#[doc = "Field `ADIMC` writer - ADIMC"]
pub type ADIMC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMR_SPEC, O>;
#[doc = "Field `ADIML` reader - ADIML"]
pub type ADIML_R = crate::BitReader;
#[doc = "Field `ADIML` writer - ADIML"]
pub type ADIML_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMR_SPEC, O>;
#[doc = "Field `ADIMU` reader - ADIMU"]
pub type ADIMU_R = crate::BitReader;
#[doc = "Field `ADIMU` writer - ADIMU"]
pub type ADIMU_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMR_SPEC, O>;
#[doc = "Field `ADIMO` reader - ADIMO"]
pub type ADIMO_R = crate::BitReader;
#[doc = "Field `ADIMO` writer - ADIMO"]
pub type ADIMO_W<'a, const O: u8> = crate::BitWriter<'a, ADC_IMR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADIMS"]
    #[inline(always)]
    pub fn adims(&self) -> ADIMS_R {
        ADIMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADIMG"]
    #[inline(always)]
    pub fn adimg(&self) -> ADIMG_R {
        ADIMG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADIMC"]
    #[inline(always)]
    pub fn adimc(&self) -> ADIMC_R {
        ADIMC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ADIML"]
    #[inline(always)]
    pub fn adiml(&self) -> ADIML_R {
        ADIML_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADIMU"]
    #[inline(always)]
    pub fn adimu(&self) -> ADIMU_R {
        ADIMU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - ADIMO"]
    #[inline(always)]
    pub fn adimo(&self) -> ADIMO_R {
        ADIMO_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADIMS"]
    #[inline(always)]
    #[must_use]
    pub fn adims(&mut self) -> ADIMS_W<0> {
        ADIMS_W::new(self)
    }
    #[doc = "Bit 1 - ADIMG"]
    #[inline(always)]
    #[must_use]
    pub fn adimg(&mut self) -> ADIMG_W<1> {
        ADIMG_W::new(self)
    }
    #[doc = "Bit 2 - ADIMC"]
    #[inline(always)]
    #[must_use]
    pub fn adimc(&mut self) -> ADIMC_W<2> {
        ADIMC_W::new(self)
    }
    #[doc = "Bit 16 - ADIML"]
    #[inline(always)]
    #[must_use]
    pub fn adiml(&mut self) -> ADIML_W<16> {
        ADIML_W::new(self)
    }
    #[doc = "Bit 17 - ADIMU"]
    #[inline(always)]
    #[must_use]
    pub fn adimu(&mut self) -> ADIMU_W<17> {
        ADIMU_W::new(self)
    }
    #[doc = "Bit 24 - ADIMO"]
    #[inline(always)]
    #[must_use]
    pub fn adimo(&mut self) -> ADIMO_W<24> {
        ADIMO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_IMR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_imr](index.html) module"]
pub struct ADC_IMR_SPEC;
impl crate::RegisterSpec for ADC_IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_imr::R](R) reader structure"]
impl crate::Readable for ADC_IMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_imr::W](W) writer structure"]
impl crate::Writable for ADC_IMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_IMR to value 0"]
impl crate::Resettable for ADC_IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
