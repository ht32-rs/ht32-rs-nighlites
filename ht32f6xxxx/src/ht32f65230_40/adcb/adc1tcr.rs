#[doc = "Register `ADC1TCR` reader"]
pub struct R(crate::R<ADC1TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1TCR` writer"]
pub struct W(crate::W<ADC1TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1TCR_SPEC>;
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
impl From<crate::W<ADC1TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1SW` reader - AD1SW"]
pub type AD1SW_R = crate::BitReader;
#[doc = "Field `AD1SW` writer - AD1SW"]
pub type AD1SW_W<'a, const O: u8> = crate::BitWriter<'a, ADC1TCR_SPEC, O>;
#[doc = "Field `AD1EXTI` reader - AD1EXTI"]
pub type AD1EXTI_R = crate::BitReader;
#[doc = "Field `AD1EXTI` writer - AD1EXTI"]
pub type AD1EXTI_W<'a, const O: u8> = crate::BitWriter<'a, ADC1TCR_SPEC, O>;
#[doc = "Field `AD1TM` reader - AD1TM"]
pub type AD1TM_R = crate::BitReader;
#[doc = "Field `AD1TM` writer - AD1TM"]
pub type AD1TM_W<'a, const O: u8> = crate::BitWriter<'a, ADC1TCR_SPEC, O>;
#[doc = "Field `AD1BFTM` reader - AD1BFTM"]
pub type AD1BFTM_R = crate::BitReader;
#[doc = "Field `AD1BFTM` writer - AD1BFTM"]
pub type AD1BFTM_W<'a, const O: u8> = crate::BitWriter<'a, ADC1TCR_SPEC, O>;
#[doc = "Field `AD1CMP` reader - AD1CMP"]
pub type AD1CMP_R = crate::BitReader;
#[doc = "Field `AD1CMP` writer - AD1CMP"]
pub type AD1CMP_W<'a, const O: u8> = crate::BitWriter<'a, ADC1TCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD1SW"]
    #[inline(always)]
    pub fn ad1sw(&self) -> AD1SW_R {
        AD1SW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1EXTI"]
    #[inline(always)]
    pub fn ad1exti(&self) -> AD1EXTI_R {
        AD1EXTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD1TM"]
    #[inline(always)]
    pub fn ad1tm(&self) -> AD1TM_R {
        AD1TM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD1BFTM"]
    #[inline(always)]
    pub fn ad1bftm(&self) -> AD1BFTM_R {
        AD1BFTM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD1CMP"]
    #[inline(always)]
    pub fn ad1cmp(&self) -> AD1CMP_R {
        AD1CMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD1SW"]
    #[inline(always)]
    #[must_use]
    pub fn ad1sw(&mut self) -> AD1SW_W<0> {
        AD1SW_W::new(self)
    }
    #[doc = "Bit 1 - AD1EXTI"]
    #[inline(always)]
    #[must_use]
    pub fn ad1exti(&mut self) -> AD1EXTI_W<1> {
        AD1EXTI_W::new(self)
    }
    #[doc = "Bit 2 - AD1TM"]
    #[inline(always)]
    #[must_use]
    pub fn ad1tm(&mut self) -> AD1TM_W<2> {
        AD1TM_W::new(self)
    }
    #[doc = "Bit 3 - AD1BFTM"]
    #[inline(always)]
    #[must_use]
    pub fn ad1bftm(&mut self) -> AD1BFTM_W<3> {
        AD1BFTM_W::new(self)
    }
    #[doc = "Bit 4 - AD1CMP"]
    #[inline(always)]
    #[must_use]
    pub fn ad1cmp(&mut self) -> AD1CMP_W<4> {
        AD1CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1tcr](index.html) module"]
pub struct ADC1TCR_SPEC;
impl crate::RegisterSpec for ADC1TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1tcr::R](R) reader structure"]
impl crate::Readable for ADC1TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1tcr::W](W) writer structure"]
impl crate::Writable for ADC1TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1TCR to value 0"]
impl crate::Resettable for ADC1TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
