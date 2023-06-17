#[doc = "Register `USART_RBR` reader"]
pub struct R(crate::R<USART_RBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART_RBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART_RBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART_RBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART_RBR` writer"]
pub struct W(crate::W<USART_RBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART_RBR_SPEC>;
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
impl From<crate::W<USART_RBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART_RBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RD` reader - RD"]
pub type RD_R = crate::FieldReader<u16>;
#[doc = "Field `RD` writer - RD"]
pub type RD_W<'a, const O: u8> = crate::FieldWriter<'a, USART_RBR_SPEC, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - RD"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - RD"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<0> {
        RD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_RBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart_rbr](index.html) module"]
pub struct USART_RBR_SPEC;
impl crate::RegisterSpec for USART_RBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart_rbr::R](R) reader structure"]
impl crate::Readable for USART_RBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart_rbr::W](W) writer structure"]
impl crate::Writable for USART_RBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART_RBR to value 0"]
impl crate::Resettable for USART_RBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
