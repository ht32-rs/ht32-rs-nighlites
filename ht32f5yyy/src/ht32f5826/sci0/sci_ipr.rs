#[doc = "Register `SCI_IPR` reader"]
pub struct R(crate::R<SCI_IPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCI_IPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCI_IPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCI_IPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCI_IPR` writer"]
pub struct W(crate::W<SCI_IPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCI_IPR_SPEC>;
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
impl From<crate::W<SCI_IPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCI_IPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARP` reader - PARP"]
pub type PARP_R = crate::BitReader;
#[doc = "Field `PARP` writer - PARP"]
pub type PARP_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IPR_SPEC, O>;
#[doc = "Field `RXCP` reader - RXCP"]
pub type RXCP_R = crate::BitReader;
#[doc = "Field `RXCP` writer - RXCP"]
pub type RXCP_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IPR_SPEC, O>;
#[doc = "Field `TXCP` reader - TXCP"]
pub type TXCP_R = crate::BitReader;
#[doc = "Field `TXCP` writer - TXCP"]
pub type TXCP_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IPR_SPEC, O>;
#[doc = "Field `WTP` reader - WTP"]
pub type WTP_R = crate::BitReader;
#[doc = "Field `WTP` writer - WTP"]
pub type WTP_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IPR_SPEC, O>;
#[doc = "Field `CARDIRP` reader - CARDIRP"]
pub type CARDIRP_R = crate::BitReader;
#[doc = "Field `CARDIRP` writer - CARDIRP"]
pub type CARDIRP_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IPR_SPEC, O>;
#[doc = "Field `TXBEP` reader - TXBEP"]
pub type TXBEP_R = crate::BitReader;
#[doc = "Field `TXBEP` writer - TXBEP"]
pub type TXBEP_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IPR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PARP"]
    #[inline(always)]
    pub fn parp(&self) -> PARP_R {
        PARP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXCP"]
    #[inline(always)]
    pub fn rxcp(&self) -> RXCP_R {
        RXCP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCP"]
    #[inline(always)]
    pub fn txcp(&self) -> TXCP_R {
        TXCP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WTP"]
    #[inline(always)]
    pub fn wtp(&self) -> WTP_R {
        WTP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CARDIRP"]
    #[inline(always)]
    pub fn cardirp(&self) -> CARDIRP_R {
        CARDIRP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXBEP"]
    #[inline(always)]
    pub fn txbep(&self) -> TXBEP_R {
        TXBEP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARP"]
    #[inline(always)]
    #[must_use]
    pub fn parp(&mut self) -> PARP_W<0> {
        PARP_W::new(self)
    }
    #[doc = "Bit 1 - RXCP"]
    #[inline(always)]
    #[must_use]
    pub fn rxcp(&mut self) -> RXCP_W<1> {
        RXCP_W::new(self)
    }
    #[doc = "Bit 2 - TXCP"]
    #[inline(always)]
    #[must_use]
    pub fn txcp(&mut self) -> TXCP_W<2> {
        TXCP_W::new(self)
    }
    #[doc = "Bit 3 - WTP"]
    #[inline(always)]
    #[must_use]
    pub fn wtp(&mut self) -> WTP_W<3> {
        WTP_W::new(self)
    }
    #[doc = "Bit 6 - CARDIRP"]
    #[inline(always)]
    #[must_use]
    pub fn cardirp(&mut self) -> CARDIRP_W<6> {
        CARDIRP_W::new(self)
    }
    #[doc = "Bit 7 - TXBEP"]
    #[inline(always)]
    #[must_use]
    pub fn txbep(&mut self) -> TXBEP_W<7> {
        TXBEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCI_IPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_ipr](index.html) module"]
pub struct SCI_IPR_SPEC;
impl crate::RegisterSpec for SCI_IPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sci_ipr::R](R) reader structure"]
impl crate::Readable for SCI_IPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sci_ipr::W](W) writer structure"]
impl crate::Writable for SCI_IPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCI_IPR to value 0"]
impl crate::Resettable for SCI_IPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
