#[doc = "Register `PDDIRCR` reader"]
pub struct R(crate::R<PDDIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDIRCR` writer"]
pub struct W(crate::W<PDDIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDIRCR_SPEC>;
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
impl From<crate::W<PDDIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDIR` reader - PDDIR"]
pub type PDDIR_R = crate::FieldReader;
#[doc = "Field `PDDIR` writer - PDDIR"]
pub type PDDIR_W<'a, const O: u8> = crate::FieldWriter<'a, PDDIRCR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDDIR"]
    #[inline(always)]
    pub fn pddir(&self) -> PDDIR_R {
        PDDIR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDDIR"]
    #[inline(always)]
    #[must_use]
    pub fn pddir(&mut self) -> PDDIR_W<0> {
        PDDIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddircr](index.html) module"]
pub struct PDDIRCR_SPEC;
impl crate::RegisterSpec for PDDIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddircr::R](R) reader structure"]
impl crate::Readable for PDDIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddircr::W](W) writer structure"]
impl crate::Writable for PDDIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDIRCR to value 0"]
impl crate::Resettable for PDDIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
