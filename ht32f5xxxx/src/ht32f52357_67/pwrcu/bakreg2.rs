#[doc = "Register `BAKREG2` reader"]
pub struct R(crate::R<BAKREG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAKREG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAKREG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAKREG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAKREG2` writer"]
pub struct W(crate::W<BAKREG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAKREG2_SPEC>;
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
impl From<crate::W<BAKREG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAKREG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAKREG` reader - BAKREG"]
pub type BAKREG_R = crate::FieldReader<u32>;
#[doc = "Field `BAKREG` writer - BAKREG"]
pub type BAKREG_W<'a, const O: u8> = crate::FieldWriter<'a, BAKREG2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - BAKREG"]
    #[inline(always)]
    pub fn bakreg(&self) -> BAKREG_R {
        BAKREG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BAKREG"]
    #[inline(always)]
    #[must_use]
    pub fn bakreg(&mut self) -> BAKREG_W<0> {
        BAKREG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BAKREG2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bakreg2](index.html) module"]
pub struct BAKREG2_SPEC;
impl crate::RegisterSpec for BAKREG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bakreg2::R](R) reader structure"]
impl crate::Readable for BAKREG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bakreg2::W](W) writer structure"]
impl crate::Writable for BAKREG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAKREG2 to value 0"]
impl crate::Resettable for BAKREG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
