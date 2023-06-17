#[doc = "Register `I2S_FCR` reader"]
pub struct R(crate::R<I2S_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2S_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2S_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2S_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2S_FCR` writer"]
pub struct W(crate::W<I2S_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2S_FCR_SPEC>;
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
impl From<crate::W<I2S_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2S_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXTRI` reader - TXTRI"]
pub type TXTRI_R = crate::FieldReader;
#[doc = "Field `TXTRI` writer - TXTRI"]
pub type TXTRI_W<'a, const O: u8> = crate::FieldWriter<'a, I2S_FCR_SPEC, 4, O>;
#[doc = "Field `RXTRI` reader - RXTRI"]
pub type RXTRI_R = crate::FieldReader;
#[doc = "Field `RXTRI` writer - RXTRI"]
pub type RXTRI_W<'a, const O: u8> = crate::FieldWriter<'a, I2S_FCR_SPEC, 4, O>;
#[doc = "Field `TXFR` reader - TXFR"]
pub type TXFR_R = crate::BitReader;
#[doc = "Field `TXFR` writer - TXFR"]
pub type TXFR_W<'a, const O: u8> = crate::BitWriter<'a, I2S_FCR_SPEC, O>;
#[doc = "Field `RXFR` reader - RXFR"]
pub type RXFR_R = crate::BitReader;
#[doc = "Field `RXFR` writer - RXFR"]
pub type RXFR_W<'a, const O: u8> = crate::BitWriter<'a, I2S_FCR_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - TXTRI"]
    #[inline(always)]
    pub fn txtri(&self) -> TXTRI_R {
        TXTRI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXTRI"]
    #[inline(always)]
    pub fn rxtri(&self) -> RXTRI_R {
        RXTRI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TXFR"]
    #[inline(always)]
    pub fn txfr(&self) -> TXFR_R {
        TXFR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXFR"]
    #[inline(always)]
    pub fn rxfr(&self) -> RXFR_R {
        RXFR_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXTRI"]
    #[inline(always)]
    #[must_use]
    pub fn txtri(&mut self) -> TXTRI_W<0> {
        TXTRI_W::new(self)
    }
    #[doc = "Bits 4:7 - RXTRI"]
    #[inline(always)]
    #[must_use]
    pub fn rxtri(&mut self) -> RXTRI_W<4> {
        RXTRI_W::new(self)
    }
    #[doc = "Bit 8 - TXFR"]
    #[inline(always)]
    #[must_use]
    pub fn txfr(&mut self) -> TXFR_W<8> {
        TXFR_W::new(self)
    }
    #[doc = "Bit 9 - RXFR"]
    #[inline(always)]
    #[must_use]
    pub fn rxfr(&mut self) -> RXFR_W<9> {
        RXFR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2s_fcr](index.html) module"]
pub struct I2S_FCR_SPEC;
impl crate::RegisterSpec for I2S_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2s_fcr::R](R) reader structure"]
impl crate::Readable for I2S_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2s_fcr::W](W) writer structure"]
impl crate::Writable for I2S_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2S_FCR to value 0"]
impl crate::Resettable for I2S_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
