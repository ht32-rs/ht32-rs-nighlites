#[doc = "Register `UART1_LCR` reader"]
pub struct R(crate::R<UART1_LCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART1_LCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART1_LCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART1_LCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART1_LCR` writer"]
pub struct W(crate::W<UART1_LCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART1_LCR_SPEC>;
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
impl From<crate::W<UART1_LCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART1_LCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WLS` reader - WLS"]
pub type WLS_R = crate::FieldReader;
#[doc = "Field `WLS` writer - WLS"]
pub type WLS_W<'a, const O: u8> = crate::FieldWriter<'a, UART1_LCR_SPEC, 2, O>;
#[doc = "Field `NSB` reader - NSB"]
pub type NSB_R = crate::BitReader;
#[doc = "Field `NSB` writer - NSB"]
pub type NSB_W<'a, const O: u8> = crate::BitWriter<'a, UART1_LCR_SPEC, O>;
#[doc = "Field `PBE` reader - PBE"]
pub type PBE_R = crate::BitReader;
#[doc = "Field `PBE` writer - PBE"]
pub type PBE_W<'a, const O: u8> = crate::BitWriter<'a, UART1_LCR_SPEC, O>;
#[doc = "Field `EPE` reader - EPE"]
pub type EPE_R = crate::BitReader;
#[doc = "Field `EPE` writer - EPE"]
pub type EPE_W<'a, const O: u8> = crate::BitWriter<'a, UART1_LCR_SPEC, O>;
#[doc = "Field `SPE` reader - SPE"]
pub type SPE_R = crate::BitReader;
#[doc = "Field `SPE` writer - SPE"]
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, UART1_LCR_SPEC, O>;
#[doc = "Field `BCB` reader - BCB"]
pub type BCB_R = crate::BitReader;
#[doc = "Field `BCB` writer - BCB"]
pub type BCB_W<'a, const O: u8> = crate::BitWriter<'a, UART1_LCR_SPEC, O>;
impl R {
    #[doc = "Bits 0:1 - WLS"]
    #[inline(always)]
    pub fn wls(&self) -> WLS_R {
        WLS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - NSB"]
    #[inline(always)]
    pub fn nsb(&self) -> NSB_R {
        NSB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PBE"]
    #[inline(always)]
    pub fn pbe(&self) -> PBE_R {
        PBE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EPE"]
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - BCB"]
    #[inline(always)]
    pub fn bcb(&self) -> BCB_R {
        BCB_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - WLS"]
    #[inline(always)]
    #[must_use]
    pub fn wls(&mut self) -> WLS_W<0> {
        WLS_W::new(self)
    }
    #[doc = "Bit 2 - NSB"]
    #[inline(always)]
    #[must_use]
    pub fn nsb(&mut self) -> NSB_W<2> {
        NSB_W::new(self)
    }
    #[doc = "Bit 3 - PBE"]
    #[inline(always)]
    #[must_use]
    pub fn pbe(&mut self) -> PBE_W<3> {
        PBE_W::new(self)
    }
    #[doc = "Bit 4 - EPE"]
    #[inline(always)]
    #[must_use]
    pub fn epe(&mut self) -> EPE_W<4> {
        EPE_W::new(self)
    }
    #[doc = "Bit 5 - SPE"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<5> {
        SPE_W::new(self)
    }
    #[doc = "Bit 6 - BCB"]
    #[inline(always)]
    #[must_use]
    pub fn bcb(&mut self) -> BCB_W<6> {
        BCB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART1_LCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_lcr](index.html) module"]
pub struct UART1_LCR_SPEC;
impl crate::RegisterSpec for UART1_LCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart1_lcr::R](R) reader structure"]
impl crate::Readable for UART1_LCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart1_lcr::W](W) writer structure"]
impl crate::Writable for UART1_LCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART1_LCR to value 0"]
impl crate::Resettable for UART1_LCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
