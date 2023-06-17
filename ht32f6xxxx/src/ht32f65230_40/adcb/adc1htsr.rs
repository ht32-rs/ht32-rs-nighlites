#[doc = "Register `ADC1HTSR` reader"]
pub struct R(crate::R<ADC1HTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1HTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1HTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1HTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1HTSR` writer"]
pub struct W(crate::W<ADC1HTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1HTSR_SPEC>;
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
impl From<crate::W<ADC1HTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1HTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1HSC` reader - AD1HSC"]
pub type AD1HSC_R = crate::BitReader;
#[doc = "Field `AD1HSC` writer - AD1HSC"]
pub type AD1HSC_W<'a, const O: u8> = crate::BitWriter<'a, ADC1HTSR_SPEC, O>;
#[doc = "Field `AD1HEXTIS` reader - AD1HEXTIS"]
pub type AD1HEXTIS_R = crate::FieldReader;
#[doc = "Field `AD1HEXTIS` writer - AD1HEXTIS"]
pub type AD1HEXTIS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HTSR_SPEC, 4, O>;
#[doc = "Field `AD1HTMS` reader - AD1HTMS"]
pub type AD1HTMS_R = crate::FieldReader;
#[doc = "Field `AD1HTMS` writer - AD1HTMS"]
pub type AD1HTMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HTSR_SPEC, 3, O>;
#[doc = "Field `AD1HBFTMS` reader - AD1HBFTMS"]
pub type AD1HBFTMS_R = crate::BitReader;
#[doc = "Field `AD1HBFTMS` writer - AD1HBFTMS"]
pub type AD1HBFTMS_W<'a, const O: u8> = crate::BitWriter<'a, ADC1HTSR_SPEC, O>;
#[doc = "Field `AD1HCMPS` reader - AD1HCMPS"]
pub type AD1HCMPS_R = crate::FieldReader;
#[doc = "Field `AD1HCMPS` writer - AD1HCMPS"]
pub type AD1HCMPS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HTSR_SPEC, 2, O>;
#[doc = "Field `AD1HTME` reader - AD1HTME"]
pub type AD1HTME_R = crate::FieldReader;
#[doc = "Field `AD1HTME` writer - AD1HTME"]
pub type AD1HTME_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HTSR_SPEC, 4, O>;
#[doc = "Field `AD1HMCMS` reader - AD1HMCMS"]
pub type AD1HMCMS_R = crate::FieldReader;
#[doc = "Field `AD1HMCMS` writer - AD1HMCMS"]
pub type AD1HMCMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HTSR_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - AD1HSC"]
    #[inline(always)]
    pub fn ad1hsc(&self) -> AD1HSC_R {
        AD1HSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - AD1HEXTIS"]
    #[inline(always)]
    pub fn ad1hextis(&self) -> AD1HEXTIS_R {
        AD1HEXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - AD1HTMS"]
    #[inline(always)]
    pub fn ad1htms(&self) -> AD1HTMS_R {
        AD1HTMS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - AD1HBFTMS"]
    #[inline(always)]
    pub fn ad1hbftms(&self) -> AD1HBFTMS_R {
        AD1HBFTMS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - AD1HCMPS"]
    #[inline(always)]
    pub fn ad1hcmps(&self) -> AD1HCMPS_R {
        AD1HCMPS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - AD1HTME"]
    #[inline(always)]
    pub fn ad1htme(&self) -> AD1HTME_R {
        AD1HTME_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - AD1HMCMS"]
    #[inline(always)]
    pub fn ad1hmcms(&self) -> AD1HMCMS_R {
        AD1HMCMS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AD1HSC"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hsc(&mut self) -> AD1HSC_W<0> {
        AD1HSC_W::new(self)
    }
    #[doc = "Bits 8:11 - AD1HEXTIS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hextis(&mut self) -> AD1HEXTIS_W<8> {
        AD1HEXTIS_W::new(self)
    }
    #[doc = "Bits 16:18 - AD1HTMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1htms(&mut self) -> AD1HTMS_W<16> {
        AD1HTMS_W::new(self)
    }
    #[doc = "Bit 19 - AD1HBFTMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hbftms(&mut self) -> AD1HBFTMS_W<19> {
        AD1HBFTMS_W::new(self)
    }
    #[doc = "Bits 20:21 - AD1HCMPS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hcmps(&mut self) -> AD1HCMPS_W<20> {
        AD1HCMPS_W::new(self)
    }
    #[doc = "Bits 24:27 - AD1HTME"]
    #[inline(always)]
    #[must_use]
    pub fn ad1htme(&mut self) -> AD1HTME_W<24> {
        AD1HTME_W::new(self)
    }
    #[doc = "Bits 28:29 - AD1HMCMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hmcms(&mut self) -> AD1HMCMS_W<28> {
        AD1HMCMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1HTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1htsr](index.html) module"]
pub struct ADC1HTSR_SPEC;
impl crate::RegisterSpec for ADC1HTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1htsr::R](R) reader structure"]
impl crate::Readable for ADC1HTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1htsr::W](W) writer structure"]
impl crate::Writable for ADC1HTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1HTSR to value 0"]
impl crate::Resettable for ADC1HTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
