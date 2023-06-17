#[doc = "Register `RTCSR` reader"]
pub struct R(crate::R<RTCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCSR` writer"]
pub struct W(crate::W<RTCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCSR_SPEC>;
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
impl From<crate::W<RTCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSECFLAG` reader - CSECFLAG"]
pub type CSECFLAG_R = crate::BitReader;
#[doc = "Field `CSECFLAG` writer - CSECFLAG"]
pub type CSECFLAG_W<'a, const O: u8> = crate::BitWriter<'a, RTCSR_SPEC, O>;
#[doc = "Field `CMFLAG` reader - CMFLAG"]
pub type CMFLAG_R = crate::BitReader;
#[doc = "Field `CMFLAG` writer - CMFLAG"]
pub type CMFLAG_W<'a, const O: u8> = crate::BitWriter<'a, RTCSR_SPEC, O>;
#[doc = "Field `OVFLAG` reader - OVFLAG"]
pub type OVFLAG_R = crate::BitReader;
#[doc = "Field `OVFLAG` writer - OVFLAG"]
pub type OVFLAG_W<'a, const O: u8> = crate::BitWriter<'a, RTCSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CSECFLAG"]
    #[inline(always)]
    pub fn csecflag(&self) -> CSECFLAG_R {
        CSECFLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMFLAG"]
    #[inline(always)]
    pub fn cmflag(&self) -> CMFLAG_R {
        CMFLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OVFLAG"]
    #[inline(always)]
    pub fn ovflag(&self) -> OVFLAG_R {
        OVFLAG_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSECFLAG"]
    #[inline(always)]
    #[must_use]
    pub fn csecflag(&mut self) -> CSECFLAG_W<0> {
        CSECFLAG_W::new(self)
    }
    #[doc = "Bit 1 - CMFLAG"]
    #[inline(always)]
    #[must_use]
    pub fn cmflag(&mut self) -> CMFLAG_W<1> {
        CMFLAG_W::new(self)
    }
    #[doc = "Bit 2 - OVFLAG"]
    #[inline(always)]
    #[must_use]
    pub fn ovflag(&mut self) -> OVFLAG_W<2> {
        OVFLAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcsr](index.html) module"]
pub struct RTCSR_SPEC;
impl crate::RegisterSpec for RTCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtcsr::R](R) reader structure"]
impl crate::Readable for RTCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcsr::W](W) writer structure"]
impl crate::Writable for RTCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTCSR to value 0"]
impl crate::Resettable for RTCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
