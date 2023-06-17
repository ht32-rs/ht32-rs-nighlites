#[doc = "Register `MIDI_MCU_CH1` reader"]
pub struct R(crate::R<MIDI_MCU_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_MCU_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_MCU_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_MCU_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_MCU_CH1` writer"]
pub struct W(crate::W<MIDI_MCU_CH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_MCU_CH1_SPEC>;
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
impl From<crate::W<MIDI_MCU_CH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_MCU_CH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH1A` reader - CH1A"]
pub type CH1A_R = crate::FieldReader<u16>;
#[doc = "Field `CH1A` writer - CH1A"]
pub type CH1A_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_MCU_CH1_SPEC, 16, O, u16>;
#[doc = "Field `CH1B` reader - CH1B"]
pub type CH1B_R = crate::FieldReader<u16>;
#[doc = "Field `CH1B` writer - CH1B"]
pub type CH1B_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_MCU_CH1_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CH1A"]
    #[inline(always)]
    pub fn ch1a(&self) -> CH1A_R {
        CH1A_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CH1B"]
    #[inline(always)]
    pub fn ch1b(&self) -> CH1B_R {
        CH1B_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH1A"]
    #[inline(always)]
    #[must_use]
    pub fn ch1a(&mut self) -> CH1A_W<0> {
        CH1A_W::new(self)
    }
    #[doc = "Bits 16:31 - CH1B"]
    #[inline(always)]
    #[must_use]
    pub fn ch1b(&mut self) -> CH1B_W<16> {
        CH1B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_MCU_CH1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_mcu_ch1](index.html) module"]
pub struct MIDI_MCU_CH1_SPEC;
impl crate::RegisterSpec for MIDI_MCU_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_mcu_ch1::R](R) reader structure"]
impl crate::Readable for MIDI_MCU_CH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_mcu_ch1::W](W) writer structure"]
impl crate::Writable for MIDI_MCU_CH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_MCU_CH1 to value 0"]
impl crate::Resettable for MIDI_MCU_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
