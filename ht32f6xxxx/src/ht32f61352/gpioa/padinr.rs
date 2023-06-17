#[doc = "Register `PADINR` reader"]
pub struct R(crate::R<PADINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADINR` writer"]
pub struct W(crate::W<PADINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADINR_SPEC>;
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
impl From<crate::W<PADINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADIN` reader - PADIN"]
pub type PADIN_R = crate::FieldReader<u16>;
#[doc = "Field `PADIN` writer - PADIN"]
pub type PADIN_W<'a, const O: u8> = crate::FieldWriter<'a, PADINR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PADIN"]
    #[inline(always)]
    pub fn padin(&self) -> PADIN_R {
        PADIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PADIN"]
    #[inline(always)]
    #[must_use]
    pub fn padin(&mut self) -> PADIN_W<0> {
        PADIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PADINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padinr](index.html) module"]
pub struct PADINR_SPEC;
impl crate::RegisterSpec for PADINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padinr::R](R) reader structure"]
impl crate::Readable for PADINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padinr::W](W) writer structure"]
impl crate::Writable for PADINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADINR to value 0"]
impl crate::Resettable for PADINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
