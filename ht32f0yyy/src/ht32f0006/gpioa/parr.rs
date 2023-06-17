#[doc = "Register `PARR` reader"]
pub struct R(crate::R<PARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PARR` writer"]
pub struct W(crate::W<PARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PARR_SPEC>;
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
impl From<crate::W<PARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARST` reader - PARST"]
pub type PARST_R = crate::FieldReader<u16>;
#[doc = "Field `PARST` writer - PARST"]
pub type PARST_W<'a, const O: u8> = crate::FieldWriter<'a, PARR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PARST"]
    #[inline(always)]
    pub fn parst(&self) -> PARST_R {
        PARST_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PARST"]
    #[inline(always)]
    #[must_use]
    pub fn parst(&mut self) -> PARST_W<0> {
        PARST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PARR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parr](index.html) module"]
pub struct PARR_SPEC;
impl crate::RegisterSpec for PARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parr::R](R) reader structure"]
impl crate::Readable for PARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [parr::W](W) writer structure"]
impl crate::Writable for PARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PARR to value 0"]
impl crate::Resettable for PARR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
