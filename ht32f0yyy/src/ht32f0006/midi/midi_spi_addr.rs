#[doc = "Register `MIDI_SPI_ADDR` reader"]
pub struct R(crate::R<MIDI_SPI_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_SPI_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_SPI_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_SPI_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_SPI_ADDR` writer"]
pub struct W(crate::W<MIDI_SPI_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_SPI_ADDR_SPEC>;
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
impl From<crate::W<MIDI_SPI_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_SPI_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIA` reader - SPIA"]
pub type SPIA_R = crate::FieldReader<u32>;
#[doc = "Field `SPIA` writer - SPIA"]
pub type SPIA_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_SPI_ADDR_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - SPIA"]
    #[inline(always)]
    pub fn spia(&self) -> SPIA_R {
        SPIA_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - SPIA"]
    #[inline(always)]
    #[must_use]
    pub fn spia(&mut self) -> SPIA_W<0> {
        SPIA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_SPI_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_spi_addr](index.html) module"]
pub struct MIDI_SPI_ADDR_SPEC;
impl crate::RegisterSpec for MIDI_SPI_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_spi_addr::R](R) reader structure"]
impl crate::Readable for MIDI_SPI_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_spi_addr::W](W) writer structure"]
impl crate::Writable for MIDI_SPI_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_SPI_ADDR to value 0"]
impl crate::Resettable for MIDI_SPI_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
