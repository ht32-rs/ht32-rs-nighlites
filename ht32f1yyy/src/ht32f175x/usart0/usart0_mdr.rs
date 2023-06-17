#[doc = "Register `USART0_MDR` reader"]
pub struct R(crate::R<USART0_MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART0_MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART0_MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART0_MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART0_MDR` writer"]
pub struct W(crate::W<USART0_MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART0_MDR_SPEC>;
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
impl From<crate::W<USART0_MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART0_MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, USART0_MDR_SPEC, 2, O>;
#[doc = "Field `TRSM` reader - TRSM"]
pub type TRSM_R = crate::BitReader;
#[doc = "Field `TRSM` writer - TRSM"]
pub type TRSM_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MDR_SPEC, O>;
#[doc = "Field `TXDMAEN` reader - TXDMAEN"]
pub type TXDMAEN_R = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - TXDMAEN"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MDR_SPEC, O>;
#[doc = "Field `RXDMAEN` reader - RXDMAEN"]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - RXDMAEN"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, USART0_MDR_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    pub fn trsm(&self) -> TRSM_R {
        TRSM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TXDMAEN"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXDMAEN"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - MODE"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bit 2 - TRSM"]
    #[inline(always)]
    #[must_use]
    pub fn trsm(&mut self) -> TRSM_W<2> {
        TRSM_W::new(self)
    }
    #[doc = "Bit 4 - TXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<4> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 5 - RXDMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<5> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART0_MDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_mdr](index.html) module"]
pub struct USART0_MDR_SPEC;
impl crate::RegisterSpec for USART0_MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart0_mdr::R](R) reader structure"]
impl crate::Readable for USART0_MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart0_mdr::W](W) writer structure"]
impl crate::Writable for USART0_MDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART0_MDR to value 0"]
impl crate::Resettable for USART0_MDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
