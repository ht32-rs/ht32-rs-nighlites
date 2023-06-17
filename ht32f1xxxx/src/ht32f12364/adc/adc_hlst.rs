#[doc = "Register `ADC_HLST` reader"]
pub struct R(crate::R<ADC_HLST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_HLST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_HLST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_HLST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_HLST` writer"]
pub struct W(crate::W<ADC_HLST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_HLST_SPEC>;
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
impl From<crate::W<ADC_HLST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_HLST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADHSEQ0` reader - ADHSEQ0"]
pub type ADHSEQ0_R = crate::FieldReader;
#[doc = "Field `ADHSEQ0` writer - ADHSEQ0"]
pub type ADHSEQ0_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HLST_SPEC, 5, O>;
#[doc = "Field `ADHSEQ1` reader - ADHSEQ1"]
pub type ADHSEQ1_R = crate::FieldReader;
#[doc = "Field `ADHSEQ1` writer - ADHSEQ1"]
pub type ADHSEQ1_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HLST_SPEC, 5, O>;
#[doc = "Field `ADHSEQ2` reader - ADHSEQ2"]
pub type ADHSEQ2_R = crate::FieldReader;
#[doc = "Field `ADHSEQ2` writer - ADHSEQ2"]
pub type ADHSEQ2_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HLST_SPEC, 5, O>;
#[doc = "Field `ADHSEQ3` reader - ADHSEQ3"]
pub type ADHSEQ3_R = crate::FieldReader;
#[doc = "Field `ADHSEQ3` writer - ADHSEQ3"]
pub type ADHSEQ3_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HLST_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ADHSEQ0"]
    #[inline(always)]
    pub fn adhseq0(&self) -> ADHSEQ0_R {
        ADHSEQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADHSEQ1"]
    #[inline(always)]
    pub fn adhseq1(&self) -> ADHSEQ1_R {
        ADHSEQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADHSEQ2"]
    #[inline(always)]
    pub fn adhseq2(&self) -> ADHSEQ2_R {
        ADHSEQ2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADHSEQ3"]
    #[inline(always)]
    pub fn adhseq3(&self) -> ADHSEQ3_R {
        ADHSEQ3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADHSEQ0"]
    #[inline(always)]
    #[must_use]
    pub fn adhseq0(&mut self) -> ADHSEQ0_W<0> {
        ADHSEQ0_W::new(self)
    }
    #[doc = "Bits 8:12 - ADHSEQ1"]
    #[inline(always)]
    #[must_use]
    pub fn adhseq1(&mut self) -> ADHSEQ1_W<8> {
        ADHSEQ1_W::new(self)
    }
    #[doc = "Bits 16:20 - ADHSEQ2"]
    #[inline(always)]
    #[must_use]
    pub fn adhseq2(&mut self) -> ADHSEQ2_W<16> {
        ADHSEQ2_W::new(self)
    }
    #[doc = "Bits 24:28 - ADHSEQ3"]
    #[inline(always)]
    #[must_use]
    pub fn adhseq3(&mut self) -> ADHSEQ3_W<24> {
        ADHSEQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_HLST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hlst](index.html) module"]
pub struct ADC_HLST_SPEC;
impl crate::RegisterSpec for ADC_HLST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_hlst::R](R) reader structure"]
impl crate::Readable for ADC_HLST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_hlst::W](W) writer structure"]
impl crate::Writable for ADC_HLST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_HLST to value 0"]
impl crate::Resettable for ADC_HLST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
