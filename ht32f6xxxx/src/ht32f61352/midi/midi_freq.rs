#[doc = "Register `MIDI_FREQ` reader"]
pub struct R(crate::R<MIDI_FREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_FREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_FREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_FREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_FREQ` writer"]
pub struct W(crate::W<MIDI_FREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_FREQ_SPEC>;
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
impl From<crate::W<MIDI_FREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_FREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FR` reader - FR"]
pub type FR_R = crate::FieldReader<u16>;
#[doc = "Field `FR` writer - FR"]
pub type FR_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_FREQ_SPEC, 12, O, u16>;
#[doc = "Field `BL` reader - BL"]
pub type BL_R = crate::FieldReader;
#[doc = "Field `BL` writer - BL"]
pub type BL_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_FREQ_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:11 - FR"]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - BL"]
    #[inline(always)]
    pub fn bl(&self) -> BL_R {
        BL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - FR"]
    #[inline(always)]
    #[must_use]
    pub fn fr(&mut self) -> FR_W<0> {
        FR_W::new(self)
    }
    #[doc = "Bits 12:15 - BL"]
    #[inline(always)]
    #[must_use]
    pub fn bl(&mut self) -> BL_W<12> {
        BL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_FREQ\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_freq](index.html) module"]
pub struct MIDI_FREQ_SPEC;
impl crate::RegisterSpec for MIDI_FREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_freq::R](R) reader structure"]
impl crate::Readable for MIDI_FREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_freq::W](W) writer structure"]
impl crate::Writable for MIDI_FREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_FREQ to value 0"]
impl crate::Resettable for MIDI_FREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
