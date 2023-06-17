#[doc = "Register `PCODR` reader"]
pub struct R(crate::R<PCODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCODR` writer"]
pub struct W(crate::W<PCODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCODR_SPEC>;
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
impl From<crate::W<PCODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCOD` reader - PCOD"]
pub type PCOD_R = crate::FieldReader<u16>;
#[doc = "Field `PCOD` writer - PCOD"]
pub type PCOD_W<'a, const O: u8> = crate::FieldWriter<'a, PCODR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCOD"]
    #[inline(always)]
    pub fn pcod(&self) -> PCOD_R {
        PCOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCOD"]
    #[inline(always)]
    #[must_use]
    pub fn pcod(&mut self) -> PCOD_W<0> {
        PCOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcodr](index.html) module"]
pub struct PCODR_SPEC;
impl crate::RegisterSpec for PCODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcodr::R](R) reader structure"]
impl crate::Readable for PCODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcodr::W](W) writer structure"]
impl crate::Writable for PCODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCODR to value 0"]
impl crate::Resettable for PCODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
