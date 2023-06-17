#[doc = "Register `PCLOCKR` reader"]
pub struct R(crate::R<PCLOCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCLOCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCLOCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCLOCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCLOCKR` writer"]
pub struct W(crate::W<PCLOCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCLOCKR_SPEC>;
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
impl From<crate::W<PCLOCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCLOCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCLOCK` reader - PCLOCK"]
pub type PCLOCK_R = crate::FieldReader<u16>;
#[doc = "Field `PCLOCK` writer - PCLOCK"]
pub type PCLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, PCLOCKR_SPEC, 16, O, u16>;
#[doc = "Field `PCLKEY` reader - PCLKEY"]
pub type PCLKEY_R = crate::FieldReader<u16>;
#[doc = "Field `PCLKEY` writer - PCLKEY"]
pub type PCLKEY_W<'a, const O: u8> = crate::FieldWriter<'a, PCLOCKR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCLOCK"]
    #[inline(always)]
    pub fn pclock(&self) -> PCLOCK_R {
        PCLOCK_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PCLKEY"]
    #[inline(always)]
    pub fn pclkey(&self) -> PCLKEY_R {
        PCLKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCLOCK"]
    #[inline(always)]
    #[must_use]
    pub fn pclock(&mut self) -> PCLOCK_W<0> {
        PCLOCK_W::new(self)
    }
    #[doc = "Bits 16:31 - PCLKEY"]
    #[inline(always)]
    #[must_use]
    pub fn pclkey(&mut self) -> PCLKEY_W<16> {
        PCLKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCLOCKR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pclockr](index.html) module"]
pub struct PCLOCKR_SPEC;
impl crate::RegisterSpec for PCLOCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pclockr::R](R) reader structure"]
impl crate::Readable for PCLOCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pclockr::W](W) writer structure"]
impl crate::Writable for PCLOCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCLOCKR to value 0"]
impl crate::Resettable for PCLOCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
