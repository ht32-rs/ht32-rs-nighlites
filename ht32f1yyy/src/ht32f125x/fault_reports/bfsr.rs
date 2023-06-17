#[doc = "Register `BFSR` reader"]
pub struct R(crate::R<BFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFSR` writer"]
pub struct W(crate::W<BFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFSR_SPEC>;
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
impl From<crate::W<BFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IBUSERR` reader - IBUSERR"]
pub type IBUSERR_R = crate::BitReader;
#[doc = "Field `IBUSERR` writer - IBUSERR"]
pub type IBUSERR_W<'a, const O: u8> = crate::BitWriter<'a, BFSR_SPEC, O>;
#[doc = "Field `PRECISERR` reader - PRECISERR"]
pub type PRECISERR_R = crate::BitReader;
#[doc = "Field `PRECISERR` writer - PRECISERR"]
pub type PRECISERR_W<'a, const O: u8> = crate::BitWriter<'a, BFSR_SPEC, O>;
#[doc = "Field `IMPRECISERR` reader - IMPRECISERR"]
pub type IMPRECISERR_R = crate::BitReader;
#[doc = "Field `IMPRECISERR` writer - IMPRECISERR"]
pub type IMPRECISERR_W<'a, const O: u8> = crate::BitWriter<'a, BFSR_SPEC, O>;
#[doc = "Field `UNSTKERR` reader - UNSTKERR"]
pub type UNSTKERR_R = crate::BitReader;
#[doc = "Field `UNSTKERR` writer - UNSTKERR"]
pub type UNSTKERR_W<'a, const O: u8> = crate::BitWriter<'a, BFSR_SPEC, O>;
#[doc = "Field `STKERR` reader - STKERR"]
pub type STKERR_R = crate::BitReader;
#[doc = "Field `STKERR` writer - STKERR"]
pub type STKERR_W<'a, const O: u8> = crate::BitWriter<'a, BFSR_SPEC, O>;
#[doc = "Field `BFARVALID` reader - BFARVALID"]
pub type BFARVALID_R = crate::BitReader;
#[doc = "Field `BFARVALID` writer - BFARVALID"]
pub type BFARVALID_W<'a, const O: u8> = crate::BitWriter<'a, BFSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - IBUSERR"]
    #[inline(always)]
    pub fn ibuserr(&self) -> IBUSERR_R {
        IBUSERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PRECISERR"]
    #[inline(always)]
    pub fn preciserr(&self) -> PRECISERR_R {
        PRECISERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IMPRECISERR"]
    #[inline(always)]
    pub fn impreciserr(&self) -> IMPRECISERR_R {
        IMPRECISERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UNSTKERR"]
    #[inline(always)]
    pub fn unstkerr(&self) -> UNSTKERR_R {
        UNSTKERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STKERR"]
    #[inline(always)]
    pub fn stkerr(&self) -> STKERR_R {
        STKERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - BFARVALID"]
    #[inline(always)]
    pub fn bfarvalid(&self) -> BFARVALID_R {
        BFARVALID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IBUSERR"]
    #[inline(always)]
    #[must_use]
    pub fn ibuserr(&mut self) -> IBUSERR_W<0> {
        IBUSERR_W::new(self)
    }
    #[doc = "Bit 1 - PRECISERR"]
    #[inline(always)]
    #[must_use]
    pub fn preciserr(&mut self) -> PRECISERR_W<1> {
        PRECISERR_W::new(self)
    }
    #[doc = "Bit 2 - IMPRECISERR"]
    #[inline(always)]
    #[must_use]
    pub fn impreciserr(&mut self) -> IMPRECISERR_W<2> {
        IMPRECISERR_W::new(self)
    }
    #[doc = "Bit 3 - UNSTKERR"]
    #[inline(always)]
    #[must_use]
    pub fn unstkerr(&mut self) -> UNSTKERR_W<3> {
        UNSTKERR_W::new(self)
    }
    #[doc = "Bit 4 - STKERR"]
    #[inline(always)]
    #[must_use]
    pub fn stkerr(&mut self) -> STKERR_W<4> {
        STKERR_W::new(self)
    }
    #[doc = "Bit 7 - BFARVALID"]
    #[inline(always)]
    #[must_use]
    pub fn bfarvalid(&mut self) -> BFARVALID_W<7> {
        BFARVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BFSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfsr](index.html) module"]
pub struct BFSR_SPEC;
impl crate::RegisterSpec for BFSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bfsr::R](R) reader structure"]
impl crate::Readable for BFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bfsr::W](W) writer structure"]
impl crate::Writable for BFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFSR to value 0"]
impl crate::Resettable for BFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
