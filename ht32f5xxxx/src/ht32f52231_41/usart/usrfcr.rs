#[doc = "Register `USRFCR` reader"]
pub struct R(crate::R<USRFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USRFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USRFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USRFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USRFCR` writer"]
pub struct W(crate::W<USRFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USRFCR_SPEC>;
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
impl From<crate::W<USRFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USRFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXR` reader - TXR"]
pub type TXR_R = crate::BitReader;
#[doc = "Field `TXR` writer - TXR"]
pub type TXR_W<'a, const O: u8> = crate::BitWriter<'a, USRFCR_SPEC, O>;
#[doc = "Field `RXR` reader - RXR"]
pub type RXR_R = crate::BitReader;
#[doc = "Field `RXR` writer - RXR"]
pub type RXR_W<'a, const O: u8> = crate::BitWriter<'a, USRFCR_SPEC, O>;
#[doc = "Field `TXTL` reader - TXTL"]
pub type TXTL_R = crate::FieldReader;
#[doc = "Field `TXTL` writer - TXTL"]
pub type TXTL_W<'a, const O: u8> = crate::FieldWriter<'a, USRFCR_SPEC, 2, O>;
#[doc = "Field `RXTL` reader - RXTL"]
pub type RXTL_R = crate::FieldReader;
#[doc = "Field `RXTL` writer - RXTL"]
pub type RXTL_W<'a, const O: u8> = crate::FieldWriter<'a, USRFCR_SPEC, 2, O>;
#[doc = "Field `TXFS` reader - TXFS"]
pub type TXFS_R = crate::FieldReader;
#[doc = "Field `TXFS` writer - TXFS"]
pub type TXFS_W<'a, const O: u8> = crate::FieldWriter<'a, USRFCR_SPEC, 4, O>;
#[doc = "Field `RXFS` reader - RXFS"]
pub type RXFS_R = crate::FieldReader;
#[doc = "Field `RXFS` writer - RXFS"]
pub type RXFS_W<'a, const O: u8> = crate::FieldWriter<'a, USRFCR_SPEC, 4, O>;
impl R {
    #[doc = "Bit 0 - TXR"]
    #[inline(always)]
    pub fn txr(&self) -> TXR_R {
        TXR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXR"]
    #[inline(always)]
    pub fn rxr(&self) -> RXR_R {
        RXR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - TXTL"]
    #[inline(always)]
    pub fn txtl(&self) -> TXTL_R {
        TXTL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RXTL"]
    #[inline(always)]
    pub fn rxtl(&self) -> RXTL_R {
        RXTL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:19 - TXFS"]
    #[inline(always)]
    pub fn txfs(&self) -> TXFS_R {
        TXFS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RXFS"]
    #[inline(always)]
    pub fn rxfs(&self) -> RXFS_R {
        RXFS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TXR"]
    #[inline(always)]
    #[must_use]
    pub fn txr(&mut self) -> TXR_W<0> {
        TXR_W::new(self)
    }
    #[doc = "Bit 1 - RXR"]
    #[inline(always)]
    #[must_use]
    pub fn rxr(&mut self) -> RXR_W<1> {
        RXR_W::new(self)
    }
    #[doc = "Bits 4:5 - TXTL"]
    #[inline(always)]
    #[must_use]
    pub fn txtl(&mut self) -> TXTL_W<4> {
        TXTL_W::new(self)
    }
    #[doc = "Bits 6:7 - RXTL"]
    #[inline(always)]
    #[must_use]
    pub fn rxtl(&mut self) -> RXTL_W<6> {
        RXTL_W::new(self)
    }
    #[doc = "Bits 16:19 - TXFS"]
    #[inline(always)]
    #[must_use]
    pub fn txfs(&mut self) -> TXFS_W<16> {
        TXFS_W::new(self)
    }
    #[doc = "Bits 24:27 - RXFS"]
    #[inline(always)]
    #[must_use]
    pub fn rxfs(&mut self) -> RXFS_W<24> {
        RXFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USRFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usrfcr](index.html) module"]
pub struct USRFCR_SPEC;
impl crate::RegisterSpec for USRFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usrfcr::R](R) reader structure"]
impl crate::Readable for USRFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usrfcr::W](W) writer structure"]
impl crate::Writable for USRFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USRFCR to value 0"]
impl crate::Resettable for USRFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
