#[doc = "Register `MCTM_REPR` reader"]
pub struct R(crate::R<MCTM_REPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTM_REPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTM_REPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTM_REPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTM_REPR` writer"]
pub struct W(crate::W<MCTM_REPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTM_REPR_SPEC>;
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
impl From<crate::W<MCTM_REPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTM_REPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REPV` reader - REPV"]
pub type REPV_R = crate::FieldReader;
#[doc = "Field `REPV` writer - REPV"]
pub type REPV_W<'a, const O: u8> = crate::FieldWriter<'a, MCTM_REPR_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - REPV"]
    #[inline(always)]
    pub fn repv(&self) -> REPV_R {
        REPV_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REPV"]
    #[inline(always)]
    #[must_use]
    pub fn repv(&mut self) -> REPV_W<0> {
        REPV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCTM_REPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctm_repr](index.html) module"]
pub struct MCTM_REPR_SPEC;
impl crate::RegisterSpec for MCTM_REPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mctm_repr::R](R) reader structure"]
impl crate::Readable for MCTM_REPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctm_repr::W](W) writer structure"]
impl crate::Writable for MCTM_REPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTM_REPR to value 0"]
impl crate::Resettable for MCTM_REPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
