#[doc = "Register `SPISR` reader"]
pub struct R(crate::R<SPISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPISR` writer"]
pub struct W(crate::W<SPISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPISR_SPEC>;
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
impl From<crate::W<SPISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXBE` reader - TXBE"]
pub type TXBE_R = crate::BitReader;
#[doc = "Field `TXBE` writer - TXBE"]
pub type TXBE_W<'a, const O: u8> = crate::BitWriter<'a, SPISR_SPEC, O>;
#[doc = "Field `TXE` reader - TXE"]
pub type TXE_R = crate::BitReader;
#[doc = "Field `TXE` writer - TXE"]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, SPISR_SPEC, O>;
#[doc = "Field `RXBNE` reader - RXBNE"]
pub type RXBNE_R = crate::BitReader;
#[doc = "Field `RXBNE` writer - RXBNE"]
pub type RXBNE_W<'a, const O: u8> = crate::BitWriter<'a, SPISR_SPEC, O>;
#[doc = "Field `WC` reader - WC"]
pub type WC_R = crate::BitReader;
#[doc = "Field `WC` writer - WC"]
pub type WC_W<'a, const O: u8> = crate::BitWriter<'a, SPISR_SPEC, O>;
#[doc = "Field `RO` reader - RO"]
pub type RO_R = crate::BitReader;
#[doc = "Field `RO` writer - RO"]
pub type RO_W<'a, const O: u8> = crate::BitWriter<'a, SPISR_SPEC, O>;
#[doc = "Field `MF` reader - MF"]
pub type MF_R = crate::BitReader;
#[doc = "Field `MF` writer - MF"]
pub type MF_W<'a, const O: u8> = crate::BitWriter<'a, SPISR_SPEC, O>;
#[doc = "Field `SA` reader - SA"]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - SA"]
pub type SA_W<'a, const O: u8> = crate::BitWriter<'a, SPISR_SPEC, O>;
#[doc = "Field `TO` reader - TO"]
pub type TO_R = crate::BitReader;
#[doc = "Field `TO` writer - TO"]
pub type TO_W<'a, const O: u8> = crate::BitWriter<'a, SPISR_SPEC, O>;
#[doc = "Field `BUSY` reader - BUSY"]
pub type BUSY_R = crate::BitReader;
#[doc = "Field `BUSY` writer - BUSY"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, SPISR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - TXBE"]
    #[inline(always)]
    pub fn txbe(&self) -> TXBE_R {
        TXBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXE"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXBNE"]
    #[inline(always)]
    pub fn rxbne(&self) -> RXBNE_R {
        RXBNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WC"]
    #[inline(always)]
    pub fn wc(&self) -> WC_R {
        WC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RO"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MF"]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SA"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TO"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXBE"]
    #[inline(always)]
    #[must_use]
    pub fn txbe(&mut self) -> TXBE_W<0> {
        TXBE_W::new(self)
    }
    #[doc = "Bit 1 - TXE"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<1> {
        TXE_W::new(self)
    }
    #[doc = "Bit 2 - RXBNE"]
    #[inline(always)]
    #[must_use]
    pub fn rxbne(&mut self) -> RXBNE_W<2> {
        RXBNE_W::new(self)
    }
    #[doc = "Bit 3 - WC"]
    #[inline(always)]
    #[must_use]
    pub fn wc(&mut self) -> WC_W<3> {
        WC_W::new(self)
    }
    #[doc = "Bit 4 - RO"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RO_W<4> {
        RO_W::new(self)
    }
    #[doc = "Bit 5 - MF"]
    #[inline(always)]
    #[must_use]
    pub fn mf(&mut self) -> MF_W<5> {
        MF_W::new(self)
    }
    #[doc = "Bit 6 - SA"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<6> {
        SA_W::new(self)
    }
    #[doc = "Bit 7 - TO"]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<7> {
        TO_W::new(self)
    }
    #[doc = "Bit 8 - BUSY"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<8> {
        BUSY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spisr](index.html) module"]
pub struct SPISR_SPEC;
impl crate::RegisterSpec for SPISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spisr::R](R) reader structure"]
impl crate::Readable for SPISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spisr::W](W) writer structure"]
impl crate::Writable for SPISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPISR to value 0"]
impl crate::Resettable for SPISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
