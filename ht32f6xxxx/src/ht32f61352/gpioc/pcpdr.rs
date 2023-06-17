#[doc = "Register `PCPDR` reader"]
pub struct R(crate::R<PCPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCPDR` writer"]
pub struct W(crate::W<PCPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCPDR_SPEC>;
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
impl From<crate::W<PCPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCPD` reader - PCPD"]
pub type PCPD_R = crate::FieldReader<u16>;
#[doc = "Field `PCPD` writer - PCPD"]
pub type PCPD_W<'a, const O: u8> = crate::FieldWriter<'a, PCPDR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCPD"]
    #[inline(always)]
    pub fn pcpd(&self) -> PCPD_R {
        PCPD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCPD"]
    #[inline(always)]
    #[must_use]
    pub fn pcpd(&mut self) -> PCPD_W<0> {
        PCPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcpdr](index.html) module"]
pub struct PCPDR_SPEC;
impl crate::RegisterSpec for PCPDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcpdr::R](R) reader structure"]
impl crate::Readable for PCPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcpdr::W](W) writer structure"]
impl crate::Writable for PCPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCPDR to value 0"]
impl crate::Resettable for PCPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
