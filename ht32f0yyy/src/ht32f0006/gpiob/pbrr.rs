#[doc = "Register `PBRR` reader"]
pub struct R(crate::R<PBRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBRR` writer"]
pub struct W(crate::W<PBRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBRR_SPEC>;
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
impl From<crate::W<PBRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBRST` reader - PBRST"]
pub type PBRST_R = crate::FieldReader<u16>;
#[doc = "Field `PBRST` writer - PBRST"]
pub type PBRST_W<'a, const O: u8> = crate::FieldWriter<'a, PBRR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PBRST"]
    #[inline(always)]
    pub fn pbrst(&self) -> PBRST_R {
        PBRST_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PBRST"]
    #[inline(always)]
    #[must_use]
    pub fn pbrst(&mut self) -> PBRST_W<0> {
        PBRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbrr](index.html) module"]
pub struct PBRR_SPEC;
impl crate::RegisterSpec for PBRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbrr::R](R) reader structure"]
impl crate::Readable for PBRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbrr::W](W) writer structure"]
impl crate::Writable for PBRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBRR to value 0"]
impl crate::Resettable for PBRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
