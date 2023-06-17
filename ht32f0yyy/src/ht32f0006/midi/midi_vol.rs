#[doc = "Register `MIDI_VOL` reader"]
pub struct R(crate::R<MIDI_VOL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_VOL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_VOL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_VOL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_VOL` writer"]
pub struct W(crate::W<MIDI_VOL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_VOL_SPEC>;
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
impl From<crate::W<MIDI_VOL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_VOL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VR` reader - VR"]
pub type VR_R = crate::FieldReader<u16>;
#[doc = "Field `VR` writer - VR"]
pub type VR_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_VOL_SPEC, 10, O, u16>;
#[doc = "Field `VL` reader - VL"]
pub type VL_R = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VL"]
pub type VL_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_VOL_SPEC, 10, O, u16>;
#[doc = "Field `ENV` reader - ENV"]
pub type ENV_R = crate::FieldReader;
#[doc = "Field `ENV` writer - ENV"]
pub type ENV_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_VOL_SPEC, 2, O>;
#[doc = "Field `A_R` reader - A_R"]
pub type A_R_R = crate::BitReader;
#[doc = "Field `A_R` writer - A_R"]
pub type A_R_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_VOL_SPEC, O>;
impl R {
    #[doc = "Bits 0:9 - VR"]
    #[inline(always)]
    pub fn vr(&self) -> VR_R {
        VR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - VL"]
    #[inline(always)]
    pub fn vl(&self) -> VL_R {
        VL_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - ENV"]
    #[inline(always)]
    pub fn env(&self) -> ENV_R {
        ENV_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - A_R"]
    #[inline(always)]
    pub fn a_r(&self) -> A_R_R {
        A_R_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - VR"]
    #[inline(always)]
    #[must_use]
    pub fn vr(&mut self) -> VR_W<0> {
        VR_W::new(self)
    }
    #[doc = "Bits 16:25 - VL"]
    #[inline(always)]
    #[must_use]
    pub fn vl(&mut self) -> VL_W<16> {
        VL_W::new(self)
    }
    #[doc = "Bits 29:30 - ENV"]
    #[inline(always)]
    #[must_use]
    pub fn env(&mut self) -> ENV_W<29> {
        ENV_W::new(self)
    }
    #[doc = "Bit 31 - A_R"]
    #[inline(always)]
    #[must_use]
    pub fn a_r(&mut self) -> A_R_W<31> {
        A_R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_VOL\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_vol](index.html) module"]
pub struct MIDI_VOL_SPEC;
impl crate::RegisterSpec for MIDI_VOL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_vol::R](R) reader structure"]
impl crate::Readable for MIDI_VOL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_vol::W](W) writer structure"]
impl crate::Writable for MIDI_VOL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_VOL to value 0"]
impl crate::Resettable for MIDI_VOL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
