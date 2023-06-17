#[doc = "Register `PFDRVR` reader"]
pub struct R(crate::R<PFDRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFDRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFDRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFDRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFDRVR` writer"]
pub struct W(crate::W<PFDRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFDRVR_SPEC>;
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
impl From<crate::W<PFDRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFDRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PFDV0` reader - PFDV0"]
pub type PFDV0_R = crate::FieldReader;
#[doc = "Field `PFDV0` writer - PFDV0"]
pub type PFDV0_W<'a, const O: u8> = crate::FieldWriter<'a, PFDRVR_SPEC, 2, O>;
#[doc = "Field `PFDV1` reader - PFDV1"]
pub type PFDV1_R = crate::FieldReader;
#[doc = "Field `PFDV1` writer - PFDV1"]
pub type PFDV1_W<'a, const O: u8> = crate::FieldWriter<'a, PFDRVR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PFDV0"]
    #[inline(always)]
    pub fn pfdv0(&self) -> PFDV0_R {
        PFDV0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PFDV1"]
    #[inline(always)]
    pub fn pfdv1(&self) -> PFDV1_R {
        PFDV1_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PFDV0"]
    #[inline(always)]
    #[must_use]
    pub fn pfdv0(&mut self) -> PFDV0_W<0> {
        PFDV0_W::new(self)
    }
    #[doc = "Bits 2:3 - PFDV1"]
    #[inline(always)]
    #[must_use]
    pub fn pfdv1(&mut self) -> PFDV1_W<2> {
        PFDV1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PFDRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfdrvr](index.html) module"]
pub struct PFDRVR_SPEC;
impl crate::RegisterSpec for PFDRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfdrvr::R](R) reader structure"]
impl crate::Readable for PFDRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfdrvr::W](W) writer structure"]
impl crate::Writable for PFDRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFDRVR to value 0"]
impl crate::Resettable for PFDRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
