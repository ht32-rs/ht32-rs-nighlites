#[doc = "Register `ADC1HCONV` reader"]
pub struct R(crate::R<ADC1HCONV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC1HCONV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC1HCONV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC1HCONV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC1HCONV` writer"]
pub struct W(crate::W<ADC1HCONV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC1HCONV_SPEC>;
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
impl From<crate::W<ADC1HCONV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC1HCONV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD1HMODE` reader - AD1HMODE"]
pub type AD1HMODE_R = crate::FieldReader;
#[doc = "Field `AD1HMODE` writer - AD1HMODE"]
pub type AD1HMODE_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HCONV_SPEC, 2, O>;
#[doc = "Field `AD1HSEQL` reader - AD1HSEQL"]
pub type AD1HSEQL_R = crate::FieldReader;
#[doc = "Field `AD1HSEQL` writer - AD1HSEQL"]
pub type AD1HSEQL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HCONV_SPEC, 2, O>;
#[doc = "Field `AD1HSUBL` reader - AD1HSUBL"]
pub type AD1HSUBL_R = crate::FieldReader;
#[doc = "Field `AD1HSUBL` writer - AD1HSUBL"]
pub type AD1HSUBL_W<'a, const O: u8> = crate::FieldWriter<'a, ADC1HCONV_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - AD1HMODE"]
    #[inline(always)]
    pub fn ad1hmode(&self) -> AD1HMODE_R {
        AD1HMODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - AD1HSEQL"]
    #[inline(always)]
    pub fn ad1hseql(&self) -> AD1HSEQL_R {
        AD1HSEQL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - AD1HSUBL"]
    #[inline(always)]
    pub fn ad1hsubl(&self) -> AD1HSUBL_R {
        AD1HSUBL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AD1HMODE"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hmode(&mut self) -> AD1HMODE_W<0> {
        AD1HMODE_W::new(self)
    }
    #[doc = "Bits 8:9 - AD1HSEQL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hseql(&mut self) -> AD1HSEQL_W<8> {
        AD1HSEQL_W::new(self)
    }
    #[doc = "Bits 16:17 - AD1HSUBL"]
    #[inline(always)]
    #[must_use]
    pub fn ad1hsubl(&mut self) -> AD1HSUBL_W<16> {
        AD1HSUBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC1HCONV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1hconv](index.html) module"]
pub struct ADC1HCONV_SPEC;
impl crate::RegisterSpec for ADC1HCONV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc1hconv::R](R) reader structure"]
impl crate::Readable for ADC1HCONV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc1hconv::W](W) writer structure"]
impl crate::Writable for ADC1HCONV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC1HCONV to value 0"]
impl crate::Resettable for ADC1HCONV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
