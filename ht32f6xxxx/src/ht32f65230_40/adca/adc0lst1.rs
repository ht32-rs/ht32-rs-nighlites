#[doc = "Register `ADC0LST1` reader"]
pub struct R(crate::R<ADC0LST1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0LST1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0LST1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0LST1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0LST1` writer"]
pub struct W(crate::W<ADC0LST1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0LST1_SPEC>;
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
impl From<crate::W<ADC0LST1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0LST1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0SEQ0` reader - AD0SEQ0"]
pub type AD0SEQ0_R = crate::FieldReader;
#[doc = "Field `AD0SEQ0` writer - AD0SEQ0"]
pub type AD0SEQ0_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0LST1_SPEC, 5, O>;
#[doc = "Field `AD0SEQ1` reader - AD0SEQ1"]
pub type AD0SEQ1_R = crate::FieldReader;
#[doc = "Field `AD0SEQ1` writer - AD0SEQ1"]
pub type AD0SEQ1_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0LST1_SPEC, 5, O>;
#[doc = "Field `AD0SEQ2` reader - AD0SEQ2"]
pub type AD0SEQ2_R = crate::FieldReader;
#[doc = "Field `AD0SEQ2` writer - AD0SEQ2"]
pub type AD0SEQ2_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0LST1_SPEC, 5, O>;
#[doc = "Field `AD0SEQ3` reader - AD0SEQ3"]
pub type AD0SEQ3_R = crate::FieldReader;
#[doc = "Field `AD0SEQ3` writer - AD0SEQ3"]
pub type AD0SEQ3_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0LST1_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - AD0SEQ0"]
    #[inline(always)]
    pub fn ad0seq0(&self) -> AD0SEQ0_R {
        AD0SEQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - AD0SEQ1"]
    #[inline(always)]
    pub fn ad0seq1(&self) -> AD0SEQ1_R {
        AD0SEQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - AD0SEQ2"]
    #[inline(always)]
    pub fn ad0seq2(&self) -> AD0SEQ2_R {
        AD0SEQ2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - AD0SEQ3"]
    #[inline(always)]
    pub fn ad0seq3(&self) -> AD0SEQ3_R {
        AD0SEQ3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - AD0SEQ0"]
    #[inline(always)]
    #[must_use]
    pub fn ad0seq0(&mut self) -> AD0SEQ0_W<0> {
        AD0SEQ0_W::new(self)
    }
    #[doc = "Bits 8:12 - AD0SEQ1"]
    #[inline(always)]
    #[must_use]
    pub fn ad0seq1(&mut self) -> AD0SEQ1_W<8> {
        AD0SEQ1_W::new(self)
    }
    #[doc = "Bits 16:20 - AD0SEQ2"]
    #[inline(always)]
    #[must_use]
    pub fn ad0seq2(&mut self) -> AD0SEQ2_W<16> {
        AD0SEQ2_W::new(self)
    }
    #[doc = "Bits 24:28 - AD0SEQ3"]
    #[inline(always)]
    #[must_use]
    pub fn ad0seq3(&mut self) -> AD0SEQ3_W<24> {
        AD0SEQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0LST1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0lst1](index.html) module"]
pub struct ADC0LST1_SPEC;
impl crate::RegisterSpec for ADC0LST1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0lst1::R](R) reader structure"]
impl crate::Readable for ADC0LST1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0lst1::W](W) writer structure"]
impl crate::Writable for ADC0LST1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0LST1 to value 0"]
impl crate::Resettable for ADC0LST1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
