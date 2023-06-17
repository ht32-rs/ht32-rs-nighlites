#[doc = "Register `ADC_HDR1` reader"]
pub struct R(crate::R<ADC_HDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_HDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_HDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_HDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_HDR1` writer"]
pub struct W(crate::W<ADC_HDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_HDR1_SPEC>;
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
impl From<crate::W<ADC_HDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_HDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADHD1` reader - ADHD1"]
pub type ADHD1_R = crate::FieldReader<u16>;
#[doc = "Field `ADHD1` writer - ADHD1"]
pub type ADHD1_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HDR1_SPEC, 16, O, u16>;
#[doc = "Field `ADHVLD1` reader - ADHVLD1"]
pub type ADHVLD1_R = crate::BitReader;
#[doc = "Field `ADHVLD1` writer - ADHVLD1"]
pub type ADHVLD1_W<'a, const O: u8> = crate::BitWriter<'a, ADC_HDR1_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADHD1"]
    #[inline(always)]
    pub fn adhd1(&self) -> ADHD1_R {
        ADHD1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADHVLD1"]
    #[inline(always)]
    pub fn adhvld1(&self) -> ADHVLD1_R {
        ADHVLD1_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADHD1"]
    #[inline(always)]
    #[must_use]
    pub fn adhd1(&mut self) -> ADHD1_W<0> {
        ADHD1_W::new(self)
    }
    #[doc = "Bit 31 - ADHVLD1"]
    #[inline(always)]
    #[must_use]
    pub fn adhvld1(&mut self) -> ADHVLD1_W<31> {
        ADHVLD1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_HDR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hdr1](index.html) module"]
pub struct ADC_HDR1_SPEC;
impl crate::RegisterSpec for ADC_HDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_hdr1::R](R) reader structure"]
impl crate::Readable for ADC_HDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_hdr1::W](W) writer structure"]
impl crate::Writable for ADC_HDR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_HDR1 to value 0"]
impl crate::Resettable for ADC_HDR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
