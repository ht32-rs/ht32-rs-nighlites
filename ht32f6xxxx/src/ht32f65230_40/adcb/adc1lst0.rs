#[doc = "Register `ADC1LST0` reader"]
pub struct R(crate::R<ADC1LST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1LST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1LST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1LST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1LST0` writer"]
pub struct W(crate::W<ADC1LST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1LST0_SPEC>;
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
impl From<crate::W<ADC1LST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1LST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1SEQ0` reader - AD1SEQ0"]
pub type AD1SEQ0_R = crate::FieldReader;
#[doc = "Field `AD1SEQ0` writer - AD1SEQ0"]
pub type AD1SEQ0_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1LST0_SPEC, 5, O>;
#[doc = "Field `AD1SEQ1` reader - AD1SEQ1"]
pub type AD1SEQ1_R = crate::FieldReader;
#[doc = "Field `AD1SEQ1` writer - AD1SEQ1"]
pub type AD1SEQ1_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1LST0_SPEC, 5, O>;
#[doc = "Field `AD1SEQ2` reader - AD1SEQ2"]
pub type AD1SEQ2_R = crate::FieldReader;
#[doc = "Field `AD1SEQ2` writer - AD1SEQ2"]
pub type AD1SEQ2_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1LST0_SPEC, 5, O>;
#[doc = "Field `AD1SEQ3` reader - AD1SEQ3"]
pub type AD1SEQ3_R = crate::FieldReader;
#[doc = "Field `AD1SEQ3` writer - AD1SEQ3"]
pub type AD1SEQ3_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1LST0_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - AD1SEQ0"]
    #[inline(always)]
    pub fn ad1seq0(&self) -> AD1SEQ0_R {
        AD1SEQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - AD1SEQ1"]
    #[inline(always)]
    pub fn ad1seq1(&self) -> AD1SEQ1_R {
        AD1SEQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - AD1SEQ2"]
    #[inline(always)]
    pub fn ad1seq2(&self) -> AD1SEQ2_R {
        AD1SEQ2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - AD1SEQ3"]
    #[inline(always)]
    pub fn ad1seq3(&self) -> AD1SEQ3_R {
        AD1SEQ3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - AD1SEQ0"]
    #[inline(always)]
    #[must_use]
    pub fn ad1seq0(&mut self) -> AD1SEQ0_W<0> {
        AD1SEQ0_W::new(self)
    }
    #[doc = "Bits 8:12 - AD1SEQ1"]
    #[inline(always)]
    #[must_use]
    pub fn ad1seq1(&mut self) -> AD1SEQ1_W<8> {
        AD1SEQ1_W::new(self)
    }
    #[doc = "Bits 16:20 - AD1SEQ2"]
    #[inline(always)]
    #[must_use]
    pub fn ad1seq2(&mut self) -> AD1SEQ2_W<16> {
        AD1SEQ2_W::new(self)
    }
    #[doc = "Bits 24:28 - AD1SEQ3"]
    #[inline(always)]
    #[must_use]
    pub fn ad1seq3(&mut self) -> AD1SEQ3_W<24> {
        AD1SEQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1LST0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1lst0](index.html) module"]
pub struct ADC1LST0_SPEC;
impl crate::RegisterSpec for ADC1LST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1lst0::R](R) reader structure"]
impl crate::Readable for ADC1LST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1lst0::W](W) writer structure"]
impl crate::Writable for ADC1LST0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1LST0 to value 0"]
impl crate::Resettable for ADC1LST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
