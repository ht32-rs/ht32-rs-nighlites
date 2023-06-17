#[doc = "Register `PCDOUTR` reader"]
pub struct R(crate::R<PCDOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDOUTR` writer"]
pub struct W(crate::W<PCDOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDOUTR_SPEC>;
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
impl From<crate::W<PCDOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCDOUT` reader - PCDOUT"]
pub type PCDOUT_R = crate::FieldReader<u16>;
#[doc = "Field `PCDOUT` writer - PCDOUT"]
pub type PCDOUT_W<'a, const O: u8> = crate::FieldWriter<'a, PCDOUTR_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - PCDOUT"]
    #[inline(always)]
    pub fn pcdout(&self) -> PCDOUT_R {
        PCDOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PCDOUT"]
    #[inline(always)]
    #[must_use]
    pub fn pcdout(&mut self) -> PCDOUT_W<0> {
        PCDOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PCDOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdoutr](index.html) module"]
pub struct PCDOUTR_SPEC;
impl crate::RegisterSpec for PCDOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcdoutr::R](R) reader structure"]
impl crate::Readable for PCDOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdoutr::W](W) writer structure"]
impl crate::Writable for PCDOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCDOUTR to value 0"]
impl crate::Resettable for PCDOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
