#[doc = "Register `URIER` reader"]
pub struct R(crate::R<URIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<URIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<URIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<URIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `URIER` writer"]
pub struct W(crate::W<URIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<URIER_SPEC>;
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
impl From<crate::W<URIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<URIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDRIE` reader - RXDRIE"]
pub type RXDRIE_R = crate::BitReader;
#[doc = "Field `RXDRIE` writer - RXDRIE"]
pub type RXDRIE_W<'a, const O: u8> = crate::BitWriter<'a, URIER_SPEC, O>;
#[doc = "Field `TXDEIE` reader - TXDEIE"]
pub type TXDEIE_R = crate::BitReader;
#[doc = "Field `TXDEIE` writer - TXDEIE"]
pub type TXDEIE_W<'a, const O: u8> = crate::BitWriter<'a, URIER_SPEC, O>;
#[doc = "Field `TXCIE` reader - TXCIE"]
pub type TXCIE_R = crate::BitReader;
#[doc = "Field `TXCIE` writer - TXCIE"]
pub type TXCIE_W<'a, const O: u8> = crate::BitWriter<'a, URIER_SPEC, O>;
#[doc = "Field `OEI` reader - OEI"]
pub type OEI_R = crate::BitReader;
#[doc = "Field `OEI` writer - OEI"]
pub type OEI_W<'a, const O: u8> = crate::BitWriter<'a, URIER_SPEC, O>;
#[doc = "Field `PEIE` reader - PEIE"]
pub type PEIE_R = crate::BitReader;
#[doc = "Field `PEIE` writer - PEIE"]
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, URIER_SPEC, O>;
#[doc = "Field `FEIE` reader - FEIE"]
pub type FEIE_R = crate::BitReader;
#[doc = "Field `FEIE` writer - FEIE"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, URIER_SPEC, O>;
#[doc = "Field `BIE` reader - BIE"]
pub type BIE_R = crate::BitReader;
#[doc = "Field `BIE` writer - BIE"]
pub type BIE_W<'a, const O: u8> = crate::BitWriter<'a, URIER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - RXDRIE"]
    #[inline(always)]
    pub fn rxdrie(&self) -> RXDRIE_R {
        RXDRIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXDEIE"]
    #[inline(always)]
    pub fn txdeie(&self) -> TXDEIE_R {
        TXDEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCIE"]
    #[inline(always)]
    pub fn txcie(&self) -> TXCIE_R {
        TXCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OEI"]
    #[inline(always)]
    pub fn oei(&self) -> OEI_R {
        OEI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PEIE"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FEIE"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BIE"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXDRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rxdrie(&mut self) -> RXDRIE_W<0> {
        RXDRIE_W::new(self)
    }
    #[doc = "Bit 1 - TXDEIE"]
    #[inline(always)]
    #[must_use]
    pub fn txdeie(&mut self) -> TXDEIE_W<1> {
        TXDEIE_W::new(self)
    }
    #[doc = "Bit 2 - TXCIE"]
    #[inline(always)]
    #[must_use]
    pub fn txcie(&mut self) -> TXCIE_W<2> {
        TXCIE_W::new(self)
    }
    #[doc = "Bit 3 - OEI"]
    #[inline(always)]
    #[must_use]
    pub fn oei(&mut self) -> OEI_W<3> {
        OEI_W::new(self)
    }
    #[doc = "Bit 4 - PEIE"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<4> {
        PEIE_W::new(self)
    }
    #[doc = "Bit 5 - FEIE"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<5> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 6 - BIE"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<6> {
        BIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "URIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urier](index.html) module"]
pub struct URIER_SPEC;
impl crate::RegisterSpec for URIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [urier::R](R) reader structure"]
impl crate::Readable for URIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [urier::W](W) writer structure"]
impl crate::Writable for URIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets URIER to value 0"]
impl crate::Resettable for URIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
