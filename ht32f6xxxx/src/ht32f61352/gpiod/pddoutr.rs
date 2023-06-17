#[doc = "Register `PDDOUTR` reader"]
pub struct R(crate::R<PDDOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDOUTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDOUTR` writer"]
pub struct W(crate::W<PDDOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDOUTR_SPEC>;
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
impl From<crate::W<PDDOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDOUTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDOUT` reader - PDDOUT"]
pub type PDDOUT_R = crate::FieldReader;
#[doc = "Field `PDDOUT` writer - PDDOUT"]
pub type PDDOUT_W<'a, const O: u8> = crate::FieldWriter<'a, PDDOUTR_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PDDOUT"]
    #[inline(always)]
    pub fn pddout(&self) -> PDDOUT_R {
        PDDOUT_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PDDOUT"]
    #[inline(always)]
    #[must_use]
    pub fn pddout(&mut self) -> PDDOUT_W<0> {
        PDDOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDOUTR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddoutr](index.html) module"]
pub struct PDDOUTR_SPEC;
impl crate::RegisterSpec for PDDOUTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddoutr::R](R) reader structure"]
impl crate::Readable for PDDOUTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddoutr::W](W) writer structure"]
impl crate::Writable for PDDOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDOUTR to value 0"]
impl crate::Resettable for PDDOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
