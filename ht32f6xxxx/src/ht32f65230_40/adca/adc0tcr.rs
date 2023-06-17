#[doc = "Register `ADC0TCR` reader"]
pub struct R(crate::R<ADC0TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0TCR` writer"]
pub struct W(crate::W<ADC0TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0TCR_SPEC>;
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
impl From<crate::W<ADC0TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0SW` reader - AD0SW"]
pub type AD0SW_R = crate::BitReader;
#[doc = "Field `AD0SW` writer - AD0SW"]
pub type AD0SW_W<'a, const O: u8> = crate::BitWriter<'a, ADC0TCR_SPEC, O>;
#[doc = "Field `AD0EXTI` reader - AD0EXTI"]
pub type AD0EXTI_R = crate::BitReader;
#[doc = "Field `AD0EXTI` writer - AD0EXTI"]
pub type AD0EXTI_W<'a, const O: u8> = crate::BitWriter<'a, ADC0TCR_SPEC, O>;
#[doc = "Field `AD0TM` reader - AD0TM"]
pub type AD0TM_R = crate::BitReader;
#[doc = "Field `AD0TM` writer - AD0TM"]
pub type AD0TM_W<'a, const O: u8> = crate::BitWriter<'a, ADC0TCR_SPEC, O>;
#[doc = "Field `AD0BFTM` reader - AD0BFTM"]
pub type AD0BFTM_R = crate::BitReader;
#[doc = "Field `AD0BFTM` writer - AD0BFTM"]
pub type AD0BFTM_W<'a, const O: u8> = crate::BitWriter<'a, ADC0TCR_SPEC, O>;
#[doc = "Field `AD0CMP` reader - AD0CMP"]
pub type AD0CMP_R = crate::BitReader;
#[doc = "Field `AD0CMP` writer - AD0CMP"]
pub type AD0CMP_W<'a, const O: u8> = crate::BitWriter<'a, ADC0TCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD0SW"]
    #[inline(always)]
    pub fn ad0sw(&self) -> AD0SW_R {
        AD0SW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD0EXTI"]
    #[inline(always)]
    pub fn ad0exti(&self) -> AD0EXTI_R {
        AD0EXTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD0TM"]
    #[inline(always)]
    pub fn ad0tm(&self) -> AD0TM_R {
        AD0TM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD0BFTM"]
    #[inline(always)]
    pub fn ad0bftm(&self) -> AD0BFTM_R {
        AD0BFTM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD0CMP"]
    #[inline(always)]
    pub fn ad0cmp(&self) -> AD0CMP_R {
        AD0CMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD0SW"]
    #[inline(always)]
    #[must_use]
    pub fn ad0sw(&mut self) -> AD0SW_W<0> {
        AD0SW_W::new(self)
    }
    #[doc = "Bit 1 - AD0EXTI"]
    #[inline(always)]
    #[must_use]
    pub fn ad0exti(&mut self) -> AD0EXTI_W<1> {
        AD0EXTI_W::new(self)
    }
    #[doc = "Bit 2 - AD0TM"]
    #[inline(always)]
    #[must_use]
    pub fn ad0tm(&mut self) -> AD0TM_W<2> {
        AD0TM_W::new(self)
    }
    #[doc = "Bit 3 - AD0BFTM"]
    #[inline(always)]
    #[must_use]
    pub fn ad0bftm(&mut self) -> AD0BFTM_W<3> {
        AD0BFTM_W::new(self)
    }
    #[doc = "Bit 4 - AD0CMP"]
    #[inline(always)]
    #[must_use]
    pub fn ad0cmp(&mut self) -> AD0CMP_W<4> {
        AD0CMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0tcr](index.html) module"]
pub struct ADC0TCR_SPEC;
impl crate::RegisterSpec for ADC0TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0tcr::R](R) reader structure"]
impl crate::Readable for ADC0TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0tcr::W](W) writer structure"]
impl crate::Writable for ADC0TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0TCR to value 0"]
impl crate::Resettable for ADC0TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
