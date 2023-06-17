#[doc = "Register `SCI_IER` reader"]
pub struct R(crate::R<SCI_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCI_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCI_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCI_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCI_IER` writer"]
pub struct W(crate::W<SCI_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCI_IER_SPEC>;
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
impl From<crate::W<SCI_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCI_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARE` reader - PARE"]
pub type PARE_R = crate::BitReader;
#[doc = "Field `PARE` writer - PARE"]
pub type PARE_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IER_SPEC, O>;
#[doc = "Field `RXCE` reader - RXCE"]
pub type RXCE_R = crate::BitReader;
#[doc = "Field `RXCE` writer - RXCE"]
pub type RXCE_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IER_SPEC, O>;
#[doc = "Field `TXCE` reader - TXCE"]
pub type TXCE_R = crate::BitReader;
#[doc = "Field `TXCE` writer - TXCE"]
pub type TXCE_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IER_SPEC, O>;
#[doc = "Field `WTE` reader - WTE"]
pub type WTE_R = crate::BitReader;
#[doc = "Field `WTE` writer - WTE"]
pub type WTE_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IER_SPEC, O>;
#[doc = "Field `CARDIRE` reader - CARDIRE"]
pub type CARDIRE_R = crate::BitReader;
#[doc = "Field `CARDIRE` writer - CARDIRE"]
pub type CARDIRE_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IER_SPEC, O>;
#[doc = "Field `TXBEE` reader - TXBEE"]
pub type TXBEE_R = crate::BitReader;
#[doc = "Field `TXBEE` writer - TXBEE"]
pub type TXBEE_W<'a, const O: u8> = crate::BitWriter<'a, SCI_IER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PARE"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXCE"]
    #[inline(always)]
    pub fn rxce(&self) -> RXCE_R {
        RXCE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXCE"]
    #[inline(always)]
    pub fn txce(&self) -> TXCE_R {
        TXCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WTE"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CARDIRE"]
    #[inline(always)]
    pub fn cardire(&self) -> CARDIRE_R {
        CARDIRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXBEE"]
    #[inline(always)]
    pub fn txbee(&self) -> TXBEE_R {
        TXBEE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PARE"]
    #[inline(always)]
    #[must_use]
    pub fn pare(&mut self) -> PARE_W<0> {
        PARE_W::new(self)
    }
    #[doc = "Bit 1 - RXCE"]
    #[inline(always)]
    #[must_use]
    pub fn rxce(&mut self) -> RXCE_W<1> {
        RXCE_W::new(self)
    }
    #[doc = "Bit 2 - TXCE"]
    #[inline(always)]
    #[must_use]
    pub fn txce(&mut self) -> TXCE_W<2> {
        TXCE_W::new(self)
    }
    #[doc = "Bit 3 - WTE"]
    #[inline(always)]
    #[must_use]
    pub fn wte(&mut self) -> WTE_W<3> {
        WTE_W::new(self)
    }
    #[doc = "Bit 6 - CARDIRE"]
    #[inline(always)]
    #[must_use]
    pub fn cardire(&mut self) -> CARDIRE_W<6> {
        CARDIRE_W::new(self)
    }
    #[doc = "Bit 7 - TXBEE"]
    #[inline(always)]
    #[must_use]
    pub fn txbee(&mut self) -> TXBEE_W<7> {
        TXBEE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SCI_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sci_ier](index.html) module"]
pub struct SCI_IER_SPEC;
impl crate::RegisterSpec for SCI_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sci_ier::R](R) reader structure"]
impl crate::Readable for SCI_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sci_ier::W](W) writer structure"]
impl crate::Writable for SCI_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCI_IER to value 0"]
impl crate::Resettable for SCI_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
