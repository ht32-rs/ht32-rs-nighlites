#[doc = "Register `CKCU_HSIATCR` reader"]
pub struct R(crate::R<CKCU_HSIATCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKCU_HSIATCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKCU_HSIATCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKCU_HSIATCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKCU_HSIATCR` writer"]
pub struct W(crate::W<CKCU_HSIATCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKCU_HSIATCR_SPEC>;
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
impl From<crate::W<CKCU_HSIATCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKCU_HSIATCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATCNT` reader - ATCNT"]
pub type ATCNT_R = crate::FieldReader<u16>;
#[doc = "Field `ATCNT` writer - ATCNT"]
pub type ATCNT_W<'a, const O: u8> = crate::FieldWriter<'a, CKCU_HSIATCR_SPEC, 14, O, u16>;
impl R {
    #[doc = "Bits 0:13 - ATCNT"]
    #[inline(always)]
    pub fn atcnt(&self) -> ATCNT_R {
        ATCNT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - ATCNT"]
    #[inline(always)]
    #[must_use]
    pub fn atcnt(&mut self) -> ATCNT_W<0> {
        ATCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CKCU_HSIATCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckcu_hsiatcr](index.html) module"]
pub struct CKCU_HSIATCR_SPEC;
impl crate::RegisterSpec for CKCU_HSIATCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckcu_hsiatcr::R](R) reader structure"]
impl crate::Readable for CKCU_HSIATCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckcu_hsiatcr::W](W) writer structure"]
impl crate::Writable for CKCU_HSIATCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CKCU_HSIATCR to value 0"]
impl crate::Resettable for CKCU_HSIATCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
