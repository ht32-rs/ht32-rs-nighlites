#[doc = "Register `UART0_LSR` reader"]
pub struct R(crate::R<UART0_LSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART0_LSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART0_LSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART0_LSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART0_LSR` writer"]
pub struct W(crate::W<UART0_LSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART0_LSR_SPEC>;
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
impl From<crate::W<UART0_LSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART0_LSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFDR` reader - RFDR"]
pub type RFDR_R = crate::BitReader;
#[doc = "Field `RFDR` writer - RFDR"]
pub type RFDR_W<'a, const O: u8> = crate::BitWriter<'a, UART0_LSR_SPEC, O>;
#[doc = "Field `OEI` reader - OEI"]
pub type OEI_R = crate::BitReader;
#[doc = "Field `OEI` writer - OEI"]
pub type OEI_W<'a, const O: u8> = crate::BitWriter<'a, UART0_LSR_SPEC, O>;
#[doc = "Field `PEI` reader - PEI"]
pub type PEI_R = crate::BitReader;
#[doc = "Field `PEI` writer - PEI"]
pub type PEI_W<'a, const O: u8> = crate::BitWriter<'a, UART0_LSR_SPEC, O>;
#[doc = "Field `FEI` reader - FEI"]
pub type FEI_R = crate::BitReader;
#[doc = "Field `FEI` writer - FEI"]
pub type FEI_W<'a, const O: u8> = crate::BitWriter<'a, UART0_LSR_SPEC, O>;
#[doc = "Field `BII` reader - BII"]
pub type BII_R = crate::BitReader;
#[doc = "Field `BII` writer - BII"]
pub type BII_W<'a, const O: u8> = crate::BitWriter<'a, UART0_LSR_SPEC, O>;
#[doc = "Field `TXFEMPT` reader - TXFEMPT"]
pub type TXFEMPT_R = crate::BitReader;
#[doc = "Field `TXFEMPT` writer - TXFEMPT"]
pub type TXFEMPT_W<'a, const O: u8> = crate::BitWriter<'a, UART0_LSR_SPEC, O>;
#[doc = "Field `TXEMPT` reader - TXEMPT"]
pub type TXEMPT_R = crate::BitReader;
#[doc = "Field `TXEMPT` writer - TXEMPT"]
pub type TXEMPT_W<'a, const O: u8> = crate::BitWriter<'a, UART0_LSR_SPEC, O>;
#[doc = "Field `ERRRX` reader - ERRRX"]
pub type ERRRX_R = crate::BitReader;
#[doc = "Field `ERRRX` writer - ERRRX"]
pub type ERRRX_W<'a, const O: u8> = crate::BitWriter<'a, UART0_LSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - RFDR"]
    #[inline(always)]
    pub fn rfdr(&self) -> RFDR_R {
        RFDR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    pub fn oei(&self) -> OEI_R {
        OEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    pub fn pei(&self) -> PEI_R {
        PEI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    pub fn fei(&self) -> FEI_R {
        FEI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    pub fn bii(&self) -> BII_R {
        BII_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXFEMPT"]
    #[inline(always)]
    pub fn txfempt(&self) -> TXFEMPT_R {
        TXFEMPT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXEMPT"]
    #[inline(always)]
    pub fn txempt(&self) -> TXEMPT_R {
        TXEMPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ERRRX"]
    #[inline(always)]
    pub fn errrx(&self) -> ERRRX_R {
        ERRRX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RFDR"]
    #[inline(always)]
    #[must_use]
    pub fn rfdr(&mut self) -> RFDR_W<0> {
        RFDR_W::new(self)
    }
    #[doc = "Bit 1 - OEI"]
    #[inline(always)]
    #[must_use]
    pub fn oei(&mut self) -> OEI_W<1> {
        OEI_W::new(self)
    }
    #[doc = "Bit 2 - PEI"]
    #[inline(always)]
    #[must_use]
    pub fn pei(&mut self) -> PEI_W<2> {
        PEI_W::new(self)
    }
    #[doc = "Bit 3 - FEI"]
    #[inline(always)]
    #[must_use]
    pub fn fei(&mut self) -> FEI_W<3> {
        FEI_W::new(self)
    }
    #[doc = "Bit 4 - BII"]
    #[inline(always)]
    #[must_use]
    pub fn bii(&mut self) -> BII_W<4> {
        BII_W::new(self)
    }
    #[doc = "Bit 5 - TXFEMPT"]
    #[inline(always)]
    #[must_use]
    pub fn txfempt(&mut self) -> TXFEMPT_W<5> {
        TXFEMPT_W::new(self)
    }
    #[doc = "Bit 6 - TXEMPT"]
    #[inline(always)]
    #[must_use]
    pub fn txempt(&mut self) -> TXEMPT_W<6> {
        TXEMPT_W::new(self)
    }
    #[doc = "Bit 7 - ERRRX"]
    #[inline(always)]
    #[must_use]
    pub fn errrx(&mut self) -> ERRRX_W<7> {
        ERRRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0_LSR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_lsr](index.html) module"]
pub struct UART0_LSR_SPEC;
impl crate::RegisterSpec for UART0_LSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart0_lsr::R](R) reader structure"]
impl crate::Readable for UART0_LSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart0_lsr::W](W) writer structure"]
impl crate::Writable for UART0_LSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART0_LSR to value 0"]
impl crate::Resettable for UART0_LSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
