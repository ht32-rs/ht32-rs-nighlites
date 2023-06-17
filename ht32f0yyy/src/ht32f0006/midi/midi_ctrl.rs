#[doc = "Register `MIDI_CTRL` reader"]
pub struct R(crate::R<MIDI_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_CTRL` writer"]
pub struct W(crate::W<MIDI_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_CTRL_SPEC>;
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
impl From<crate::W<MIDI_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHS` reader - CHS"]
pub type CHS_R = crate::FieldReader;
#[doc = "Field `CHS` writer - CHS"]
pub type CHS_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_CTRL_SPEC, 3, O>;
#[doc = "Field `SPI_DISLOOP` reader - SPI_DISLOOP"]
pub type SPI_DISLOOP_R = crate::BitReader;
#[doc = "Field `SPI_DISLOOP` writer - SPI_DISLOOP"]
pub type SPI_DISLOOP_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_CTRL_SPEC, O>;
#[doc = "Field `SPI_RDEN` reader - SPI_RDEN"]
pub type SPI_RDEN_R = crate::BitReader;
#[doc = "Field `SPI_RDEN` writer - SPI_RDEN"]
pub type SPI_RDEN_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_CTRL_SPEC, O>;
#[doc = "Field `MUSIC_EN` reader - MUSIC_EN"]
pub type MUSIC_EN_R = crate::BitReader;
#[doc = "Field `MUSIC_EN` writer - MUSIC_EN"]
pub type MUSIC_EN_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_CTRL_SPEC, O>;
#[doc = "Field `DACDS` reader - DACDS"]
pub type DACDS_R = crate::FieldReader;
#[doc = "Field `DACDS` writer - DACDS"]
pub type DACDS_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_CTRL_SPEC, 3, O>;
#[doc = "Field `MCUCHEN` reader - MCUCHEN"]
pub type MCUCHEN_R = crate::FieldReader;
#[doc = "Field `MCUCHEN` writer - MCUCHEN"]
pub type MCUCHEN_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_CTRL_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:2 - CHS"]
    #[inline(always)]
    pub fn chs(&self) -> CHS_R {
        CHS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - SPI_DISLOOP"]
    #[inline(always)]
    pub fn spi_disloop(&self) -> SPI_DISLOOP_R {
        SPI_DISLOOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI_RDEN"]
    #[inline(always)]
    pub fn spi_rden(&self) -> SPI_RDEN_R {
        SPI_RDEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MUSIC_EN"]
    #[inline(always)]
    pub fn music_en(&self) -> MUSIC_EN_R {
        MUSIC_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - DACDS"]
    #[inline(always)]
    pub fn dacds(&self) -> DACDS_R {
        DACDS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - MCUCHEN"]
    #[inline(always)]
    pub fn mcuchen(&self) -> MCUCHEN_R {
        MCUCHEN_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - CHS"]
    #[inline(always)]
    #[must_use]
    pub fn chs(&mut self) -> CHS_W<0> {
        CHS_W::new(self)
    }
    #[doc = "Bit 5 - SPI_DISLOOP"]
    #[inline(always)]
    #[must_use]
    pub fn spi_disloop(&mut self) -> SPI_DISLOOP_W<5> {
        SPI_DISLOOP_W::new(self)
    }
    #[doc = "Bit 6 - SPI_RDEN"]
    #[inline(always)]
    #[must_use]
    pub fn spi_rden(&mut self) -> SPI_RDEN_W<6> {
        SPI_RDEN_W::new(self)
    }
    #[doc = "Bit 7 - MUSIC_EN"]
    #[inline(always)]
    #[must_use]
    pub fn music_en(&mut self) -> MUSIC_EN_W<7> {
        MUSIC_EN_W::new(self)
    }
    #[doc = "Bits 8:10 - DACDS"]
    #[inline(always)]
    #[must_use]
    pub fn dacds(&mut self) -> DACDS_W<8> {
        DACDS_W::new(self)
    }
    #[doc = "Bits 12:15 - MCUCHEN"]
    #[inline(always)]
    #[must_use]
    pub fn mcuchen(&mut self) -> MCUCHEN_W<12> {
        MCUCHEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_CTRL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_ctrl](index.html) module"]
pub struct MIDI_CTRL_SPEC;
impl crate::RegisterSpec for MIDI_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_ctrl::R](R) reader structure"]
impl crate::Readable for MIDI_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_ctrl::W](W) writer structure"]
impl crate::Writable for MIDI_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_CTRL to value 0"]
impl crate::Resettable for MIDI_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
