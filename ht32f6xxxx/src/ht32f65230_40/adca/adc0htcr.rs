#[doc = "Register `ADC0HTCR` reader"]
pub struct R(crate::R<ADC0HTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0HTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0HTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0HTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0HTCR` writer"]
pub struct W(crate::W<ADC0HTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0HTCR_SPEC>;
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
impl From<crate::W<ADC0HTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0HTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0HSW` reader - AD0HSW"]
pub type AD0HSW_R = crate::BitReader;
#[doc = "Field `AD0HSW` writer - AD0HSW"]
pub type AD0HSW_W<'a, const O: u8> = crate::BitWriter<'a, ADC0HTCR_SPEC, O>;
#[doc = "Field `AD0HEXTI` reader - AD0HEXTI"]
pub type AD0HEXTI_R = crate::BitReader;
#[doc = "Field `AD0HEXTI` writer - AD0HEXTI"]
pub type AD0HEXTI_W<'a, const O: u8> = crate::BitWriter<'a, ADC0HTCR_SPEC, O>;
#[doc = "Field `AD0HTM` reader - AD0HTM"]
pub type AD0HTM_R = crate::BitReader;
#[doc = "Field `AD0HTM` writer - AD0HTM"]
pub type AD0HTM_W<'a, const O: u8> = crate::BitWriter<'a, ADC0HTCR_SPEC, O>;
#[doc = "Field `AD0HBFTM` reader - AD0HBFTM"]
pub type AD0HBFTM_R = crate::BitReader;
#[doc = "Field `AD0HBFTM` writer - AD0HBFTM"]
pub type AD0HBFTM_W<'a, const O: u8> = crate::BitWriter<'a, ADC0HTCR_SPEC, O>;
#[doc = "Field `AD0HCMP` reader - AD0HCMP"]
pub type AD0HCMP_R = crate::BitReader;
#[doc = "Field `AD0HCMP` writer - AD0HCMP"]
pub type AD0HCMP_W<'a, const O: u8> = crate::BitWriter<'a, ADC0HTCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - AD0HSW"]
    #[inline(always)]
    pub fn ad0hsw(&self) -> AD0HSW_R {
        AD0HSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD0HEXTI"]
    #[inline(always)]
    pub fn ad0hexti(&self) -> AD0HEXTI_R {
        AD0HEXTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD0HTM"]
    #[inline(always)]
    pub fn ad0htm(&self) -> AD0HTM_R {
        AD0HTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD0HBFTM"]
    #[inline(always)]
    pub fn ad0hbftm(&self) -> AD0HBFTM_R {
        AD0HBFTM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD0HCMP"]
    #[inline(always)]
    pub fn ad0hcmp(&self) -> AD0HCMP_R {
        AD0HCMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD0HSW"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hsw(&mut self) -> AD0HSW_W<0> {
        AD0HSW_W::new(self)
    }
    #[doc = "Bit 1 - AD0HEXTI"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hexti(&mut self) -> AD0HEXTI_W<1> {
        AD0HEXTI_W::new(self)
    }
    #[doc = "Bit 2 - AD0HTM"]
    #[inline(always)]
    #[must_use]
    pub fn ad0htm(&mut self) -> AD0HTM_W<2> {
        AD0HTM_W::new(self)
    }
    #[doc = "Bit 3 - AD0HBFTM"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hbftm(&mut self) -> AD0HBFTM_W<3> {
        AD0HBFTM_W::new(self)
    }
    #[doc = "Bit 4 - AD0HCMP"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hcmp(&mut self) -> AD0HCMP_W<4> {
        AD0HCMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0HTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0htcr](index.html) module"]
pub struct ADC0HTCR_SPEC;
impl crate::RegisterSpec for ADC0HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0htcr::R](R) reader structure"]
impl crate::Readable for ADC0HTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0htcr::W](W) writer structure"]
impl crate::Writable for ADC0HTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0HTCR to value 0"]
impl crate::Resettable for ADC0HTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
