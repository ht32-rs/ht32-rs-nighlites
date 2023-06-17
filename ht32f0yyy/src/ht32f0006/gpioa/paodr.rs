#[doc = "Register `PAODR` reader"]
pub struct R(crate::R<PAODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAODR` writer"]
pub struct W(crate::W<PAODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAODR_SPEC>;
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
impl From<crate::W<PAODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAOD` reader - PAOD"]
pub type PAOD_R = crate::FieldReader<u16>;
#[doc = "Field `PAOD` writer - PAOD"]
pub type PAOD_W<'a, const O: u8> = crate::FieldWriter<'a, PAODR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PAOD"]
    #[inline(always)]
    pub fn paod(&self) -> PAOD_R {
        PAOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PAOD"]
    #[inline(always)]
    #[must_use]
    pub fn paod(&mut self) -> PAOD_W<0> {
        PAOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PAODR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paodr](index.html) module"]
pub struct PAODR_SPEC;
impl crate::RegisterSpec for PAODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [paodr::R](R) reader structure"]
impl crate::Readable for PAODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paodr::W](W) writer structure"]
impl crate::Writable for PAODR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PAODR to value 0"]
impl crate::Resettable for PAODR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
