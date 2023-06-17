#[doc = "Register `MIDI_CHAN` reader"]
pub struct R(crate::R<MIDI_CHAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_CHAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_CHAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_CHAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_CHAN` writer"]
pub struct W(crate::W<MIDI_CHAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_CHAN_SPEC>;
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
impl From<crate::W<MIDI_CHAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_CHAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH` reader - CH"]
pub type CH_R = crate::FieldReader;
#[doc = "Field `CH` writer - CH"]
pub type CH_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_CHAN_SPEC, 5, O>;
#[doc = "Field `FR` reader - FR"]
pub type FR_R = crate::BitReader;
#[doc = "Field `FR` writer - FR"]
pub type FR_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_CHAN_SPEC, O>;
#[doc = "Field `VM` reader - VM"]
pub type VM_R = crate::BitReader;
#[doc = "Field `VM` writer - VM"]
pub type VM_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_CHAN_SPEC, O>;
#[doc = "Field `ST` reader - ST"]
pub type ST_R = crate::BitReader;
#[doc = "Field `ST` writer - ST"]
pub type ST_W<'a, const O: u8> = crate::BitWriter<'a, MIDI_CHAN_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - CH"]
    #[inline(always)]
    pub fn ch(&self) -> CH_R {
        CH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - FR"]
    #[inline(always)]
    pub fn fr(&self) -> FR_R {
        FR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VM"]
    #[inline(always)]
    pub fn vm(&self) -> VM_R {
        VM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - CH"]
    #[inline(always)]
    #[must_use]
    pub fn ch(&mut self) -> CH_W<0> {
        CH_W::new(self)
    }
    #[doc = "Bit 8 - FR"]
    #[inline(always)]
    #[must_use]
    pub fn fr(&mut self) -> FR_W<8> {
        FR_W::new(self)
    }
    #[doc = "Bit 9 - VM"]
    #[inline(always)]
    #[must_use]
    pub fn vm(&mut self) -> VM_W<9> {
        VM_W::new(self)
    }
    #[doc = "Bit 10 - ST"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<10> {
        ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_CHAN\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_chan](index.html) module"]
pub struct MIDI_CHAN_SPEC;
impl crate::RegisterSpec for MIDI_CHAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_chan::R](R) reader structure"]
impl crate::Readable for MIDI_CHAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_chan::W](W) writer structure"]
impl crate::Writable for MIDI_CHAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_CHAN to value 0"]
impl crate::Resettable for MIDI_CHAN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
