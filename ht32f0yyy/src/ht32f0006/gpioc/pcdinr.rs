#[doc = "Register `PCDINR` reader"]
pub struct R(crate::R<PCDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDINR` writer"]
pub struct W(crate::W<PCDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDINR_SPEC>;
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
impl From<crate::W<PCDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCDIN` reader - PCDIN"]
pub type PCDIN_R = crate::FieldReader<u16>;
#[doc = "Field `PCDIN` writer - PCDIN"]
pub type PCDIN_W<'a, const O: u8> = crate::FieldWriter<'a, PCDINR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCDIN"]
    #[inline(always)]
    pub fn pcdin(&self) -> PCDIN_R {
        PCDIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCDIN"]
    #[inline(always)]
    #[must_use]
    pub fn pcdin(&mut self) -> PCDIN_W<0> {
        PCDIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdinr](index.html) module"]
pub struct PCDINR_SPEC;
impl crate::RegisterSpec for PCDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcdinr::R](R) reader structure"]
impl crate::Readable for PCDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdinr::W](W) writer structure"]
impl crate::Writable for PCDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDINR to value 0"]
impl crate::Resettable for PCDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
