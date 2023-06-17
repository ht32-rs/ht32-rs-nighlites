#[doc = "Register `ADC0HCONV` reader"]
pub struct R(crate::R<ADC0HCONV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0HCONV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0HCONV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0HCONV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0HCONV` writer"]
pub struct W(crate::W<ADC0HCONV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0HCONV_SPEC>;
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
impl From<crate::W<ADC0HCONV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0HCONV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0HMODE` reader - AD0HMODE"]
pub type AD0HMODE_R = crate::FieldReader;
#[doc = "Field `AD0HMODE` writer - AD0HMODE"]
pub type AD0HMODE_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HCONV_SPEC, 2, O>;
#[doc = "Field `AD0HSEQL` reader - AD0HSEQL"]
pub type AD0HSEQL_R = crate::FieldReader;
#[doc = "Field `AD0HSEQL` writer - AD0HSEQL"]
pub type AD0HSEQL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HCONV_SPEC, 2, O>;
#[doc = "Field `AD0HSUBL` reader - AD0HSUBL"]
pub type AD0HSUBL_R = crate::FieldReader;
#[doc = "Field `AD0HSUBL` writer - AD0HSUBL"]
pub type AD0HSUBL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0HCONV_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - AD0HMODE"]
    #[inline(always)]
    pub fn ad0hmode(&self) -> AD0HMODE_R {
        AD0HMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - AD0HSEQL"]
    #[inline(always)]
    pub fn ad0hseql(&self) -> AD0HSEQL_R {
        AD0HSEQL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - AD0HSUBL"]
    #[inline(always)]
    pub fn ad0hsubl(&self) -> AD0HSUBL_R {
        AD0HSUBL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AD0HMODE"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hmode(&mut self) -> AD0HMODE_W<0> {
        AD0HMODE_W::new(self)
    }
    #[doc = "Bits 8:9 - AD0HSEQL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hseql(&mut self) -> AD0HSEQL_W<8> {
        AD0HSEQL_W::new(self)
    }
    #[doc = "Bits 16:17 - AD0HSUBL"]
    #[inline(always)]
    #[must_use]
    pub fn ad0hsubl(&mut self) -> AD0HSUBL_W<16> {
        AD0HSUBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0HCONV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0hconv](index.html) module"]
pub struct ADC0HCONV_SPEC;
impl crate::RegisterSpec for ADC0HCONV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0hconv::R](R) reader structure"]
impl crate::Readable for ADC0HCONV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0hconv::W](W) writer structure"]
impl crate::Writable for ADC0HCONV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0HCONV to value 0"]
impl crate::Resettable for ADC0HCONV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
