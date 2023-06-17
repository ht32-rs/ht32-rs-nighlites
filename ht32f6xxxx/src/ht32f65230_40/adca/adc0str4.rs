#[doc = "Register `ADC0STR4` reader"]
pub struct R(crate::R<ADC0STR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0STR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0STR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0STR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC0STR4` writer"]
pub struct W(crate::W<ADC0STR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC0STR4_SPEC>;
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
impl From<crate::W<ADC0STR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC0STR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD0ST` reader - AD0ST"]
pub type AD0ST_R = crate::FieldReader;
#[doc = "Field `AD0ST` writer - AD0ST"]
pub type AD0ST_W<'a, const O: u8> = crate::FieldWriter<'a, ADC0STR4_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - AD0ST"]
    #[inline(always)]
    pub fn ad0st(&self) -> AD0ST_R {
        AD0ST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - AD0ST"]
    #[inline(always)]
    #[must_use]
    pub fn ad0st(&mut self) -> AD0ST_W<0> {
        AD0ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC0STR4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0str4](index.html) module"]
pub struct ADC0STR4_SPEC;
impl crate::RegisterSpec for ADC0STR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0str4::R](R) reader structure"]
impl crate::Readable for ADC0STR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc0str4::W](W) writer structure"]
impl crate::Writable for ADC0STR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADC0STR4 to value 0"]
impl crate::Resettable for ADC0STR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
