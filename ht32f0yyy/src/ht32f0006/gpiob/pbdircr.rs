#[doc = "Register `PBDIRCR` reader"]
pub struct R(crate::R<PBDIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDIRCR` writer"]
pub struct W(crate::W<PBDIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDIRCR_SPEC>;
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
impl From<crate::W<PBDIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBDIR` reader - PBDIR"]
pub type PBDIR_R = crate::FieldReader<u16>;
#[doc = "Field `PBDIR` writer - PBDIR"]
pub type PBDIR_W<'a, const O: u8> = crate::FieldWriter<'a, PBDIRCR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PBDIR"]
    #[inline(always)]
    pub fn pbdir(&self) -> PBDIR_R {
        PBDIR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PBDIR"]
    #[inline(always)]
    #[must_use]
    pub fn pbdir(&mut self) -> PBDIR_W<0> {
        PBDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBDIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdircr](index.html) module"]
pub struct PBDIRCR_SPEC;
impl crate::RegisterSpec for PBDIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbdircr::R](R) reader structure"]
impl crate::Readable for PBDIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdircr::W](W) writer structure"]
impl crate::Writable for PBDIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBDIRCR to value 0"]
impl crate::Resettable for PBDIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
