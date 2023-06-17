#[doc = "Register `USART1_TBR` reader"]
pub struct R(crate::R<USART1_USART1_TBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART1_USART1_TBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART1_USART1_TBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART1_USART1_TBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART1_TBR` writer"]
pub struct W(crate::W<USART1_USART1_TBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART1_USART1_TBR_SPEC>;
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
impl From<crate::W<USART1_USART1_TBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART1_USART1_TBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TD` reader - TD"]
pub type TD_R = crate::FieldReader<u16>;
#[doc = "Field `TD` writer - TD"]
pub type TD_W<'a, const O: u8> = crate::FieldWriter<'a, USART1_USART1_TBR_SPEC, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - TD"]
    #[inline(always)]
    pub fn td(&self) -> TD_R {
        TD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TD"]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TD_W<0> {
        TD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART1_TBR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_usart1_tbr](index.html) module"]
pub struct USART1_USART1_TBR_SPEC;
impl crate::RegisterSpec for USART1_USART1_TBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart1_usart1_tbr::R](R) reader structure"]
impl crate::Readable for USART1_USART1_TBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart1_usart1_tbr::W](W) writer structure"]
impl crate::Writable for USART1_USART1_TBR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USART1_TBR to value 0"]
impl crate::Resettable for USART1_USART1_TBR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
