#[doc = "Register `ADC1STR2` reader"]
pub struct R(crate::R<ADC1STR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1STR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1STR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1STR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1STR2` writer"]
pub struct W(crate::W<ADC1STR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1STR2_SPEC>;
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
impl From<crate::W<ADC1STR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1STR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1ST` reader - AD1ST"]
pub type AD1ST_R = crate::FieldReader;
#[doc = "Field `AD1ST` writer - AD1ST"]
pub type AD1ST_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1STR2_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - AD1ST"]
    #[inline(always)]
    pub fn ad1st(&self) -> AD1ST_R {
        AD1ST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AD1ST"]
    #[inline(always)]
    #[must_use]
    pub fn ad1st(&mut self) -> AD1ST_W<0> {
        AD1ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1STR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1str2](index.html) module"]
pub struct ADC1STR2_SPEC;
impl crate::RegisterSpec for ADC1STR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1str2::R](R) reader structure"]
impl crate::Readable for ADC1STR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1str2::W](W) writer structure"]
impl crate::Writable for ADC1STR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1STR2 to value 0"]
impl crate::Resettable for ADC1STR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
