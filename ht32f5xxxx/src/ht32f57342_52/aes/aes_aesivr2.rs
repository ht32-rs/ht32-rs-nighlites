#[doc = "Register `AES_AESIVR2` reader"]
pub struct R(crate::R<AES_AESIVR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_AESIVR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_AESIVR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_AESIVR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_AESIVR2` writer"]
pub struct W(crate::W<AES_AESIVR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_AESIVR2_SPEC>;
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
impl From<crate::W<AES_AESIVR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_AESIVR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IVData` reader - IVData"]
pub type IVDATA_R = crate::FieldReader<u32>;
#[doc = "Field `IVData` writer - IVData"]
pub type IVDATA_W<'a, const O: u8> = crate::FieldWriter<'a, AES_AESIVR2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - IVData"]
    #[inline(always)]
    pub fn ivdata(&self) -> IVDATA_R {
        IVDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - IVData"]
    #[inline(always)]
    #[must_use]
    pub fn ivdata(&mut self) -> IVDATA_W<0> {
        IVDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES_AESIVR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_aesivr2](index.html) module"]
pub struct AES_AESIVR2_SPEC;
impl crate::RegisterSpec for AES_AESIVR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_aesivr2::R](R) reader structure"]
impl crate::Readable for AES_AESIVR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_aesivr2::W](W) writer structure"]
impl crate::Writable for AES_AESIVR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_AESIVR2 to value 0"]
impl crate::Resettable for AES_AESIVR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
