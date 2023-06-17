#[doc = "Register `PCDIRCR` reader"]
pub struct R(crate::R<PCDIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDIRCR` writer"]
pub struct W(crate::W<PCDIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDIRCR_SPEC>;
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
impl From<crate::W<PCDIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCDIR` reader - PCDIR"]
pub type PCDIR_R = crate::FieldReader<u16>;
#[doc = "Field `PCDIR` writer - PCDIR"]
pub type PCDIR_W<'a, const O: u8> = crate::FieldWriter<'a, PCDIRCR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCDIR"]
    #[inline(always)]
    pub fn pcdir(&self) -> PCDIR_R {
        PCDIR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCDIR"]
    #[inline(always)]
    #[must_use]
    pub fn pcdir(&mut self) -> PCDIR_W<0> {
        PCDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCDIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdircr](index.html) module"]
pub struct PCDIRCR_SPEC;
impl crate::RegisterSpec for PCDIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcdircr::R](R) reader structure"]
impl crate::Readable for PCDIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdircr::W](W) writer structure"]
impl crate::Writable for PCDIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDIRCR to value 0"]
impl crate::Resettable for PCDIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
