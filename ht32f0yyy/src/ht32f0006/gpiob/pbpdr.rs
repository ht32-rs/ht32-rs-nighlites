#[doc = "Register `PBPDR` reader"]
pub struct R(crate::R<PBPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBPDR` writer"]
pub struct W(crate::W<PBPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBPDR_SPEC>;
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
impl From<crate::W<PBPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBPD` reader - PBPD"]
pub type PBPD_R = crate::FieldReader<u16>;
#[doc = "Field `PBPD` writer - PBPD"]
pub type PBPD_W<'a, const O: u8> = crate::FieldWriter<'a, PBPDR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PBPD"]
    #[inline(always)]
    pub fn pbpd(&self) -> PBPD_R {
        PBPD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PBPD"]
    #[inline(always)]
    #[must_use]
    pub fn pbpd(&mut self) -> PBPD_W<0> {
        PBPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbpdr](index.html) module"]
pub struct PBPDR_SPEC;
impl crate::RegisterSpec for PBPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbpdr::R](R) reader structure"]
impl crate::Readable for PBPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbpdr::W](W) writer structure"]
impl crate::Writable for PBPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBPDR to value 0"]
impl crate::Resettable for PBPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
