#[doc = "Register `I2SFCR` reader"]
pub struct R(crate::R<I2SFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SFCR` writer"]
pub struct W(crate::W<I2SFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SFCR_SPEC>;
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
impl From<crate::W<I2SFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFTLS` reader - TXFTLS"]
pub type TXFTLS_R = crate::FieldReader;
#[doc = "Field `TXFTLS` writer - TXFTLS"]
pub type TXFTLS_W<'a, const O: u8> = crate::FieldWriter<'a, I2SFCR_SPEC, 4, O>;
#[doc = "Field `RXFTLS` reader - RXFTLS"]
pub type RXFTLS_R = crate::FieldReader;
#[doc = "Field `RXFTLS` writer - RXFTLS"]
pub type RXFTLS_W<'a, const O: u8> = crate::FieldWriter<'a, I2SFCR_SPEC, 4, O>;
#[doc = "Field `TXFRST` reader - TXFRST"]
pub type TXFRST_R = crate::BitReader;
#[doc = "Field `TXFRST` writer - TXFRST"]
pub type TXFRST_W<'a, const O: u8> = crate::BitWriter<'a, I2SFCR_SPEC, O>;
#[doc = "Field `RXFRST` reader - RXFRST"]
pub type RXFRST_R = crate::BitReader;
#[doc = "Field `RXFRST` writer - RXFRST"]
pub type RXFRST_W<'a, const O: u8> = crate::BitWriter<'a, I2SFCR_SPEC, O>;
impl R {
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    pub fn txftls(&self) -> TXFTLS_R {
        TXFTLS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    pub fn rxftls(&self) -> RXFTLS_R {
        RXFTLS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - TXFRST"]
    #[inline(always)]
    pub fn txfrst(&self) -> TXFRST_R {
        TXFRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RXFRST"]
    #[inline(always)]
    pub fn rxfrst(&self) -> RXFRST_R {
        RXFRST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - TXFTLS"]
    #[inline(always)]
    #[must_use]
    pub fn txftls(&mut self) -> TXFTLS_W<0> {
        TXFTLS_W::new(self)
    }
    #[doc = "Bits 4:7 - RXFTLS"]
    #[inline(always)]
    #[must_use]
    pub fn rxftls(&mut self) -> RXFTLS_W<4> {
        RXFTLS_W::new(self)
    }
    #[doc = "Bit 8 - TXFRST"]
    #[inline(always)]
    #[must_use]
    pub fn txfrst(&mut self) -> TXFRST_W<8> {
        TXFRST_W::new(self)
    }
    #[doc = "Bit 9 - RXFRST"]
    #[inline(always)]
    #[must_use]
    pub fn rxfrst(&mut self) -> RXFRST_W<9> {
        RXFRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2SFCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2sfcr](index.html) module"]
pub struct I2SFCR_SPEC;
impl crate::RegisterSpec for I2SFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2sfcr::R](R) reader structure"]
impl crate::Readable for I2SFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2sfcr::W](W) writer structure"]
impl crate::Writable for I2SFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SFCR to value 0"]
impl crate::Resettable for I2SFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
