#[doc = "Register `MIDI_MCU_CH0` reader"]
pub struct R(crate::R<MIDI_MCU_CH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_MCU_CH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_MCU_CH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_MCU_CH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_MCU_CH0` writer"]
pub struct W(crate::W<MIDI_MCU_CH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_MCU_CH0_SPEC>;
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
impl From<crate::W<MIDI_MCU_CH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_MCU_CH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0A` reader - CH0A"]
pub type CH0A_R = crate::FieldReader<u16>;
#[doc = "Field `CH0A` writer - CH0A"]
pub type CH0A_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_MCU_CH0_SPEC, 16, O, u16>;
#[doc = "Field `CH0B` reader - CH0B"]
pub type CH0B_R = crate::FieldReader<u16>;
#[doc = "Field `CH0B` writer - CH0B"]
pub type CH0B_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_MCU_CH0_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CH0A"]
    #[inline(always)]
    pub fn ch0a(&self) -> CH0A_R {
        CH0A_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - CH0B"]
    #[inline(always)]
    pub fn ch0b(&self) -> CH0B_R {
        CH0B_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CH0A"]
    #[inline(always)]
    #[must_use]
    pub fn ch0a(&mut self) -> CH0A_W<0> {
        CH0A_W::new(self)
    }
    #[doc = "Bits 16:31 - CH0B"]
    #[inline(always)]
    #[must_use]
    pub fn ch0b(&mut self) -> CH0B_W<16> {
        CH0B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_MCU_CH0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_mcu_ch0](index.html) module"]
pub struct MIDI_MCU_CH0_SPEC;
impl crate::RegisterSpec for MIDI_MCU_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_mcu_ch0::R](R) reader structure"]
impl crate::Readable for MIDI_MCU_CH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_mcu_ch0::W](W) writer structure"]
impl crate::Writable for MIDI_MCU_CH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_MCU_CH0 to value 0"]
impl crate::Resettable for MIDI_MCU_CH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
