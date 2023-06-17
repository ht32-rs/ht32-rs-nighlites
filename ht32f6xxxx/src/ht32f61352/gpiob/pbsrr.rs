#[doc = "Register `PBSRR` reader"]
pub struct R(crate::R<PBSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBSRR` writer"]
pub struct W(crate::W<PBSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBSRR_SPEC>;
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
impl From<crate::W<PBSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBSET` reader - PBSET"]
pub type PBSET_R = crate::FieldReader<u16>;
#[doc = "Field `PBSET` writer - PBSET"]
pub type PBSET_W<'a, const O: u8> = crate::FieldWriter<'a, PBSRR_SPEC, 16, O, u16>;
#[doc = "Field `PBRST` reader - PBRST"]
pub type PBRST_R = crate::FieldReader<u16>;
#[doc = "Field `PBRST` writer - PBRST"]
pub type PBRST_W<'a, const O: u8> = crate::FieldWriter<'a, PBSRR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PBSET"]
    #[inline(always)]
    pub fn pbset(&self) -> PBSET_R {
        PBSET_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PBRST"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PBSET"]
    #[inline(always)]
    #[must_use]
    pub fn pbset(&mut self) -> PBSET_W<0> {
        PBSET_W::new(self)
    }
    #[doc = "Bits 16:31 - PBRST"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PBRST_W<16> {
        PBRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBSRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbsrr](index.html) module"]
pub struct PBSRR_SPEC;
impl crate::RegisterSpec for PBSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbsrr::R](R) reader structure"]
impl crate::Readable for PBSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbsrr::W](W) writer structure"]
impl crate::Writable for PBSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBSRR to value 0"]
impl crate::Resettable for PBSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
