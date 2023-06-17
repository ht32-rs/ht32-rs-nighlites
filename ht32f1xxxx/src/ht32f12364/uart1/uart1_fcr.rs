#[doc = "Register `UART1_FCR` reader"]
pub struct R(crate::R<UART1_FCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART1_FCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART1_FCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART1_FCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART1_FCR` writer"]
pub struct W(crate::W<UART1_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART1_FCR_SPEC>;
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
impl From<crate::W<UART1_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART1_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FME` reader - FME"]
pub type FME_R = crate::BitReader;
#[doc = "Field `FME` writer - FME"]
pub type FME_W<'a, const O: u8> = crate::BitWriter<'a, UART1_FCR_SPEC, O>;
#[doc = "Field `RFR` reader - RFR"]
pub type RFR_R = crate::BitReader;
#[doc = "Field `RFR` writer - RFR"]
pub type RFR_W<'a, const O: u8> = crate::BitWriter<'a, UART1_FCR_SPEC, O>;
#[doc = "Field `TFR` reader - TFR"]
pub type TFR_R = crate::BitReader;
#[doc = "Field `TFR` writer - TFR"]
pub type TFR_W<'a, const O: u8> = crate::BitWriter<'a, UART1_FCR_SPEC, O>;
#[doc = "Field `TFTL` reader - TFTL"]
pub type TFTL_R = crate::FieldReader;
#[doc = "Field `TFTL` writer - TFTL"]
pub type TFTL_W<'a, const O: u8> = crate::FieldWriter<'a, UART1_FCR_SPEC, 2, O>;
#[doc = "Field `RFTL` reader - RFTL"]
pub type RFTL_R = crate::FieldReader;
#[doc = "Field `RFTL` writer - RFTL"]
pub type RFTL_W<'a, const O: u8> = crate::FieldWriter<'a, UART1_FCR_SPEC, 2, O>;
#[doc = "Field `URTXEN` reader - URTXEN"]
pub type URTXEN_R = crate::BitReader;
#[doc = "Field `URTXEN` writer - URTXEN"]
pub type URTXEN_W<'a, const O: u8> = crate::BitWriter<'a, UART1_FCR_SPEC, O>;
#[doc = "Field `URRXEN` reader - URRXEN"]
pub type URRXEN_R = crate::BitReader;
#[doc = "Field `URRXEN` writer - URRXEN"]
pub type URRXEN_W<'a, const O: u8> = crate::BitWriter<'a, UART1_FCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - FME"]
    #[inline(always)]
    pub fn fme(&self) -> FME_R {
        FME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RFR"]
    #[inline(always)]
    pub fn rfr(&self) -> RFR_R {
        RFR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TFR"]
    #[inline(always)]
    pub fn tfr(&self) -> TFR_R {
        TFR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - TFTL"]
    #[inline(always)]
    pub fn tftl(&self) -> TFTL_R {
        TFTL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - RFTL"]
    #[inline(always)]
    pub fn rftl(&self) -> RFTL_R {
        RFTL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - URTXEN"]
    #[inline(always)]
    pub fn urtxen(&self) -> URTXEN_R {
        URTXEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - URRXEN"]
    #[inline(always)]
    pub fn urrxen(&self) -> URRXEN_R {
        URRXEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FME"]
    #[inline(always)]
    #[must_use]
    pub fn fme(&mut self) -> FME_W<0> {
        FME_W::new(self)
    }
    #[doc = "Bit 1 - RFR"]
    #[inline(always)]
    #[must_use]
    pub fn rfr(&mut self) -> RFR_W<1> {
        RFR_W::new(self)
    }
    #[doc = "Bit 2 - TFR"]
    #[inline(always)]
    #[must_use]
    pub fn tfr(&mut self) -> TFR_W<2> {
        TFR_W::new(self)
    }
    #[doc = "Bits 4:5 - TFTL"]
    #[inline(always)]
    #[must_use]
    pub fn tftl(&mut self) -> TFTL_W<4> {
        TFTL_W::new(self)
    }
    #[doc = "Bits 6:7 - RFTL"]
    #[inline(always)]
    #[must_use]
    pub fn rftl(&mut self) -> RFTL_W<6> {
        RFTL_W::new(self)
    }
    #[doc = "Bit 8 - URTXEN"]
    #[inline(always)]
    #[must_use]
    pub fn urtxen(&mut self) -> URTXEN_W<8> {
        URTXEN_W::new(self)
    }
    #[doc = "Bit 9 - URRXEN"]
    #[inline(always)]
    #[must_use]
    pub fn urrxen(&mut self) -> URRXEN_W<9> {
        URRXEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART1_FCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_fcr](index.html) module"]
pub struct UART1_FCR_SPEC;
impl crate::RegisterSpec for UART1_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart1_fcr::R](R) reader structure"]
impl crate::Readable for UART1_FCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart1_fcr::W](W) writer structure"]
impl crate::Writable for UART1_FCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART1_FCR to value 0"]
impl crate::Resettable for UART1_FCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
