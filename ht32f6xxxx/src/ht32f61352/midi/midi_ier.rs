#[doc = "Register `MIDI_IER` reader"]
pub struct R(crate::R<MIDI_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_IER` writer"]
pub struct W(crate::W<MIDI_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_IER_SPEC>;
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
impl From<crate::W<MIDI_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTEN` reader - INTEN"]
pub type INTEN_R = crate::BitReader;
#[doc = "Field `INTEN` writer - INTEN"]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_IER_SPEC, O>;
#[doc = "Field `MIDII_DMAEN` reader - MIDII_DMAEN"]
pub type MIDII_DMAEN_R = crate::BitReader;
#[doc = "Field `MIDII_DMAEN` writer - MIDII_DMAEN"]
pub type MIDII_DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_IER_SPEC, O>;
#[doc = "Field `MIDIO_DMAEN` reader - MIDIO_DMAEN"]
pub type MIDIO_DMAEN_R = crate::BitReader;
#[doc = "Field `MIDIO_DMAEN` writer - MIDIO_DMAEN"]
pub type MIDIO_DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_IER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - INTEN"]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MIDII_DMAEN"]
    #[inline(always)]
    pub fn midii_dmaen(&self) -> MIDII_DMAEN_R {
        MIDII_DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MIDIO_DMAEN"]
    #[inline(always)]
    pub fn midio_dmaen(&self) -> MIDIO_DMAEN_R {
        MIDIO_DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INTEN"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<0> {
        INTEN_W::new(self)
    }
    #[doc = "Bit 1 - MIDII_DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn midii_dmaen(&mut self) -> MIDII_DMAEN_W<1> {
        MIDII_DMAEN_W::new(self)
    }
    #[doc = "Bit 2 - MIDIO_DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn midio_dmaen(&mut self) -> MIDIO_DMAEN_W<2> {
        MIDIO_DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_ier](index.html) module"]
pub struct MIDI_IER_SPEC;
impl crate::RegisterSpec for MIDI_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_ier::R](R) reader structure"]
impl crate::Readable for MIDI_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_ier::W](W) writer structure"]
impl crate::Writable for MIDI_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_IER to value 0"]
impl crate::Resettable for MIDI_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
