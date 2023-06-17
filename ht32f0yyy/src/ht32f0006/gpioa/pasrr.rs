#[doc = "Register `PASRR` reader"]
pub struct R(crate::R<PASRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PASRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PASRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PASRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PASRR` writer"]
pub struct W(crate::W<PASRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PASRR_SPEC>;
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
impl From<crate::W<PASRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PASRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PASET` reader - PASET"]
pub type PASET_R = crate::FieldReader<u16>;
#[doc = "Field `PASET` writer - PASET"]
pub type PASET_W<'a, const O: u8> = crate::FieldWriter<'a, PASRR_SPEC, 16, O, u16>;
#[doc = "Field `PARST` reader - PARST"]
pub type PARST_R = crate::FieldReader<u16>;
#[doc = "Field `PARST` writer - PARST"]
pub type PARST_W<'a, const O: u8> = crate::FieldWriter<'a, PASRR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PASET"]
    #[inline(always)]
    pub fn paset(&self) -> PASET_R {
        PASET_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PARST"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PASET"]
    #[inline(always)]
    #[must_use]
    pub fn paset(&mut self) -> PASET_W<0> {
        PASET_W::new(self)
    }
    #[doc = "Bits 16:31 - PARST"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> PARST_W<16> {
        PARST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PASRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pasrr](index.html) module"]
pub struct PASRR_SPEC;
impl crate::RegisterSpec for PASRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pasrr::R](R) reader structure"]
impl crate::Readable for PASRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pasrr::W](W) writer structure"]
impl crate::Writable for PASRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PASRR to value 0"]
impl crate::Resettable for PASRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
