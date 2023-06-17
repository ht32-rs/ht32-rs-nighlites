#[doc = "Register `PBODR` reader"]
pub struct R(crate::R<PBODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBODR` writer"]
pub struct W(crate::W<PBODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBODR_SPEC>;
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
impl From<crate::W<PBODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBOD` reader - PBOD"]
pub type PBOD_R = crate::FieldReader<u16>;
#[doc = "Field `PBOD` writer - PBOD"]
pub type PBOD_W<'a, const O: u8> = crate::FieldWriter<'a, PBODR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PBOD"]
    #[inline(always)]
    pub fn pbod(&self) -> PBOD_R {
        PBOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PBOD"]
    #[inline(always)]
    #[must_use]
    pub fn pbod(&mut self) -> PBOD_W<0> {
        PBOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbodr](index.html) module"]
pub struct PBODR_SPEC;
impl crate::RegisterSpec for PBODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbodr::R](R) reader structure"]
impl crate::Readable for PBODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbodr::W](W) writer structure"]
impl crate::Writable for PBODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBODR to value 0"]
impl crate::Resettable for PBODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
