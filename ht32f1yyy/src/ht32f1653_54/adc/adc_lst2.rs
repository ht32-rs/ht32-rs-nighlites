#[doc = "Register `ADC_LST2` reader"]
pub struct R(crate::R<ADC_LST2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_LST2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_LST2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_LST2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_LST2` writer"]
pub struct W(crate::W<ADC_LST2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_LST2_SPEC>;
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
impl From<crate::W<ADC_LST2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_LST2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSEQ8` reader - ADSEQ8"]
pub type ADSEQ8_R = crate::FieldReader;
#[doc = "Field `ADSEQ8` writer - ADSEQ8"]
pub type ADSEQ8_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST2_SPEC, 5, O>;
#[doc = "Field `ADSEQ9` reader - ADSEQ9"]
pub type ADSEQ9_R = crate::FieldReader;
#[doc = "Field `ADSEQ9` writer - ADSEQ9"]
pub type ADSEQ9_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST2_SPEC, 5, O>;
#[doc = "Field `ADSEQ10` reader - ADSEQ10"]
pub type ADSEQ10_R = crate::FieldReader;
#[doc = "Field `ADSEQ10` writer - ADSEQ10"]
pub type ADSEQ10_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST2_SPEC, 5, O>;
#[doc = "Field `ADSEQ11` reader - ADSEQ11"]
pub type ADSEQ11_R = crate::FieldReader;
#[doc = "Field `ADSEQ11` writer - ADSEQ11"]
pub type ADSEQ11_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_LST2_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - ADSEQ8"]
    #[inline(always)]
    pub fn adseq8(&self) -> ADSEQ8_R {
        ADSEQ8_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - ADSEQ9"]
    #[inline(always)]
    pub fn adseq9(&self) -> ADSEQ9_R {
        ADSEQ9_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADSEQ10"]
    #[inline(always)]
    pub fn adseq10(&self) -> ADSEQ10_R {
        ADSEQ10_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - ADSEQ11"]
    #[inline(always)]
    pub fn adseq11(&self) -> ADSEQ11_R {
        ADSEQ11_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ADSEQ8"]
    #[inline(always)]
    #[must_use]
    pub fn adseq8(&mut self) -> ADSEQ8_W<0> {
        ADSEQ8_W::new(self)
    }
    #[doc = "Bits 8:12 - ADSEQ9"]
    #[inline(always)]
    #[must_use]
    pub fn adseq9(&mut self) -> ADSEQ9_W<8> {
        ADSEQ9_W::new(self)
    }
    #[doc = "Bits 16:20 - ADSEQ10"]
    #[inline(always)]
    #[must_use]
    pub fn adseq10(&mut self) -> ADSEQ10_W<16> {
        ADSEQ10_W::new(self)
    }
    #[doc = "Bits 24:28 - ADSEQ11"]
    #[inline(always)]
    #[must_use]
    pub fn adseq11(&mut self) -> ADSEQ11_W<24> {
        ADSEQ11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_LST2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_lst2](index.html) module"]
pub struct ADC_LST2_SPEC;
impl crate::RegisterSpec for ADC_LST2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_lst2::R](R) reader structure"]
impl crate::Readable for ADC_LST2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_lst2::W](W) writer structure"]
impl crate::Writable for ADC_LST2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_LST2 to value 0"]
impl crate::Resettable for ADC_LST2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
