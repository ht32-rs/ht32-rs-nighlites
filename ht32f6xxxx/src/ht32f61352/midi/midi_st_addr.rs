#[doc = "Register `MIDI_ST_ADDR` reader"]
pub struct R(crate::R<MIDI_ST_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_ST_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_ST_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_ST_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_ST_ADDR` writer"]
pub struct W(crate::W<MIDI_ST_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_ST_ADDR_SPEC>;
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
impl From<crate::W<MIDI_ST_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_ST_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ST` reader - ST"]
pub type ST_R = crate::FieldReader<u32>;
#[doc = "Field `ST` writer - ST"]
pub type ST_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_ST_ADDR_SPEC, 19, O, u32>;
impl R {
    #[doc = "Bits 0:18 - ST"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(self.bits & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:18 - ST"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<0> {
        ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_ST_ADDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_st_addr](index.html) module"]
pub struct MIDI_ST_ADDR_SPEC;
impl crate::RegisterSpec for MIDI_ST_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_st_addr::R](R) reader structure"]
impl crate::Readable for MIDI_ST_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_st_addr::W](W) writer structure"]
impl crate::Writable for MIDI_ST_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_ST_ADDR to value 0"]
impl crate::Resettable for MIDI_ST_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
