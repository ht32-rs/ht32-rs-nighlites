#[doc = "Register `ADC0HTSR` reader"]
pub struct R(crate::R<ADC0HTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0HTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0HTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0HTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0HTSR` writer"]
pub struct W(crate::W<ADC0HTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0HTSR_SPEC>;
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
impl From<crate::W<ADC0HTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0HTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0HSC` reader - AD0HSC"]
pub type AD0HSC_R = crate::BitReader;
#[doc = "Field `AD0HSC` writer - AD0HSC"]
pub type AD0HSC_W<'a, const O: u8> = crate::BitWriter<'a, ADC0HTSR_SPEC, O>;
#[doc = "Field `AD0HEXTIS` reader - AD0HEXTIS"]
pub type AD0HEXTIS_R = crate::FieldReader;
#[doc = "Field `AD0HEXTIS` writer - AD0HEXTIS"]
pub type AD0HEXTIS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HTSR_SPEC, 4, O>;
#[doc = "Field `AD0HTMS` reader - AD0HTMS"]
pub type AD0HTMS_R = crate::FieldReader;
#[doc = "Field `AD0HTMS` writer - AD0HTMS"]
pub type AD0HTMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HTSR_SPEC, 3, O>;
#[doc = "Field `AD0HBFTMS` reader - AD0HBFTMS"]
pub type AD0HBFTMS_R = crate::BitReader;
#[doc = "Field `AD0HBFTMS` writer - AD0HBFTMS"]
pub type AD0HBFTMS_W<'a, const O: u8> = crate::BitWriter<'a, ADC0HTSR_SPEC, O>;
#[doc = "Field `AD0HCMPS` reader - AD0HCMPS"]
pub type AD0HCMPS_R = crate::FieldReader;
#[doc = "Field `AD0HCMPS` writer - AD0HCMPS"]
pub type AD0HCMPS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HTSR_SPEC, 2, O>;
#[doc = "Field `AD0HTME` reader - AD0HTME"]
pub type AD0HTME_R = crate::FieldReader;
#[doc = "Field `AD0HTME` writer - AD0HTME"]
pub type AD0HTME_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HTSR_SPEC, 4, O>;
#[doc = "Field `AD0HMCMS` reader - AD0HMCMS"]
pub type AD0HMCMS_R = crate::FieldReader;
#[doc = "Field `AD0HMCMS` writer - AD0HMCMS"]
pub type AD0HMCMS_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HTSR_SPEC, 2, O>;
impl R {
    #[doc = "Bit 0 - AD0HSC"]
    #[inline(always)]
    pub fn ad0hsc(&self) -> AD0HSC_R {
        AD0HSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - AD0HEXTIS"]
    #[inline(always)]
    pub fn ad0hextis(&self) -> AD0HEXTIS_R {
        AD0HEXTIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - AD0HTMS"]
    #[inline(always)]
    pub fn ad0htms(&self) -> AD0HTMS_R {
        AD0HTMS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - AD0HBFTMS"]
    #[inline(always)]
    pub fn ad0hbftms(&self) -> AD0HBFTMS_R {
        AD0HBFTMS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - AD0HCMPS"]
    #[inline(always)]
    pub fn ad0hcmps(&self) -> AD0HCMPS_R {
        AD0HCMPS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:27 - AD0HTME"]
    #[inline(always)]
    pub fn ad0htme(&self) -> AD0HTME_R {
        AD0HTME_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - AD0HMCMS"]
    #[inline(always)]
    pub fn ad0hmcms(&self) -> AD0HMCMS_R {
        AD0HMCMS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AD0HSC"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hsc(&mut self) -> AD0HSC_W<0> {
        AD0HSC_W::new(self)
    }
    #[doc = "Bits 8:11 - AD0HEXTIS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hextis(&mut self) -> AD0HEXTIS_W<8> {
        AD0HEXTIS_W::new(self)
    }
    #[doc = "Bits 16:18 - AD0HTMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0htms(&mut self) -> AD0HTMS_W<16> {
        AD0HTMS_W::new(self)
    }
    #[doc = "Bit 19 - AD0HBFTMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hbftms(&mut self) -> AD0HBFTMS_W<19> {
        AD0HBFTMS_W::new(self)
    }
    #[doc = "Bits 20:21 - AD0HCMPS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hcmps(&mut self) -> AD0HCMPS_W<20> {
        AD0HCMPS_W::new(self)
    }
    #[doc = "Bits 24:27 - AD0HTME"]
    #[inline(always)]
    #[must_use]
    pub fn ad0htme(&mut self) -> AD0HTME_W<24> {
        AD0HTME_W::new(self)
    }
    #[doc = "Bits 28:29 - AD0HMCMS"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hmcms(&mut self) -> AD0HMCMS_W<28> {
        AD0HMCMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0HTSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0htsr](index.html) module"]
pub struct ADC0HTSR_SPEC;
impl crate::RegisterSpec for ADC0HTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0htsr::R](R) reader structure"]
impl crate::Readable for ADC0HTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0htsr::W](W) writer structure"]
impl crate::Writable for ADC0HTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0HTSR to value 0"]
impl crate::Resettable for ADC0HTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
