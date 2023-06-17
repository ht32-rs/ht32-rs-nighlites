#[doc = "Register `PADOUTR` reader"]
pub struct R(crate::R<PADOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADOUTR` writer"]
pub struct W(crate::W<PADOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADOUTR_SPEC>;
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
impl From<crate::W<PADOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADOUT` reader - PADOUT"]
pub type PADOUT_R = crate::FieldReader<u16>;
#[doc = "Field `PADOUT` writer - PADOUT"]
pub type PADOUT_W<'a, const O: u8> = crate::FieldWriter<'a, PADOUTR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PADOUT"]
    #[inline(always)]
    pub fn padout(&self) -> PADOUT_R {
        PADOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PADOUT"]
    #[inline(always)]
    #[must_use]
    pub fn padout(&mut self) -> PADOUT_W<0> {
        PADOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PADOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padoutr](index.html) module"]
pub struct PADOUTR_SPEC;
impl crate::RegisterSpec for PADOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padoutr::R](R) reader structure"]
impl crate::Readable for PADOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padoutr::W](W) writer structure"]
impl crate::Writable for PADOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PADOUTR to value 0"]
impl crate::Resettable for PADOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
