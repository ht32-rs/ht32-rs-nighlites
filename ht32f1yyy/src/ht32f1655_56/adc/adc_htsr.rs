#[doc = "Register `ADC_HTSR` reader"]
pub struct R(crate::R<ADC_HTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_HTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_HTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_HTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_HTSR` writer"]
pub struct W(crate::W<ADC_HTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_HTSR_SPEC>;
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
impl From<crate::W<ADC_HTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_HTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADHSC` reader - ADHSC"]
pub type ADHSC_R = crate::BitReader;
#[doc = "Field `ADHSC` writer - ADHSC"]
pub type ADHSC_W<'a, const O: u8> = crate::BitWriter<'a, ADC_HTSR_SPEC, O>;
#[doc = "Field `ADHEXTIS` reader - ADHEXTIS"]
pub type ADHEXTIS_R = crate::FieldReader;
#[doc = "Field `ADHEXTIS` writer - ADHEXTIS"]
pub type ADHEXTIS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HTSR_SPEC, 4, O>;
#[doc = "Field `HTMS` reader - HTMS"]
pub type HTMS_R = crate::FieldReader;
#[doc = "Field `HTMS` writer - HTMS"]
pub type HTMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HTSR_SPEC, 3, O>;
#[doc = "Field `HBFTMS` reader - HBFTMS"]
pub type HBFTMS_R = crate::BitReader;
#[doc = "Field `HBFTMS` writer - HBFTMS"]
pub type HBFTMS_W<'a, const O: u8> = crate::BitWriter<'a, ADC_HTSR_SPEC, O>;
#[doc = "Field `HTME` reader - HTME"]
pub type HTME_R = crate::FieldReader;
#[doc = "Field `HTME` writer - HTME"]
pub type HTME_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HTSR_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - ADHSC"]
    #[inline(always)]
    pub fn adhsc(&self) -> ADHSC_R {
        ADHSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADHEXTIS"]
    #[inline(always)]
    pub fn adhextis(&self) -> ADHEXTIS_R {
        ADHEXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - HTMS"]
    #[inline(always)]
    pub fn htms(&self) -> HTMS_R {
        HTMS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - HBFTMS"]
    #[inline(always)]
    pub fn hbftms(&self) -> HBFTMS_R {
        HBFTMS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:26 - HTME"]
    #[inline(always)]
    pub fn htme(&self) -> HTME_R {
        HTME_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADHSC"]
    #[inline(always)]
    #[must_use]
    pub fn adhsc(&mut self) -> ADHSC_W<0> {
        ADHSC_W::new(self)
    }
    #[doc = "Bits 8:11 - ADHEXTIS"]
    #[inline(always)]
    #[must_use]
    pub fn adhextis(&mut self) -> ADHEXTIS_W<8> {
        ADHEXTIS_W::new(self)
    }
    #[doc = "Bits 16:18 - HTMS"]
    #[inline(always)]
    #[must_use]
    pub fn htms(&mut self) -> HTMS_W<16> {
        HTMS_W::new(self)
    }
    #[doc = "Bit 19 - HBFTMS"]
    #[inline(always)]
    #[must_use]
    pub fn hbftms(&mut self) -> HBFTMS_W<19> {
        HBFTMS_W::new(self)
    }
    #[doc = "Bits 24:26 - HTME"]
    #[inline(always)]
    #[must_use]
    pub fn htme(&mut self) -> HTME_W<24> {
        HTME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_HTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_htsr](index.html) module"]
pub struct ADC_HTSR_SPEC;
impl crate::RegisterSpec for ADC_HTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_htsr::R](R) reader structure"]
impl crate::Readable for ADC_HTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_htsr::W](W) writer structure"]
impl crate::Writable for ADC_HTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_HTSR to value 0"]
impl crate::Resettable for ADC_HTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
