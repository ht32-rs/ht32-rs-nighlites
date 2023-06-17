#[doc = "Register `USART1_MODCR` reader"]
pub struct R(crate::R<USART1_MODCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART1_MODCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART1_MODCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART1_MODCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART1_MODCR` writer"]
pub struct W(crate::W<USART1_MODCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART1_MODCR_SPEC>;
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
impl From<crate::W<USART1_MODCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART1_MODCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTR` reader - DTR"]
pub type DTR_R = crate::BitReader;
#[doc = "Field `DTR` writer - DTR"]
pub type DTR_W<'a, const O: u8> = crate::BitWriter<'a, USART1_MODCR_SPEC, O>;
#[doc = "Field `RTS` reader - RTS"]
pub type RTS_R = crate::BitReader;
#[doc = "Field `RTS` writer - RTS"]
pub type RTS_W<'a, const O: u8> = crate::BitWriter<'a, USART1_MODCR_SPEC, O>;
#[doc = "Field `HFCEN` reader - HFCEN"]
pub type HFCEN_R = crate::BitReader;
#[doc = "Field `HFCEN` writer - HFCEN"]
pub type HFCEN_W<'a, const O: u8> = crate::BitWriter<'a, USART1_MODCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - DTR"]
    #[inline(always)]
    pub fn dtr(&self) -> DTR_R {
        DTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTS"]
    #[inline(always)]
    pub fn rts(&self) -> RTS_R {
        RTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HFCEN"]
    #[inline(always)]
    pub fn hfcen(&self) -> HFCEN_R {
        HFCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTR"]
    #[inline(always)]
    #[must_use]
    pub fn dtr(&mut self) -> DTR_W<0> {
        DTR_W::new(self)
    }
    #[doc = "Bit 1 - RTS"]
    #[inline(always)]
    #[must_use]
    pub fn rts(&mut self) -> RTS_W<1> {
        RTS_W::new(self)
    }
    #[doc = "Bit 2 - HFCEN"]
    #[inline(always)]
    #[must_use]
    pub fn hfcen(&mut self) -> HFCEN_W<2> {
        HFCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART1_MODCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_modcr](index.html) module"]
pub struct USART1_MODCR_SPEC;
impl crate::RegisterSpec for USART1_MODCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart1_modcr::R](R) reader structure"]
impl crate::Readable for USART1_MODCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart1_modcr::W](W) writer structure"]
impl crate::Writable for USART1_MODCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART1_MODCR to value 0"]
impl crate::Resettable for USART1_MODCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
