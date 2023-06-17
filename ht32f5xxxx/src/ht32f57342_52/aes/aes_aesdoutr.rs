#[doc = "Register `AES_AESDOUTR` reader"]
pub struct R(crate::R<AES_AESDOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_AESDOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_AESDOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_AESDOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_AESDOUTR` writer"]
pub struct W(crate::W<AES_AESDOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_AESDOUTR_SPEC>;
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
impl From<crate::W<AES_AESDOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_AESDOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOUT` reader - DOUT"]
pub type DOUT_R = crate::FieldReader<u32>;
#[doc = "Field `DOUT` writer - DOUT"]
pub type DOUT_W<'a, const O: u8> = crate::FieldWriter<'a, AES_AESDOUTR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - DOUT"]
    #[inline(always)]
    pub fn dout(&self) -> DOUT_R {
        DOUT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DOUT"]
    #[inline(always)]
    #[must_use]
    pub fn dout(&mut self) -> DOUT_W<0> {
        DOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES_AESDOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_aesdoutr](index.html) module"]
pub struct AES_AESDOUTR_SPEC;
impl crate::RegisterSpec for AES_AESDOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_aesdoutr::R](R) reader structure"]
impl crate::Readable for AES_AESDOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_aesdoutr::W](W) writer structure"]
impl crate::Writable for AES_AESDOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_AESDOUTR to value 0"]
impl crate::Resettable for AES_AESDOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
