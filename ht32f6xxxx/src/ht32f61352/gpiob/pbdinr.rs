#[doc = "Register `PBDINR` reader"]
pub struct R(crate::R<PBDINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDINR` writer"]
pub struct W(crate::W<PBDINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDINR_SPEC>;
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
impl From<crate::W<PBDINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBDIN` reader - PBDIN"]
pub type PBDIN_R = crate::FieldReader<u16>;
#[doc = "Field `PBDIN` writer - PBDIN"]
pub type PBDIN_W<'a, const O: u8> = crate::FieldWriter<'a, PBDINR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PBDIN"]
    #[inline(always)]
    pub fn pbdin(&self) -> PBDIN_R {
        PBDIN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PBDIN"]
    #[inline(always)]
    #[must_use]
    pub fn pbdin(&mut self) -> PBDIN_W<0> {
        PBDIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBDINR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdinr](index.html) module"]
pub struct PBDINR_SPEC;
impl crate::RegisterSpec for PBDINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbdinr::R](R) reader structure"]
impl crate::Readable for PBDINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdinr::W](W) writer structure"]
impl crate::Writable for PBDINR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBDINR to value 0"]
impl crate::Resettable for PBDINR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
