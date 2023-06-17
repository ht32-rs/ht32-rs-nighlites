#[doc = "Register `CHPOLR` reader"]
pub struct R(crate::R<CHPOLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHPOLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHPOLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHPOLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHPOLR` writer"]
pub struct W(crate::W<CHPOLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPOLR_SPEC>;
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
impl From<crate::W<CHPOLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPOLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHP` reader - CHP"]
pub type CHP_R = crate::BitReader;
#[doc = "Field `CHP` writer - CHP"]
pub type CHP_W<'a, const O: u8> = crate::BitWriter<'a, CHPOLR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CHP"]
    #[inline(always)]
    pub fn chp(&self) -> CHP_R {
        CHP_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CHP"]
    #[inline(always)]
    #[must_use]
    pub fn chp(&mut self) -> CHP_W<0> {
        CHP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CHPOLR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpolr](index.html) module"]
pub struct CHPOLR_SPEC;
impl crate::RegisterSpec for CHPOLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chpolr::R](R) reader structure"]
impl crate::Readable for CHPOLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chpolr::W](W) writer structure"]
impl crate::Writable for CHPOLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHPOLR to value 0"]
impl crate::Resettable for CHPOLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
