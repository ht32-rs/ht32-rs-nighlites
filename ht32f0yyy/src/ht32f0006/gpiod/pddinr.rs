#[doc = "Register `PDDINR` reader"]
pub struct R(crate::R<PDDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDINR` writer"]
pub struct W(crate::W<PDDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDINR_SPEC>;
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
impl From<crate::W<PDDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDIN` reader - PDDIN"]
pub type PDDIN_R = crate::FieldReader;
#[doc = "Field `PDDIN` writer - PDDIN"]
pub type PDDIN_W<'a, const O: u8> = crate::FieldWriter<'a, PDDINR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDDIN"]
    #[inline(always)]
    pub fn pddin(&self) -> PDDIN_R {
        PDDIN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDDIN"]
    #[inline(always)]
    #[must_use]
    pub fn pddin(&mut self) -> PDDIN_W<0> {
        PDDIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddinr](index.html) module"]
pub struct PDDINR_SPEC;
impl crate::RegisterSpec for PDDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddinr::R](R) reader structure"]
impl crate::Readable for PDDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddinr::W](W) writer structure"]
impl crate::Writable for PDDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDINR to value 0"]
impl crate::Resettable for PDDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
