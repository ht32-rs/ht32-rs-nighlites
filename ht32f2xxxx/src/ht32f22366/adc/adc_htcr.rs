#[doc = "Register `ADC_HTCR` reader"]
pub struct R(crate::R<ADC_HTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_HTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_HTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_HTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_HTCR` writer"]
pub struct W(crate::W<ADC_HTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_HTCR_SPEC>;
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
impl From<crate::W<ADC_HTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_HTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADHSW` reader - ADHSW"]
pub type ADHSW_R = crate::BitReader;
#[doc = "Field `ADHSW` writer - ADHSW"]
pub type ADHSW_W<'a, const O: u8> = crate::BitWriter<'a, ADC_HTCR_SPEC, O>;
#[doc = "Field `ADHEXTI` reader - ADHEXTI"]
pub type ADHEXTI_R = crate::BitReader;
#[doc = "Field `ADHEXTI` writer - ADHEXTI"]
pub type ADHEXTI_W<'a, const O: u8> = crate::BitWriter<'a, ADC_HTCR_SPEC, O>;
#[doc = "Field `HTM` reader - HTM"]
pub type HTM_R = crate::BitReader;
#[doc = "Field `HTM` writer - HTM"]
pub type HTM_W<'a, const O: u8> = crate::BitWriter<'a, ADC_HTCR_SPEC, O>;
#[doc = "Field `HBFTM` reader - HBFTM"]
pub type HBFTM_R = crate::BitReader;
#[doc = "Field `HBFTM` writer - HBFTM"]
pub type HBFTM_W<'a, const O: u8> = crate::BitWriter<'a, ADC_HTCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - ADHSW"]
    #[inline(always)]
    pub fn adhsw(&self) -> ADHSW_R {
        ADHSW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADHEXTI"]
    #[inline(always)]
    pub fn adhexti(&self) -> ADHEXTI_R {
        ADHEXTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HTM"]
    #[inline(always)]
    pub fn htm(&self) -> HTM_R {
        HTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HBFTM"]
    #[inline(always)]
    pub fn hbftm(&self) -> HBFTM_R {
        HBFTM_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADHSW"]
    #[inline(always)]
    #[must_use]
    pub fn adhsw(&mut self) -> ADHSW_W<0> {
        ADHSW_W::new(self)
    }
    #[doc = "Bit 1 - ADHEXTI"]
    #[inline(always)]
    #[must_use]
    pub fn adhexti(&mut self) -> ADHEXTI_W<1> {
        ADHEXTI_W::new(self)
    }
    #[doc = "Bit 2 - HTM"]
    #[inline(always)]
    #[must_use]
    pub fn htm(&mut self) -> HTM_W<2> {
        HTM_W::new(self)
    }
    #[doc = "Bit 3 - HBFTM"]
    #[inline(always)]
    #[must_use]
    pub fn hbftm(&mut self) -> HBFTM_W<3> {
        HBFTM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_HTCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_htcr](index.html) module"]
pub struct ADC_HTCR_SPEC;
impl crate::RegisterSpec for ADC_HTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_htcr::R](R) reader structure"]
impl crate::Readable for ADC_HTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_htcr::W](W) writer structure"]
impl crate::Writable for ADC_HTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_HTCR to value 0"]
impl crate::Resettable for ADC_HTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
