#[doc = "Register `CVRVALR0` reader"]
pub struct R(crate::R<CVRVALR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CVRVALR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CVRVALR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CVRVALR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CVRVALR0` writer"]
pub struct W(crate::W<CVRVALR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CVRVALR0_SPEC>;
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
impl From<crate::W<CVRVALR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CVRVALR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVRVAL` reader - CVRVAL"]
pub type CVRVAL_R = crate::FieldReader;
#[doc = "Field `CVRVAL` writer - CVRVAL"]
pub type CVRVAL_W<'a, const O: u8> = crate::FieldWriter<'a, CVRVALR0_SPEC, 6, O>;
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
#[doc = "CVRVALR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cvrvalr0](index.html) module"]
pub struct CVRVALR0_SPEC;
impl crate::RegisterSpec for CVRVALR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cvrvalr0::R](R) reader structure"]
impl crate::Readable for CVRVALR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cvrvalr0::W](W) writer structure"]
impl crate::Writable for CVRVALR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CVRVALR0 to value 0"]
impl crate::Resettable for CVRVALR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
