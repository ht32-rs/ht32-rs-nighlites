#[doc = "Register `MIDI_MCU_CH2` reader"]
pub struct R(crate::R<MIDI_MCU_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_MCU_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_MCU_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_MCU_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_MCU_CH2` writer"]
pub struct W(crate::W<MIDI_MCU_CH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_MCU_CH2_SPEC>;
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
impl From<crate::W<MIDI_MCU_CH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_MCU_CH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH2A` reader - CH2A"]
pub type CH2A_R = crate::FieldReader<u16>;
#[doc = "Field `CH2A` writer - CH2A"]
pub type CH2A_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_MCU_CH2_SPEC, 16, O, u16>;
#[doc = "Field `CH2B` reader - CH2B"]
pub type CH2B_R = crate::FieldReader<u16>;
#[doc = "Field `CH2B` writer - CH2B"]
pub type CH2B_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_MCU_CH2_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CH2A"]
    #[inline(always)]
    pub fn ch2a(&self) -> CH2A_R {
        CH2A_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CH2B"]
    #[inline(always)]
    pub fn ch2b(&self) -> CH2B_R {
        CH2B_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH2A"]
    #[inline(always)]
    #[must_use]
    pub fn ch2a(&mut self) -> CH2A_W<0> {
        CH2A_W::new(self)
    }
    #[doc = "Bits 16:31 - CH2B"]
    #[inline(always)]
    #[must_use]
    pub fn ch2b(&mut self) -> CH2B_W<16> {
        CH2B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_MCU_CH2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_mcu_ch2](index.html) module"]
pub struct MIDI_MCU_CH2_SPEC;
impl crate::RegisterSpec for MIDI_MCU_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_mcu_ch2::R](R) reader structure"]
impl crate::Readable for MIDI_MCU_CH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_mcu_ch2::W](W) writer structure"]
impl crate::Writable for MIDI_MCU_CH2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_MCU_CH2 to value 0"]
impl crate::Resettable for MIDI_MCU_CH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
