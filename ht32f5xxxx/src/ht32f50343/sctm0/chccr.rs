#[doc = "Register `CHCCR` reader"]
pub struct R(crate::R<CHCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCCR` writer"]
pub struct W(crate::W<CHCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCCR_SPEC>;
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
impl From<crate::W<CHCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHCCV` reader - CHCCV"]
pub type CHCCV_R = crate::FieldReader<u16>;
#[doc = "Field `CHCCV` writer - CHCCV"]
pub type CHCCV_W<'a, const O: u8> = crate::FieldWriter<'a, CHCCR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - CHCCV"]
    #[inline(always)]
    pub fn chccv(&self) -> CHCCV_R {
        CHCCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CHCCV"]
    #[inline(always)]
    #[must_use]
    pub fn chccv(&mut self) -> CHCCV_W<0> {
        CHCCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHCCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chccr](index.html) module"]
pub struct CHCCR_SPEC;
impl crate::RegisterSpec for CHCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chccr::R](R) reader structure"]
impl crate::Readable for CHCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chccr::W](W) writer structure"]
impl crate::Writable for CHCCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCCR to value 0"]
impl crate::Resettable for CHCCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
