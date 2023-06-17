#[doc = "Register `MIDI_R` reader"]
pub struct R(crate::R<MIDI_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_R` writer"]
pub struct W(crate::W<MIDI_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_R_SPEC>;
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
impl From<crate::W<MIDI_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIDIR` reader - MIDIR"]
pub type MIDIR_R = crate::FieldReader<u32>;
#[doc = "Field `MIDIR` writer - MIDIR"]
pub type MIDIR_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_R_SPEC, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - MIDIR"]
    #[inline(always)]
    pub fn midir(&self) -> MIDIR_R {
        MIDIR_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - MIDIR"]
    #[inline(always)]
    #[must_use]
    pub fn midir(&mut self) -> MIDIR_W<0> {
        MIDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_R\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_r](index.html) module"]
pub struct MIDI_R_SPEC;
impl crate::RegisterSpec for MIDI_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_r::R](R) reader structure"]
impl crate::Readable for MIDI_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_r::W](W) writer structure"]
impl crate::Writable for MIDI_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_R to value 0"]
impl crate::Resettable for MIDI_R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
