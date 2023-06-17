#[doc = "Register `UART0_IER` reader"]
pub struct R(crate::R<UART0_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART0_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART0_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART0_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART0_IER` writer"]
pub struct W(crate::W<UART0_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART0_IER_SPEC>;
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
impl From<crate::W<UART0_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART0_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFTLI_RTOIE` reader - RFTLI_RTOIE"]
pub type RFTLI_RTOIE_R = crate::BitReader;
#[doc = "Field `RFTLI_RTOIE` writer - RFTLI_RTOIE"]
pub type RFTLI_RTOIE_W<'a, const O: u8> = crate::BitWriter<'a, UART0_IER_SPEC, O>;
#[doc = "Field `TFTLIE` reader - TFTLIE"]
pub type TFTLIE_R = crate::BitReader;
#[doc = "Field `TFTLIE` writer - TFTLIE"]
pub type TFTLIE_W<'a, const O: u8> = crate::BitWriter<'a, UART0_IER_SPEC, O>;
#[doc = "Field `RLSIE` reader - RLSIE"]
pub type RLSIE_R = crate::BitReader;
#[doc = "Field `RLSIE` writer - RLSIE"]
pub type RLSIE_W<'a, const O: u8> = crate::BitWriter<'a, UART0_IER_SPEC, O>;
impl R {
    #[doc = "Bit 0 - RFTLI_RTOIE"]
    #[inline(always)]
    pub fn rftli_rtoie(&self) -> RFTLI_RTOIE_R {
        RFTLI_RTOIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFTLIE"]
    #[inline(always)]
    pub fn tftlie(&self) -> TFTLIE_R {
        TFTLIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RLSIE"]
    #[inline(always)]
    pub fn rlsie(&self) -> RLSIE_R {
        RLSIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RFTLI_RTOIE"]
    #[inline(always)]
    #[must_use]
    pub fn rftli_rtoie(&mut self) -> RFTLI_RTOIE_W<0> {
        RFTLI_RTOIE_W::new(self)
    }
    #[doc = "Bit 1 - TFTLIE"]
    #[inline(always)]
    #[must_use]
    pub fn tftlie(&mut self) -> TFTLIE_W<1> {
        TFTLIE_W::new(self)
    }
    #[doc = "Bit 2 - RLSIE"]
    #[inline(always)]
    #[must_use]
    pub fn rlsie(&mut self) -> RLSIE_W<2> {
        RLSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_ier](index.html) module"]
pub struct UART0_IER_SPEC;
impl crate::RegisterSpec for UART0_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart0_ier::R](R) reader structure"]
impl crate::Readable for UART0_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart0_ier::W](W) writer structure"]
impl crate::Writable for UART0_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART0_IER to value 0"]
impl crate::Resettable for UART0_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
