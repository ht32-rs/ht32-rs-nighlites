#[doc = "Register `SPI_FCR` reader"]
pub struct R(crate::R<SPI_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_FCR` writer"]
pub struct W(crate::W<SPI_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_FCR_SPEC>;
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
impl From<crate::W<SPI_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXFTLS` reader - TXFTLS"]
pub type TXFTLS_R = crate::FieldReader;
#[doc = "Field `TXFTLS` writer - TXFTLS"]
pub type TXFTLS_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_FCR_SPEC, 4, O>;
#[doc = "Field `RXFTLS` reader - RXFTLS"]
pub type RXFTLS_R = crate::FieldReader;
#[doc = "Field `RXFTLS` writer - RXFTLS"]
pub type RXFTLS_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_FCR_SPEC, 4, O>;
#[doc = "Field `FIFOEN` reader - FIFOEN"]
pub type FIFOEN_R = crate::BitReader;
#[doc = "Field `FIFOEN` writer - FIFOEN"]
pub type FIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_FCR_SPEC, O>;
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
    #[doc = "Bit 10 - FIFOEN"]
    #[inline(always)]
    pub fn fifoen(&self) -> FIFOEN_R {
        FIFOEN_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 10 - FIFOEN"]
    #[inline(always)]
    #[must_use]
    pub fn fifoen(&mut self) -> FIFOEN_W<10> {
        FIFOEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fcr](index.html) module"]
pub struct SPI_FCR_SPEC;
impl crate::RegisterSpec for SPI_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_fcr::R](R) reader structure"]
impl crate::Readable for SPI_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_fcr::W](W) writer structure"]
impl crate::Writable for SPI_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_FCR to value 0"]
impl crate::Resettable for SPI_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
