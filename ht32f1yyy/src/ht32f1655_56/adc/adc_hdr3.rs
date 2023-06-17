#[doc = "Register `ADC_HDR3` reader"]
pub struct R(crate::R<ADC_HDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_HDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_HDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_HDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_HDR3` writer"]
pub struct W(crate::W<ADC_HDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_HDR3_SPEC>;
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
impl From<crate::W<ADC_HDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_HDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADHD3` reader - ADHD3"]
pub type ADHD3_R = crate::FieldReader<u16>;
#[doc = "Field `ADHD3` writer - ADHD3"]
pub type ADHD3_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HDR3_SPEC, 16, O, u16>;
#[doc = "Field `ADHVLD3` reader - ADHVLD3"]
pub type ADHVLD3_R = crate::BitReader;
#[doc = "Field `ADHVLD3` writer - ADHVLD3"]
pub type ADHVLD3_W<'a, const O: u8> = crate::BitWriter<'a, ADC_HDR3_SPEC, O>;
impl R {
    #[doc = "Bits 0:15 - ADHD3"]
    #[inline(always)]
    pub fn adhd3(&self) -> ADHD3_R {
        ADHD3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - ADHVLD3"]
    #[inline(always)]
    pub fn adhvld3(&self) -> ADHVLD3_R {
        ADHVLD3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADHD3"]
    #[inline(always)]
    #[must_use]
    pub fn adhd3(&mut self) -> ADHD3_W<0> {
        ADHD3_W::new(self)
    }
    #[doc = "Bit 31 - ADHVLD3"]
    #[inline(always)]
    #[must_use]
    pub fn adhvld3(&mut self) -> ADHVLD3_W<31> {
        ADHVLD3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_HDR3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hdr3](index.html) module"]
pub struct ADC_HDR3_SPEC;
impl crate::RegisterSpec for ADC_HDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_hdr3::R](R) reader structure"]
impl crate::Readable for ADC_HDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_hdr3::W](W) writer structure"]
impl crate::Writable for ADC_HDR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_HDR3 to value 0"]
impl crate::Resettable for ADC_HDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
