#[doc = "Register `PWRCU_HSIRCR` reader"]
pub struct R(crate::R<PWRCU_HSIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCU_HSIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCU_HSIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCU_HSIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCU_HSIRCR` writer"]
pub struct W(crate::W<PWRCU_HSIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCU_HSIRCR_SPEC>;
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
impl From<crate::W<PWRCU_HSIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCU_HSIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HSIRCBL` reader - HSIRCBL"]
pub type HSIRCBL_R = crate::FieldReader;
#[doc = "Field `HSIRCBL` writer - HSIRCBL"]
pub type HSIRCBL_W<'a, const O: u8> = crate::FieldWriter<'a, PWRCU_HSIRCR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - HSIRCBL"]
    #[inline(always)]
    pub fn hsircbl(&self) -> HSIRCBL_R {
        HSIRCBL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - HSIRCBL"]
    #[inline(always)]
    #[must_use]
    pub fn hsircbl(&mut self) -> HSIRCBL_W<0> {
        HSIRCBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWRCU_HSIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcu_hsircr](index.html) module"]
pub struct PWRCU_HSIRCR_SPEC;
impl crate::RegisterSpec for PWRCU_HSIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcu_hsircr::R](R) reader structure"]
impl crate::Readable for PWRCU_HSIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcu_hsircr::W](W) writer structure"]
impl crate::Writable for PWRCU_HSIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRCU_HSIRCR to value 0"]
impl crate::Resettable for PWRCU_HSIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
