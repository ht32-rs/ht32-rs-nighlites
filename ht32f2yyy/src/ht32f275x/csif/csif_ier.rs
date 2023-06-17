#[doc = "Register `CSIF_IER` reader"]
pub struct R(crate::R<CSIF_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSIF_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSIF_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSIF_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSIF_IER` writer"]
pub struct W(crate::W<CSIF_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSIF_IER_SPEC>;
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
impl From<crate::W<CSIF_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSIF_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFFLGE` reader - SOFFLGE"]
pub type SOFFLGE_R = crate::BitReader;
#[doc = "Field `SOFFLGE` writer - SOFFLGE"]
pub type SOFFLGE_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_IER_SPEC, O>;
#[doc = "Field `EOFFLGE` reader - EOFFLGE"]
pub type EOFFLGE_R = crate::BitReader;
#[doc = "Field `EOFFLGE` writer - EOFFLGE"]
pub type EOFFLGE_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_IER_SPEC, O>;
#[doc = "Field `CAPSTAE` reader - CAPSTAE"]
pub type CAPSTAE_R = crate::BitReader;
#[doc = "Field `CAPSTAE` writer - CAPSTAE"]
pub type CAPSTAE_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_IER_SPEC, O>;
#[doc = "Field `CAPSTSE` reader - CAPSTSE"]
pub type CAPSTSE_R = crate::BitReader;
#[doc = "Field `CAPSTSE` writer - CAPSTSE"]
pub type CAPSTSE_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_IER_SPEC, O>;
#[doc = "Field `BADFRME` reader - BADFRME"]
pub type BADFRME_R = crate::BitReader;
#[doc = "Field `BADFRME` writer - BADFRME"]
pub type BADFRME_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_IER_SPEC, O>;
#[doc = "Field `FIFOOVRE` reader - FIFOOVRE"]
pub type FIFOOVRE_R = crate::BitReader;
#[doc = "Field `FIFOOVRE` writer - FIFOOVRE"]
pub type FIFOOVRE_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_IER_SPEC, O>;
#[doc = "Field `FIFOEMPE` reader - FIFOEMPE"]
pub type FIFOEMPE_R = crate::BitReader;
#[doc = "Field `FIFOEMPE` writer - FIFOEMPE"]
pub type FIFOEMPE_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_IER_SPEC, O>;
#[doc = "Field `FIFOFULE` reader - FIFOFULE"]
pub type FIFOFULE_R = crate::BitReader;
#[doc = "Field `FIFOFULE` writer - FIFOFULE"]
pub type FIFOFULE_W<'a, const O: u8> = crate::BitWriter<'a, CSIF_IER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - SOFFLGE"]
    #[inline(always)]
    pub fn sofflge(&self) -> SOFFLGE_R {
        SOFFLGE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOFFLGE"]
    #[inline(always)]
    pub fn eofflge(&self) -> EOFFLGE_R {
        EOFFLGE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CAPSTAE"]
    #[inline(always)]
    pub fn capstae(&self) -> CAPSTAE_R {
        CAPSTAE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CAPSTSE"]
    #[inline(always)]
    pub fn capstse(&self) -> CAPSTSE_R {
        CAPSTSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BADFRME"]
    #[inline(always)]
    pub fn badfrme(&self) -> BADFRME_R {
        BADFRME_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFOOVRE"]
    #[inline(always)]
    pub fn fifoovre(&self) -> FIFOOVRE_R {
        FIFOOVRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFOEMPE"]
    #[inline(always)]
    pub fn fifoempe(&self) -> FIFOEMPE_R {
        FIFOEMPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FIFOFULE"]
    #[inline(always)]
    pub fn fifofule(&self) -> FIFOFULE_R {
        FIFOFULE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SOFFLGE"]
    #[inline(always)]
    #[must_use]
    pub fn sofflge(&mut self) -> SOFFLGE_W<0> {
        SOFFLGE_W::new(self)
    }
    #[doc = "Bit 1 - EOFFLGE"]
    #[inline(always)]
    #[must_use]
    pub fn eofflge(&mut self) -> EOFFLGE_W<1> {
        EOFFLGE_W::new(self)
    }
    #[doc = "Bit 2 - CAPSTAE"]
    #[inline(always)]
    #[must_use]
    pub fn capstae(&mut self) -> CAPSTAE_W<2> {
        CAPSTAE_W::new(self)
    }
    #[doc = "Bit 3 - CAPSTSE"]
    #[inline(always)]
    #[must_use]
    pub fn capstse(&mut self) -> CAPSTSE_W<3> {
        CAPSTSE_W::new(self)
    }
    #[doc = "Bit 4 - BADFRME"]
    #[inline(always)]
    #[must_use]
    pub fn badfrme(&mut self) -> BADFRME_W<4> {
        BADFRME_W::new(self)
    }
    #[doc = "Bit 8 - FIFOOVRE"]
    #[inline(always)]
    #[must_use]
    pub fn fifoovre(&mut self) -> FIFOOVRE_W<8> {
        FIFOOVRE_W::new(self)
    }
    #[doc = "Bit 9 - FIFOEMPE"]
    #[inline(always)]
    #[must_use]
    pub fn fifoempe(&mut self) -> FIFOEMPE_W<9> {
        FIFOEMPE_W::new(self)
    }
    #[doc = "Bit 10 - FIFOFULE"]
    #[inline(always)]
    #[must_use]
    pub fn fifofule(&mut self) -> FIFOFULE_W<10> {
        FIFOFULE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CSIF_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csif_ier](index.html) module"]
pub struct CSIF_IER_SPEC;
impl crate::RegisterSpec for CSIF_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csif_ier::R](R) reader structure"]
impl crate::Readable for CSIF_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csif_ier::W](W) writer structure"]
impl crate::Writable for CSIF_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSIF_IER to value 0"]
impl crate::Resettable for CSIF_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
