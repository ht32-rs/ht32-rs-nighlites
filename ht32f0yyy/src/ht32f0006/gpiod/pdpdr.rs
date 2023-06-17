#[doc = "Register `PDPDR` reader"]
pub struct R(crate::R<PDPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDPDR` writer"]
pub struct W(crate::W<PDPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDPDR_SPEC>;
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
impl From<crate::W<PDPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDPD` reader - PDPD"]
pub type PDPD_R = crate::FieldReader;
#[doc = "Field `PDPD` writer - PDPD"]
pub type PDPD_W<'a, const O: u8> = crate::FieldWriter<'a, PDPDR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDPD"]
    #[inline(always)]
    pub fn pdpd(&self) -> PDPD_R {
        PDPD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDPD"]
    #[inline(always)]
    #[must_use]
    pub fn pdpd(&mut self) -> PDPD_W<0> {
        PDPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdpdr](index.html) module"]
pub struct PDPDR_SPEC;
impl crate::RegisterSpec for PDPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdpdr::R](R) reader structure"]
impl crate::Readable for PDPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdpdr::W](W) writer structure"]
impl crate::Writable for PDPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDPDR to value 0"]
impl crate::Resettable for PDPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
