#[doc = "Register `AES_AESDINR` reader"]
pub struct R(crate::R<AES_AESDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_AESDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_AESDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_AESDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_AESDINR` writer"]
pub struct W(crate::W<AES_AESDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_AESDINR_SPEC>;
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
impl From<crate::W<AES_AESDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_AESDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIN` reader - DIN"]
pub type DIN_R = crate::FieldReader<u32>;
#[doc = "Field `DIN` writer - DIN"]
pub type DIN_W<'a, const O: u8> = crate::FieldWriter<'a, AES_AESDINR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DIN"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DIN"]
    #[inline(always)]
    #[must_use]
    pub fn din(&mut self) -> DIN_W<0> {
        DIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES_AESDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_aesdinr](index.html) module"]
pub struct AES_AESDINR_SPEC;
impl crate::RegisterSpec for AES_AESDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_aesdinr::R](R) reader structure"]
impl crate::Readable for AES_AESDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_aesdinr::W](W) writer structure"]
impl crate::Writable for AES_AESDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_AESDINR to value 0"]
impl crate::Resettable for AES_AESDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
