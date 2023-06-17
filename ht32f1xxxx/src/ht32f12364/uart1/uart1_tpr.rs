#[doc = "Register `UART1_TPR` reader"]
pub struct R(crate::R<UART1_TPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART1_TPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART1_TPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART1_TPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART1_TPR` writer"]
pub struct W(crate::W<UART1_TPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART1_TPR_SPEC>;
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
impl From<crate::W<UART1_TPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART1_TPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTOIC` reader - RTOIC"]
pub type RTOIC_R = crate::FieldReader;
#[doc = "Field `RTOIC` writer - RTOIC"]
pub type RTOIC_W<'a, const O: u8> = crate::FieldWriter<'a, UART1_TPR_SPEC, 7, O>;
#[doc = "Field `RTOIE` reader - RTOIE"]
pub type RTOIE_R = crate::BitReader;
#[doc = "Field `RTOIE` writer - RTOIE"]
pub type RTOIE_W<'a, const O: u8> = crate::BitWriter<'a, UART1_TPR_SPEC, O>;
#[doc = "Field `TG` reader - TG"]
pub type TG_R = crate::FieldReader;
#[doc = "Field `TG` writer - TG"]
pub type TG_W<'a, const O: u8> = crate::FieldWriter<'a, UART1_TPR_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:6 - RTOIC"]
    #[inline(always)]
    pub fn rtoic(&self) -> RTOIC_R {
        RTOIC_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RTOIE"]
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - RTOIC"]
    #[inline(always)]
    #[must_use]
    pub fn rtoic(&mut self) -> RTOIC_W<0> {
        RTOIC_W::new(self)
    }
    #[doc = "Bit 7 - RTOIE"]
    #[inline(always)]
    #[must_use]
    pub fn rtoie(&mut self) -> RTOIE_W<7> {
        RTOIE_W::new(self)
    }
    #[doc = "Bits 8:15 - TG"]
    #[inline(always)]
    #[must_use]
    pub fn tg(&mut self) -> TG_W<8> {
        TG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART1_TPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_tpr](index.html) module"]
pub struct UART1_TPR_SPEC;
impl crate::RegisterSpec for UART1_TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart1_tpr::R](R) reader structure"]
impl crate::Readable for UART1_TPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart1_tpr::W](W) writer structure"]
impl crate::Writable for UART1_TPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART1_TPR to value 0"]
impl crate::Resettable for UART1_TPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
