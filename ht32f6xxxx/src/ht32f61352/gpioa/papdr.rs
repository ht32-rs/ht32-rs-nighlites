#[doc = "Register `PAPDR` reader"]
pub struct R(crate::R<PAPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAPDR` writer"]
pub struct W(crate::W<PAPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAPDR_SPEC>;
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
impl From<crate::W<PAPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAPD` reader - PAPD"]
pub type PAPD_R = crate::FieldReader<u16>;
#[doc = "Field `PAPD` writer - PAPD"]
pub type PAPD_W<'a, const O: u8> = crate::FieldWriter<'a, PAPDR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PAPD"]
    #[inline(always)]
    pub fn papd(&self) -> PAPD_R {
        PAPD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PAPD"]
    #[inline(always)]
    #[must_use]
    pub fn papd(&mut self) -> PAPD_W<0> {
        PAPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [papdr](index.html) module"]
pub struct PAPDR_SPEC;
impl crate::RegisterSpec for PAPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [papdr::R](R) reader structure"]
impl crate::Readable for PAPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [papdr::W](W) writer structure"]
impl crate::Writable for PAPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAPDR to value 0"]
impl crate::Resettable for PAPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
