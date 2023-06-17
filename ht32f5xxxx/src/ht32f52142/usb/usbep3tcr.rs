#[doc = "Register `USBEP3TCR` reader"]
pub struct R(crate::R<USBEP3TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBEP3TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBEP3TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBEP3TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBEP3TCR` writer"]
pub struct W(crate::W<USBEP3TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBEP3TCR_SPEC>;
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
impl From<crate::W<USBEP3TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBEP3TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCNT` reader - TCNT"]
pub type TCNT_R = crate::FieldReader<u16>;
#[doc = "Field `TCNT` writer - TCNT"]
pub type TCNT_W<'a, const O: u8> = crate::FieldWriter<'a, USBEP3TCR_SPEC, 9, O, u16>;
impl R {
    #[doc = "Bits 0:8 - TCNT"]
    #[inline(always)]
    pub fn tcnt(&self) -> TCNT_R {
        TCNT_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - TCNT"]
    #[inline(always)]
    #[must_use]
    pub fn tcnt(&mut self) -> TCNT_W<0> {
        TCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBEP3TCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbep3tcr](index.html) module"]
pub struct USBEP3TCR_SPEC;
impl crate::RegisterSpec for USBEP3TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbep3tcr::R](R) reader structure"]
impl crate::Readable for USBEP3TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbep3tcr::W](W) writer structure"]
impl crate::Writable for USBEP3TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBEP3TCR to value 0"]
impl crate::Resettable for USBEP3TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
