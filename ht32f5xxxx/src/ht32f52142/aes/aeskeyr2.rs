#[doc = "Register `AESKEYR2` reader"]
pub struct R(crate::R<AESKEYR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESKEYR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESKEYR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESKEYR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESKEYR2` writer"]
pub struct W(crate::W<AESKEYR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESKEYR2_SPEC>;
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
impl From<crate::W<AESKEYR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESKEYR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KeyDat` reader - KeyDat"]
pub type KEY_DAT_R = crate::FieldReader<u32>;
#[doc = "Field `KeyDat` writer - KeyDat"]
pub type KEY_DAT_W<'a, const O: u8> = crate::FieldWriter<'a, AESKEYR2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - KeyDat"]
    #[inline(always)]
    pub fn key_dat(&self) -> KEY_DAT_R {
        KEY_DAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - KeyDat"]
    #[inline(always)]
    #[must_use]
    pub fn key_dat(&mut self) -> KEY_DAT_W<0> {
        KEY_DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AESKEYR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeskeyr2](index.html) module"]
pub struct AESKEYR2_SPEC;
impl crate::RegisterSpec for AESKEYR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aeskeyr2::R](R) reader structure"]
impl crate::Readable for AESKEYR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aeskeyr2::W](W) writer structure"]
impl crate::Writable for AESKEYR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESKEYR2 to value 0"]
impl crate::Resettable for AESKEYR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
