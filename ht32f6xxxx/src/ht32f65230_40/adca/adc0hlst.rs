#[doc = "Register `ADC0HLST` reader"]
pub struct R(crate::R<ADC0HLST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0HLST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0HLST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0HLST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0HLST` writer"]
pub struct W(crate::W<ADC0HLST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0HLST_SPEC>;
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
impl From<crate::W<ADC0HLST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0HLST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0HSEQ0` reader - AD0HSEQ0"]
pub type AD0HSEQ0_R = crate::FieldReader;
#[doc = "Field `AD0HSEQ0` writer - AD0HSEQ0"]
pub type AD0HSEQ0_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HLST_SPEC, 5, O>;
#[doc = "Field `AD0HSEQ1` reader - AD0HSEQ1"]
pub type AD0HSEQ1_R = crate::FieldReader;
#[doc = "Field `AD0HSEQ1` writer - AD0HSEQ1"]
pub type AD0HSEQ1_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HLST_SPEC, 5, O>;
#[doc = "Field `AD0HSEQ2` reader - AD0HSEQ2"]
pub type AD0HSEQ2_R = crate::FieldReader;
#[doc = "Field `AD0HSEQ2` writer - AD0HSEQ2"]
pub type AD0HSEQ2_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HLST_SPEC, 5, O>;
#[doc = "Field `AD0HSEQ3` reader - AD0HSEQ3"]
pub type AD0HSEQ3_R = crate::FieldReader;
#[doc = "Field `AD0HSEQ3` writer - AD0HSEQ3"]
pub type AD0HSEQ3_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HLST_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - AD0HSEQ0"]
    #[inline(always)]
    pub fn ad0hseq0(&self) -> AD0HSEQ0_R {
        AD0HSEQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - AD0HSEQ1"]
    #[inline(always)]
    pub fn ad0hseq1(&self) -> AD0HSEQ1_R {
        AD0HSEQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - AD0HSEQ2"]
    #[inline(always)]
    pub fn ad0hseq2(&self) -> AD0HSEQ2_R {
        AD0HSEQ2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - AD0HSEQ3"]
    #[inline(always)]
    pub fn ad0hseq3(&self) -> AD0HSEQ3_R {
        AD0HSEQ3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - AD0HSEQ0"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hseq0(&mut self) -> AD0HSEQ0_W<0> {
        AD0HSEQ0_W::new(self)
    }
    #[doc = "Bits 8:12 - AD0HSEQ1"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hseq1(&mut self) -> AD0HSEQ1_W<8> {
        AD0HSEQ1_W::new(self)
    }
    #[doc = "Bits 16:20 - AD0HSEQ2"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hseq2(&mut self) -> AD0HSEQ2_W<16> {
        AD0HSEQ2_W::new(self)
    }
    #[doc = "Bits 24:28 - AD0HSEQ3"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hseq3(&mut self) -> AD0HSEQ3_W<24> {
        AD0HSEQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0HLST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0hlst](index.html) module"]
pub struct ADC0HLST_SPEC;
impl crate::RegisterSpec for ADC0HLST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0hlst::R](R) reader structure"]
impl crate::Readable for ADC0HLST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0hlst::W](W) writer structure"]
impl crate::Writable for ADC0HLST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0HLST to value 0"]
impl crate::Resettable for ADC0HLST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
