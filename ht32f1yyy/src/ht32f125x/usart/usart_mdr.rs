#[doc = "Register `USART_MDR` reader"]
pub struct R(crate::R<USART_MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_MDR` writer"]
pub struct W(crate::W<USART_MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_MDR_SPEC>;
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
impl From<crate::W<USART_MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - MODE"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - MODE"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, USART_MDR_SPEC, 2, O>;
#[doc = "Field `TRSM` reader - TRSM"]
pub type TRSM_R = crate::BitReader;
#[doc = "Field `TRSM` writer - TRSM"]
pub type TRSM_W<'a, const O: u8> = crate::BitWriter<'a, USART_MDR_SPEC, O>;
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_MDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_mdr](index.html) module"]
pub struct USART_MDR_SPEC;
impl crate::RegisterSpec for USART_MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_mdr::R](R) reader structure"]
impl crate::Readable for USART_MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_mdr::W](W) writer structure"]
impl crate::Writable for USART_MDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_MDR to value 0"]
impl crate::Resettable for USART_MDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
