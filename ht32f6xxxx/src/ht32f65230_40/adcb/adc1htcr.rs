#[doc = "Register `ADC1HTCR` reader"]
pub struct R(crate::R<ADC1HTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1HTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1HTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1HTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1HTCR` writer"]
pub struct W(crate::W<ADC1HTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1HTCR_SPEC>;
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
impl From<crate::W<ADC1HTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1HTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1HSW` reader - AD1HSW"]
pub type AD1HSW_R = crate::BitReader;
#[doc = "Field `AD1HSW` writer - AD1HSW"]
pub type AD1HSW_W<'a, const O: u8> = crate::BitWriter<'a, ADC1HTCR_SPEC, O>;
#[doc = "Field `AD1HEXTI` reader - AD1HEXTI"]
pub type AD1HEXTI_R = crate::BitReader;
#[doc = "Field `AD1HEXTI` writer - AD1HEXTI"]
pub type AD1HEXTI_W<'a, const O: u8> = crate::BitWriter<'a, ADC1HTCR_SPEC, O>;
#[doc = "Field `AD1HTM` reader - AD1HTM"]
pub type AD1HTM_R = crate::BitReader;
#[doc = "Field `AD1HTM` writer - AD1HTM"]
pub type AD1HTM_W<'a, const O: u8> = crate::BitWriter<'a, ADC1HTCR_SPEC, O>;
#[doc = "Field `AD1HBFTM` reader - AD1HBFTM"]
pub type AD1HBFTM_R = crate::BitReader;
#[doc = "Field `AD1HBFTM` writer - AD1HBFTM"]
pub type AD1HBFTM_W<'a, const O: u8> = crate::BitWriter<'a, ADC1HTCR_SPEC, O>;
#[doc = "Field `AD1HCMP` reader - AD1HCMP"]
pub type AD1HCMP_R = crate::BitReader;
#[doc = "Field `AD1HCMP` writer - AD1HCMP"]
pub type AD1HCMP_W<'a, const O: u8> = crate::BitWriter<'a, ADC1HTCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD1HSW"]
    #[inline(always)]
    pub fn ad1hsw(&self) -> AD1HSW_R {
        AD1HSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD1HEXTI"]
    #[inline(always)]
    pub fn ad1hexti(&self) -> AD1HEXTI_R {
        AD1HEXTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD1HTM"]
    #[inline(always)]
    pub fn ad1htm(&self) -> AD1HTM_R {
        AD1HTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD1HBFTM"]
    #[inline(always)]
    pub fn ad1hbftm(&self) -> AD1HBFTM_R {
        AD1HBFTM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD1HCMP"]
    #[inline(always)]
    pub fn ad1hcmp(&self) -> AD1HCMP_R {
        AD1HCMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD1HSW"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hsw(&mut self) -> AD1HSW_W<0> {
        AD1HSW_W::new(self)
    }
    #[doc = "Bit 1 - AD1HEXTI"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hexti(&mut self) -> AD1HEXTI_W<1> {
        AD1HEXTI_W::new(self)
    }
    #[doc = "Bit 2 - AD1HTM"]
    #[inline(always)]
    #[must_use]
    pub fn ad1htm(&mut self) -> AD1HTM_W<2> {
        AD1HTM_W::new(self)
    }
    #[doc = "Bit 3 - AD1HBFTM"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hbftm(&mut self) -> AD1HBFTM_W<3> {
        AD1HBFTM_W::new(self)
    }
    #[doc = "Bit 4 - AD1HCMP"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hcmp(&mut self) -> AD1HCMP_W<4> {
        AD1HCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1HTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1htcr](index.html) module"]
pub struct ADC1HTCR_SPEC;
impl crate::RegisterSpec for ADC1HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1htcr::R](R) reader structure"]
impl crate::Readable for ADC1HTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1htcr::W](W) writer structure"]
impl crate::Writable for ADC1HTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1HTCR to value 0"]
impl crate::Resettable for ADC1HTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
