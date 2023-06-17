#[doc = "Register `ADC_HDR0` reader"]
pub struct R(crate::R<ADC_HDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_HDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_HDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_HDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_HDR0` writer"]
pub struct W(crate::W<ADC_HDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_HDR0_SPEC>;
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
impl From<crate::W<ADC_HDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_HDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADHD0` reader - ADHD0"]
pub type ADHD0_R = crate::FieldReader<u16>;
#[doc = "Field `ADHD0` writer - ADHD0"]
pub type ADHD0_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HDR0_SPEC, 16, O, u16>;
#[doc = "Field `ADHVLD0` reader - ADHVLD0"]
pub type ADHVLD0_R = crate::BitReader;
#[doc = "Field `ADHVLD0` writer - ADHVLD0"]
pub type ADHVLD0_W<'a, const O: u8> = crate::BitWriter<'a, ADC_HDR0_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADHD0"]
    #[inline(always)]
    pub fn adhd0(&self) -> ADHD0_R {
        ADHD0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADHVLD0"]
    #[inline(always)]
    pub fn adhvld0(&self) -> ADHVLD0_R {
        ADHVLD0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADHD0"]
    #[inline(always)]
    #[must_use]
    pub fn adhd0(&mut self) -> ADHD0_W<0> {
        ADHD0_W::new(self)
    }
    #[doc = "Bit 31 - ADHVLD0"]
    #[inline(always)]
    #[must_use]
    pub fn adhvld0(&mut self) -> ADHVLD0_W<31> {
        ADHVLD0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_HDR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hdr0](index.html) module"]
pub struct ADC_HDR0_SPEC;
impl crate::RegisterSpec for ADC_HDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_hdr0::R](R) reader structure"]
impl crate::Readable for ADC_HDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_hdr0::W](W) writer structure"]
impl crate::Writable for ADC_HDR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_HDR0 to value 0"]
impl crate::Resettable for ADC_HDR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
