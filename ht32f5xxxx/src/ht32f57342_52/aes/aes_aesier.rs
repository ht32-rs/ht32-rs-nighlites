#[doc = "Register `AES_AESIER` reader"]
pub struct R(crate::R<AES_AESIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_AESIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_AESIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_AESIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_AESIER` writer"]
pub struct W(crate::W<AES_AESIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_AESIER_SPEC>;
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
impl From<crate::W<AES_AESIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_AESIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFINTEN` reader - IFINTEN"]
pub type IFINTEN_R = crate::BitReader;
#[doc = "Field `IFINTEN` writer - IFINTEN"]
pub type IFINTEN_W<'a, const O: u8> = crate::BitWriter<'a, AES_AESIER_SPEC, O>;
#[doc = "Field `OFINTEN` reader - OFINTEN"]
pub type OFINTEN_R = crate::BitReader;
#[doc = "Field `OFINTEN` writer - OFINTEN"]
pub type OFINTEN_W<'a, const O: u8> = crate::BitWriter<'a, AES_AESIER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - IFINTEN"]
    #[inline(always)]
    pub fn ifinten(&self) -> IFINTEN_R {
        IFINTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OFINTEN"]
    #[inline(always)]
    pub fn ofinten(&self) -> OFINTEN_R {
        OFINTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IFINTEN"]
    #[inline(always)]
    #[must_use]
    pub fn ifinten(&mut self) -> IFINTEN_W<0> {
        IFINTEN_W::new(self)
    }
    #[doc = "Bit 1 - OFINTEN"]
    #[inline(always)]
    #[must_use]
    pub fn ofinten(&mut self) -> OFINTEN_W<1> {
        OFINTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES_AESIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_aesier](index.html) module"]
pub struct AES_AESIER_SPEC;
impl crate::RegisterSpec for AES_AESIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_aesier::R](R) reader structure"]
impl crate::Readable for AES_AESIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_aesier::W](W) writer structure"]
impl crate::Writable for AES_AESIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_AESIER to value 0"]
impl crate::Resettable for AES_AESIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
