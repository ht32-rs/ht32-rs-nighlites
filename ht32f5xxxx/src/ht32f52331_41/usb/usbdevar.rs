#[doc = "Register `USBDEVAR` reader"]
pub struct R(crate::R<USBDEVAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBDEVAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBDEVAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBDEVAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBDEVAR` writer"]
pub struct W(crate::W<USBDEVAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBDEVAR_SPEC>;
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
impl From<crate::W<USBDEVAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBDEVAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVA` reader - DEVA"]
pub type DEVA_R = crate::FieldReader;
#[doc = "Field `DEVA` writer - DEVA"]
pub type DEVA_W<'a, const O: u8> = crate::FieldWriter<'a, USBDEVAR_SPEC, 7, O>;
impl R {
    #[doc = "Bits 0:6 - DEVA"]
    #[inline(always)]
    pub fn deva(&self) -> DEVA_R {
        DEVA_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - DEVA"]
    #[inline(always)]
    #[must_use]
    pub fn deva(&mut self) -> DEVA_W<0> {
        DEVA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBDEVAR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbdevar](index.html) module"]
pub struct USBDEVAR_SPEC;
impl crate::RegisterSpec for USBDEVAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbdevar::R](R) reader structure"]
impl crate::Readable for USBDEVAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbdevar::W](W) writer structure"]
impl crate::Writable for USBDEVAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBDEVAR to value 0"]
impl crate::Resettable for USBDEVAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
