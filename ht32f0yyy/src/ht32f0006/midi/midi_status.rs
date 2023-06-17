#[doc = "Register `MIDI_STATUS` reader"]
pub struct R(crate::R<MIDI_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_STATUS` writer"]
pub struct W(crate::W<MIDI_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_STATUS_SPEC>;
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
impl From<crate::W<MIDI_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTF` reader - INTF"]
pub type INTF_R = crate::BitReader;
#[doc = "Field `INTF` writer - INTF"]
pub type INTF_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_STATUS_SPEC, O>;
impl R {
    #[doc = "Bit 0 - INTF"]
    #[inline(always)]
    pub fn intf(&self) -> INTF_R {
        INTF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INTF"]
    #[inline(always)]
    #[must_use]
    pub fn intf(&mut self) -> INTF_W<0> {
        INTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_STATUS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_status](index.html) module"]
pub struct MIDI_STATUS_SPEC;
impl crate::RegisterSpec for MIDI_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_status::R](R) reader structure"]
impl crate::Readable for MIDI_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_status::W](W) writer structure"]
impl crate::Writable for MIDI_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_STATUS to value 0"]
impl crate::Resettable for MIDI_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
