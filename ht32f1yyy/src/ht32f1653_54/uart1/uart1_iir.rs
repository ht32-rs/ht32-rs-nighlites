#[doc = "Register `UART1_IIR` reader"]
pub struct R(crate::R<UART1_IIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART1_IIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART1_IIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART1_IIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART1_IIR` writer"]
pub struct W(crate::W<UART1_IIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART1_IIR_SPEC>;
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
impl From<crate::W<UART1_IIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART1_IIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NIP` reader - NIP"]
pub type NIP_R = crate::BitReader;
#[doc = "Field `NIP` writer - NIP"]
pub type NIP_W<'a, const O: u8> = crate::BitWriter<'a, UART1_IIR_SPEC, O>;
#[doc = "Field `IID` reader - IID"]
pub type IID_R = crate::FieldReader;
#[doc = "Field `IID` writer - IID"]
pub type IID_W<'a, const O: u8> = crate::FieldWriter<'a, UART1_IIR_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - NIP"]
    #[inline(always)]
    pub fn nip(&self) -> NIP_R {
        NIP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - IID"]
    #[inline(always)]
    pub fn iid(&self) -> IID_R {
        IID_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - NIP"]
    #[inline(always)]
    #[must_use]
    pub fn nip(&mut self) -> NIP_W<0> {
        NIP_W::new(self)
    }
    #[doc = "Bits 1:3 - IID"]
    #[inline(always)]
    #[must_use]
    pub fn iid(&mut self) -> IID_W<1> {
        IID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART1_IIR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart1_iir](index.html) module"]
pub struct UART1_IIR_SPEC;
impl crate::RegisterSpec for UART1_IIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart1_iir::R](R) reader structure"]
impl crate::Readable for UART1_IIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart1_iir::W](W) writer structure"]
impl crate::Writable for UART1_IIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UART1_IIR to value 0"]
impl crate::Resettable for UART1_IIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
