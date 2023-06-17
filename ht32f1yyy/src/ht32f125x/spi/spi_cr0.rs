#[doc = "Register `SPI_CR0` reader"]
pub struct R(crate::R<SPI_CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_CR0` writer"]
pub struct W(crate::W<SPI_CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CR0_SPEC>;
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
impl From<crate::W<SPI_CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIEN` reader - SPIEN"]
pub type SPIEN_R = crate::BitReader;
#[doc = "Field `SPIEN` writer - SPIEN"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CR0_SPEC, O>;
#[doc = "Field `SELOEN` reader - SELOEN"]
pub type SELOEN_R = crate::BitReader;
#[doc = "Field `SELOEN` writer - SELOEN"]
pub type SELOEN_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CR0_SPEC, O>;
#[doc = "Field `SSELC` reader - SSELC"]
pub type SSELC_R = crate::BitReader;
#[doc = "Field `SSELC` writer - SSELC"]
pub type SSELC_W<'a, const O: u8> = crate::BitWriter<'a, SPI_CR0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    pub fn spien(&self) -> SPIEN_R {
        SPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    pub fn seloen(&self) -> SELOEN_R {
        SELOEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    pub fn sselc(&self) -> SSELC_R {
        SSELC_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPIEN"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<0> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 3 - SELOEN"]
    #[inline(always)]
    #[must_use]
    pub fn seloen(&mut self) -> SELOEN_W<3> {
        SELOEN_W::new(self)
    }
    #[doc = "Bit 4 - SSELC"]
    #[inline(always)]
    #[must_use]
    pub fn sselc(&mut self) -> SSELC_W<4> {
        SSELC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI_CR0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_cr0](index.html) module"]
pub struct SPI_CR0_SPEC;
impl crate::RegisterSpec for SPI_CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_cr0::R](R) reader structure"]
impl crate::Readable for SPI_CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_cr0::W](W) writer structure"]
impl crate::Writable for SPI_CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_CR0 to value 0"]
impl crate::Resettable for SPI_CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
