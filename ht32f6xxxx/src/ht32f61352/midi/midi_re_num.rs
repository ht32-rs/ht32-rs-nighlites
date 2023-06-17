#[doc = "Register `MIDI_RE_NUM` reader"]
pub struct R(crate::R<MIDI_RE_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_RE_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_RE_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_RE_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_RE_NUM` writer"]
pub struct W(crate::W<MIDI_RE_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_RE_NUM_SPEC>;
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
impl From<crate::W<MIDI_RE_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_RE_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RE` reader - RE"]
pub type RE_R = crate::FieldReader<u16>;
#[doc = "Field `RE` writer - RE"]
pub type RE_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_RE_NUM_SPEC, 15, O, u16>;
#[doc = "Field `WBS` reader - WBS"]
pub type WBS_R = crate::FieldReader;
#[doc = "Field `WBS` writer - WBS"]
pub type WBS_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_RE_NUM_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:14 - RE"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:17 - WBS"]
    #[inline(always)]
    pub fn wbs(&self) -> WBS_R {
        WBS_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - RE"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<0> {
        RE_W::new(self)
    }
    #[doc = "Bits 16:17 - WBS"]
    #[inline(always)]
    #[must_use]
    pub fn wbs(&mut self) -> WBS_W<16> {
        WBS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_RE_NUM\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_re_num](index.html) module"]
pub struct MIDI_RE_NUM_SPEC;
impl crate::RegisterSpec for MIDI_RE_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_re_num::R](R) reader structure"]
impl crate::Readable for MIDI_RE_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_re_num::W](W) writer structure"]
impl crate::Writable for MIDI_RE_NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_RE_NUM to value 0"]
impl crate::Resettable for MIDI_RE_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
