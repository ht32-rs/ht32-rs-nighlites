#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCNT0` reader - TCNT0"]
pub type TCNT0_R = crate::FieldReader<u16>;
#[doc = "Field `TCNT0` writer - TCNT0"]
pub type TCNT0_W<'a, const O: u8> = crate::FieldWriter<'a, TCR_SPEC, 10, O, u16>;
#[doc = "Field `TCNT1` reader - TCNT1"]
pub type TCNT1_R = crate::FieldReader<u16>;
#[doc = "Field `TCNT1` writer - TCNT1"]
pub type TCNT1_W<'a, const O: u8> = crate::FieldWriter<'a, TCR_SPEC, 10, O, u16>;
impl R {
    #[doc = "Bits 0:9 - TCNT0"]
    #[inline(always)]
    pub fn tcnt0(&self) -> TCNT0_R {
        TCNT0_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - TCNT1"]
    #[inline(always)]
    pub fn tcnt1(&self) -> TCNT1_R {
        TCNT1_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - TCNT0"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt0(&mut self) -> TCNT0_W<0> {
        TCNT0_W::new(self)
    }
    #[doc = "Bits 16:25 - TCNT1"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt1(&mut self) -> TCNT1_W<16> {
        TCNT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint data transfer byte count\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
