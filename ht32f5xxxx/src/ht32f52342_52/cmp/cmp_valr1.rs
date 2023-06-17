#[doc = "Register `CMP_VALR1` reader"]
pub struct R(crate::R<CMP_VALR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP_VALR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP_VALR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP_VALR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP_VALR1` writer"]
pub struct W(crate::W<CMP_VALR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP_VALR1_SPEC>;
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
impl From<crate::W<CMP_VALR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP_VALR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVRVAL` reader - CVRVAL"]
pub type CVRVAL_R = crate::FieldReader;
#[doc = "Field `CVRVAL` writer - CVRVAL"]
pub type CVRVAL_W<'a, const O: u8> = crate::FieldWriter<'a, CMP_VALR1_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - CVRVAL"]
    #[inline(always)]
    pub fn cvrval(&self) -> CVRVAL_R {
        CVRVAL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - CVRVAL"]
    #[inline(always)]
    #[must_use]
    pub fn cvrval(&mut self) -> CVRVAL_W<0> {
        CVRVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP_VALR1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp_valr1](index.html) module"]
pub struct CMP_VALR1_SPEC;
impl crate::RegisterSpec for CMP_VALR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp_valr1::R](R) reader structure"]
impl crate::Readable for CMP_VALR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp_valr1::W](W) writer structure"]
impl crate::Writable for CMP_VALR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMP_VALR1 to value 0"]
impl crate::Resettable for CMP_VALR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
