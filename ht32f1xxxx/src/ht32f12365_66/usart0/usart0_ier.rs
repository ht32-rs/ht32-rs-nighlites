#[doc = "Register `USART0_IER` reader"]
pub struct R(crate::R<USART0_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART0_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART0_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART0_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART0_IER` writer"]
pub struct W(crate::W<USART0_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART0_IER_SPEC>;
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
impl From<crate::W<USART0_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART0_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFTLI_RTOIE` reader - RFTLI_RTOIE"]
pub type RFTLI_RTOIE_R = crate::BitReader;
#[doc = "Field `RFTLI_RTOIE` writer - RFTLI_RTOIE"]
pub type RFTLI_RTOIE_W<'a, const O: u8> = crate::BitWriter<'a, USART0_IER_SPEC, O>;
#[doc = "Field `TFTLIE` reader - TFTLIE"]
pub type TFTLIE_R = crate::BitReader;
#[doc = "Field `TFTLIE` writer - TFTLIE"]
pub type TFTLIE_W<'a, const O: u8> = crate::BitWriter<'a, USART0_IER_SPEC, O>;
#[doc = "Field `RLSIE` reader - RLSIE"]
pub type RLSIE_R = crate::BitReader;
#[doc = "Field `RLSIE` writer - RLSIE"]
pub type RLSIE_W<'a, const O: u8> = crate::BitWriter<'a, USART0_IER_SPEC, O>;
#[doc = "Field `MODSIE` reader - MODSIE"]
pub type MODSIE_R = crate::BitReader;
#[doc = "Field `MODSIE` writer - MODSIE"]
pub type MODSIE_W<'a, const O: u8> = crate::BitWriter<'a, USART0_IER_SPEC, O>;
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
    #[doc = "Bit 3 - MODSIE"]
    #[inline(always)]
    pub fn modsie(&self) -> MODSIE_R {
        MODSIE_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bit 3 - MODSIE"]
    #[inline(always)]
    #[must_use]
    pub fn modsie(&mut self) -> MODSIE_W<3> {
        MODSIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART0_IER\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart0_ier](index.html) module"]
pub struct USART0_IER_SPEC;
impl crate::RegisterSpec for USART0_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart0_ier::R](R) reader structure"]
impl crate::Readable for USART0_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart0_ier::W](W) writer structure"]
impl crate::Writable for USART0_IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART0_IER to value 0"]
impl crate::Resettable for USART0_IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
