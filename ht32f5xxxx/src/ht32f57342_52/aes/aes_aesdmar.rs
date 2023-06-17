#[doc = "Register `AES_AESDMAR` reader"]
pub struct R(crate::R<AES_AESDMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AES_AESDMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AES_AESDMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AES_AESDMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AES_AESDMAR` writer"]
pub struct W(crate::W<AES_AESDMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AES_AESDMAR_SPEC>;
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
impl From<crate::W<AES_AESDMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AES_AESDMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IFDMAEN` reader - IFDMAEN"]
pub type IFDMAEN_R = crate::BitReader;
#[doc = "Field `IFDMAEN` writer - IFDMAEN"]
pub type IFDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, AES_AESDMAR_SPEC, O>;
#[doc = "Field `OFDMAEN` reader - OFDMAEN"]
pub type OFDMAEN_R = crate::BitReader;
#[doc = "Field `OFDMAEN` writer - OFDMAEN"]
pub type OFDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, AES_AESDMAR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - IFDMAEN"]
    #[inline(always)]
    pub fn ifdmaen(&self) -> IFDMAEN_R {
        IFDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OFDMAEN"]
    #[inline(always)]
    pub fn ofdmaen(&self) -> OFDMAEN_R {
        OFDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IFDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn ifdmaen(&mut self) -> IFDMAEN_W<0> {
        IFDMAEN_W::new(self)
    }
    #[doc = "Bit 1 - OFDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn ofdmaen(&mut self) -> OFDMAEN_W<1> {
        OFDMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES_AESDMAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aes_aesdmar](index.html) module"]
pub struct AES_AESDMAR_SPEC;
impl crate::RegisterSpec for AES_AESDMAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aes_aesdmar::R](R) reader structure"]
impl crate::Readable for AES_AESDMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aes_aesdmar::W](W) writer structure"]
impl crate::Writable for AES_AESDMAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AES_AESDMAR to value 0"]
impl crate::Resettable for AES_AESDMAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
