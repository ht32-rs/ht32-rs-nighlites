#[doc = "Register `ADC_HCONV` reader"]
pub struct R(crate::R<ADC_HCONV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_HCONV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_HCONV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_HCONV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_HCONV` writer"]
pub struct W(crate::W<ADC_HCONV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_HCONV_SPEC>;
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
impl From<crate::W<ADC_HCONV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_HCONV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADHMODE` reader - ADHMODE"]
pub type ADHMODE_R = crate::FieldReader;
#[doc = "Field `ADHMODE` writer - ADHMODE"]
pub type ADHMODE_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HCONV_SPEC, 2, O>;
#[doc = "Field `ADHSEQL` reader - ADHSEQL"]
pub type ADHSEQL_R = crate::FieldReader;
#[doc = "Field `ADHSEQL` writer - ADHSEQL"]
pub type ADHSEQL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HCONV_SPEC, 2, O>;
#[doc = "Field `ADHSUBL` reader - ADHSUBL"]
pub type ADHSUBL_R = crate::FieldReader;
#[doc = "Field `ADHSUBL` writer - ADHSUBL"]
pub type ADHSUBL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC_HCONV_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - ADHMODE"]
    #[inline(always)]
    pub fn adhmode(&self) -> ADHMODE_R {
        ADHMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - ADHSEQL"]
    #[inline(always)]
    pub fn adhseql(&self) -> ADHSEQL_R {
        ADHSEQL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ADHSUBL"]
    #[inline(always)]
    pub fn adhsubl(&self) -> ADHSUBL_R {
        ADHSUBL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADHMODE"]
    #[inline(always)]
    #[must_use]
    pub fn adhmode(&mut self) -> ADHMODE_W<0> {
        ADHMODE_W::new(self)
    }
    #[doc = "Bits 8:9 - ADHSEQL"]
    #[inline(always)]
    #[must_use]
    pub fn adhseql(&mut self) -> ADHSEQL_W<8> {
        ADHSEQL_W::new(self)
    }
    #[doc = "Bits 16:17 - ADHSUBL"]
    #[inline(always)]
    #[must_use]
    pub fn adhsubl(&mut self) -> ADHSUBL_W<16> {
        ADHSUBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_HCONV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_hconv](index.html) module"]
pub struct ADC_HCONV_SPEC;
impl crate::RegisterSpec for ADC_HCONV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_hconv::R](R) reader structure"]
impl crate::Readable for ADC_HCONV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_hconv::W](W) writer structure"]
impl crate::Writable for ADC_HCONV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC_HCONV to value 0"]
impl crate::Resettable for ADC_HCONV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
