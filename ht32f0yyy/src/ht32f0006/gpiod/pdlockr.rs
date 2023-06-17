#[doc = "Register `PDLOCKR` reader"]
pub struct R(crate::R<PDLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDLOCKR` writer"]
pub struct W(crate::W<PDLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDLOCKR_SPEC>;
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
impl From<crate::W<PDLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDLOCK` reader - PDLOCK"]
pub type PDLOCK_R = crate::FieldReader;
#[doc = "Field `PDLOCK` writer - PDLOCK"]
pub type PDLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, PDLOCKR_SPEC, 4, O>;
#[doc = "Field `PDLKEY` reader - PDLKEY"]
pub type PDLKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PDLKEY` writer - PDLKEY"]
pub type PDLKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PDLOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:3 - PDLOCK"]
    #[inline(always)]
    pub fn pdlock(&self) -> PDLOCK_R {
        PDLOCK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - PDLKEY"]
    #[inline(always)]
    pub fn pdlkey(&self) -> PDLKEY_R {
        PDLKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn pdlock(&mut self) -> PDLOCK_W<0> {
        PDLOCK_W::new(self)
    }
    #[doc = "Bits 16:31 - PDLKEY"]
    #[inline(always)]
    #[must_use]
    pub fn pdlkey(&mut self) -> PDLKEY_W<16> {
        PDLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDLOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdlockr](index.html) module"]
pub struct PDLOCKR_SPEC;
impl crate::RegisterSpec for PDLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdlockr::R](R) reader structure"]
impl crate::Readable for PDLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdlockr::W](W) writer structure"]
impl crate::Writable for PDLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDLOCKR to value 0"]
impl crate::Resettable for PDLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
