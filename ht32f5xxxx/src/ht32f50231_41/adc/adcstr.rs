#[doc = "Register `ADCSTR` reader"]
pub struct R(crate::R<ADCSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCSTR` writer"]
pub struct W(crate::W<ADCSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCSTR_SPEC>;
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
impl From<crate::W<ADCSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADST` reader - ADST"]
pub type ADST_R = crate::FieldReader;
#[doc = "Field `ADST` writer - ADST"]
pub type ADST_W<'a, const O: u8> = crate::FieldWriter<'a, ADCSTR_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - ADST"]
    #[inline(always)]
    pub fn adst(&self) -> ADST_R {
        ADST_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST"]
    #[inline(always)]
    #[must_use]
    pub fn adst(&mut self) -> ADST_W<0> {
        ADST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADCSTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcstr](index.html) module"]
pub struct ADCSTR_SPEC;
impl crate::RegisterSpec for ADCSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcstr::R](R) reader structure"]
impl crate::Readable for ADCSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcstr::W](W) writer structure"]
impl crate::Writable for ADCSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCSTR to value 0"]
impl crate::Resettable for ADCSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
