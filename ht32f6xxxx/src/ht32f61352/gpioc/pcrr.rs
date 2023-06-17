#[doc = "Register `PCRR` reader"]
pub struct R(crate::R<PCRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCRR` writer"]
pub struct W(crate::W<PCRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCRR_SPEC>;
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
impl From<crate::W<PCRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCRST` reader - PCRST"]
pub type PCRST_R = crate::FieldReader<u16>;
#[doc = "Field `PCRST` writer - PCRST"]
pub type PCRST_W<'a, const O: u8> = crate::FieldWriter<'a, PCRR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCRST"]
    #[inline(always)]
    pub fn pcrst(&self) -> PCRST_R {
        PCRST_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCRST"]
    #[inline(always)]
    #[must_use]
    pub fn pcrst(&mut self) -> PCRST_W<0> {
        PCRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcrr](index.html) module"]
pub struct PCRR_SPEC;
impl crate::RegisterSpec for PCRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcrr::R](R) reader structure"]
impl crate::Readable for PCRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcrr::W](W) writer structure"]
impl crate::Writable for PCRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCRR to value 0"]
impl crate::Resettable for PCRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
