#[doc = "Register `MFSR` reader"]
pub struct R(crate::R<FAULT_REPORTS_MFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAULT_REPORTS_MFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAULT_REPORTS_MFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAULT_REPORTS_MFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MFSR` writer"]
pub struct W(crate::W<FAULT_REPORTS_MFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAULT_REPORTS_MFSR_SPEC>;
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
impl From<crate::W<FAULT_REPORTS_MFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAULT_REPORTS_MFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IACCVIOL` reader - IACCVIOL"]
pub type IACCVIOL_R = crate::BitReader;
#[doc = "Field `IACCVIOL` writer - IACCVIOL"]
pub type IACCVIOL_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_REPORTS_MFSR_SPEC, O>;
#[doc = "Field `DACCVIOL` reader - DACCVIOL"]
pub type DACCVIOL_R = crate::BitReader;
#[doc = "Field `DACCVIOL` writer - DACCVIOL"]
pub type DACCVIOL_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_REPORTS_MFSR_SPEC, O>;
#[doc = "Field `MUNSTKERR` reader - MUNSTKERR"]
pub type MUNSTKERR_R = crate::BitReader;
#[doc = "Field `MUNSTKERR` writer - MUNSTKERR"]
pub type MUNSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_REPORTS_MFSR_SPEC, O>;
#[doc = "Field `MSTKERR` reader - MSTKERR"]
pub type MSTKERR_R = crate::BitReader;
#[doc = "Field `MSTKERR` writer - MSTKERR"]
pub type MSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_REPORTS_MFSR_SPEC, O>;
#[doc = "Field `MMARVALID` reader - MMARVALID"]
pub type MMARVALID_R = crate::BitReader;
#[doc = "Field `MMARVALID` writer - MMARVALID"]
pub type MMARVALID_W<'a, const O: u8> = crate::BitWriter<'a, FAULT_REPORTS_MFSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - IACCVIOL"]
    #[inline(always)]
    pub fn iaccviol(&self) -> IACCVIOL_R {
        IACCVIOL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DACCVIOL"]
    #[inline(always)]
    pub fn daccviol(&self) -> DACCVIOL_R {
        DACCVIOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - MUNSTKERR"]
    #[inline(always)]
    pub fn munstkerr(&self) -> MUNSTKERR_R {
        MUNSTKERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSTKERR"]
    #[inline(always)]
    pub fn mstkerr(&self) -> MSTKERR_R {
        MSTKERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - MMARVALID"]
    #[inline(always)]
    pub fn mmarvalid(&self) -> MMARVALID_R {
        MMARVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IACCVIOL"]
    #[inline(always)]
    #[must_use]
    pub fn iaccviol(&mut self) -> IACCVIOL_W<0> {
        IACCVIOL_W::new(self)
    }
    #[doc = "Bit 1 - DACCVIOL"]
    #[inline(always)]
    #[must_use]
    pub fn daccviol(&mut self) -> DACCVIOL_W<1> {
        DACCVIOL_W::new(self)
    }
    #[doc = "Bit 3 - MUNSTKERR"]
    #[inline(always)]
    #[must_use]
    pub fn munstkerr(&mut self) -> MUNSTKERR_W<3> {
        MUNSTKERR_W::new(self)
    }
    #[doc = "Bit 4 - MSTKERR"]
    #[inline(always)]
    #[must_use]
    pub fn mstkerr(&mut self) -> MSTKERR_W<4> {
        MSTKERR_W::new(self)
    }
    #[doc = "Bit 7 - MMARVALID"]
    #[inline(always)]
    #[must_use]
    pub fn mmarvalid(&mut self) -> MMARVALID_W<7> {
        MMARVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fault_reports_mfsr](index.html) module"]
pub struct FAULT_REPORTS_MFSR_SPEC;
impl crate::RegisterSpec for FAULT_REPORTS_MFSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fault_reports_mfsr::R](R) reader structure"]
impl crate::Readable for FAULT_REPORTS_MFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fault_reports_mfsr::W](W) writer structure"]
impl crate::Writable for FAULT_REPORTS_MFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MFSR to value 0"]
impl crate::Resettable for FAULT_REPORTS_MFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
