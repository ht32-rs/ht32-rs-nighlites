#[doc = "Register `ADC1HLST` reader"]
pub struct R(crate::R<ADC1HLST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1HLST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1HLST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1HLST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1HLST` writer"]
pub struct W(crate::W<ADC1HLST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1HLST_SPEC>;
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
impl From<crate::W<ADC1HLST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1HLST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1HSEQ0` reader - AD1HSEQ0"]
pub type AD1HSEQ0_R = crate::FieldReader;
#[doc = "Field `AD1HSEQ0` writer - AD1HSEQ0"]
pub type AD1HSEQ0_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HLST_SPEC, 5, O>;
#[doc = "Field `AD1HSEQ1` reader - AD1HSEQ1"]
pub type AD1HSEQ1_R = crate::FieldReader;
#[doc = "Field `AD1HSEQ1` writer - AD1HSEQ1"]
pub type AD1HSEQ1_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HLST_SPEC, 5, O>;
#[doc = "Field `AD1HSEQ2` reader - AD1HSEQ2"]
pub type AD1HSEQ2_R = crate::FieldReader;
#[doc = "Field `AD1HSEQ2` writer - AD1HSEQ2"]
pub type AD1HSEQ2_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HLST_SPEC, 5, O>;
#[doc = "Field `AD1HSEQ3` reader - AD1HSEQ3"]
pub type AD1HSEQ3_R = crate::FieldReader;
#[doc = "Field `AD1HSEQ3` writer - AD1HSEQ3"]
pub type AD1HSEQ3_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HLST_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - AD1HSEQ0"]
    #[inline(always)]
    pub fn ad1hseq0(&self) -> AD1HSEQ0_R {
        AD1HSEQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - AD1HSEQ1"]
    #[inline(always)]
    pub fn ad1hseq1(&self) -> AD1HSEQ1_R {
        AD1HSEQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - AD1HSEQ2"]
    #[inline(always)]
    pub fn ad1hseq2(&self) -> AD1HSEQ2_R {
        AD1HSEQ2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - AD1HSEQ3"]
    #[inline(always)]
    pub fn ad1hseq3(&self) -> AD1HSEQ3_R {
        AD1HSEQ3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - AD1HSEQ0"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hseq0(&mut self) -> AD1HSEQ0_W<0> {
        AD1HSEQ0_W::new(self)
    }
    #[doc = "Bits 8:12 - AD1HSEQ1"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hseq1(&mut self) -> AD1HSEQ1_W<8> {
        AD1HSEQ1_W::new(self)
    }
    #[doc = "Bits 16:20 - AD1HSEQ2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hseq2(&mut self) -> AD1HSEQ2_W<16> {
        AD1HSEQ2_W::new(self)
    }
    #[doc = "Bits 24:28 - AD1HSEQ3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hseq3(&mut self) -> AD1HSEQ3_W<24> {
        AD1HSEQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1HLST\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1hlst](index.html) module"]
pub struct ADC1HLST_SPEC;
impl crate::RegisterSpec for ADC1HLST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1hlst::R](R) reader structure"]
impl crate::Readable for ADC1HLST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1hlst::W](W) writer structure"]
impl crate::Writable for ADC1HLST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1HLST to value 0"]
impl crate::Resettable for ADC1HLST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
