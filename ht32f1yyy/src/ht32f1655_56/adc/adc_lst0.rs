#[doc = "Register `ADC_LST0` reader"]
pub struct R(crate::R<ADC_LST0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_LST0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_LST0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_LST0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_LST0` writer"]
pub struct W(crate::W<ADC_LST0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_LST0_SPEC>;
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
impl From<crate::W<ADC_LST0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_LST0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSEQ0` reader - ADSEQ0"]
pub type ADSEQ0_R = crate::FieldReader;
#[doc = "Field `ADSEQ0` writer - ADSEQ0"]
pub type ADSEQ0_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST0_SPEC, 5, O>;
#[doc = "Field `ADSEQ1` reader - ADSEQ1"]
pub type ADSEQ1_R = crate::FieldReader;
#[doc = "Field `ADSEQ1` writer - ADSEQ1"]
pub type ADSEQ1_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST0_SPEC, 5, O>;
#[doc = "Field `ADSEQ2` reader - ADSEQ2"]
pub type ADSEQ2_R = crate::FieldReader;
#[doc = "Field `ADSEQ2` writer - ADSEQ2"]
pub type ADSEQ2_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST0_SPEC, 5, O>;
#[doc = "Field `ADSEQ3` reader - ADSEQ3"]
pub type ADSEQ3_R = crate::FieldReader;
#[doc = "Field `ADSEQ3` writer - ADSEQ3"]
pub type ADSEQ3_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST0_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ADSEQ0"]
    #[inline(always)]
    pub fn adseq0(&self) -> ADSEQ0_R {
        ADSEQ0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ1"]
    #[inline(always)]
    pub fn adseq1(&self) -> ADSEQ1_R {
        ADSEQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ2"]
    #[inline(always)]
    pub fn adseq2(&self) -> ADSEQ2_R {
        ADSEQ2_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ3"]
    #[inline(always)]
    pub fn adseq3(&self) -> ADSEQ3_R {
        ADSEQ3_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ0"]
    #[inline(always)]
    #[must_use]
    pub fn adseq0(&mut self) -> ADSEQ0_W<0> {
        ADSEQ0_W::new(self)
    }
    #[doc = "Bits 8:12 - ADSEQ1"]
    #[inline(always)]
    #[must_use]
    pub fn adseq1(&mut self) -> ADSEQ1_W<8> {
        ADSEQ1_W::new(self)
    }
    #[doc = "Bits 16:20 - ADSEQ2"]
    #[inline(always)]
    #[must_use]
    pub fn adseq2(&mut self) -> ADSEQ2_W<16> {
        ADSEQ2_W::new(self)
    }
    #[doc = "Bits 24:28 - ADSEQ3"]
    #[inline(always)]
    #[must_use]
    pub fn adseq3(&mut self) -> ADSEQ3_W<24> {
        ADSEQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_LST0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_lst0](index.html) module"]
pub struct ADC_LST0_SPEC;
impl crate::RegisterSpec for ADC_LST0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_lst0::R](R) reader structure"]
impl crate::Readable for ADC_LST0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_lst0::W](W) writer structure"]
impl crate::Writable for ADC_LST0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_LST0 to value 0"]
impl crate::Resettable for ADC_LST0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
