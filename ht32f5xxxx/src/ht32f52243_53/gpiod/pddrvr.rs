#[doc = "Register `PDDRVR` reader"]
pub struct R(crate::R<PDDRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDRVR` writer"]
pub struct W(crate::W<PDDRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDRVR_SPEC>;
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
impl From<crate::W<PDDRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDDV0` reader - PDDV0"]
pub type PDDV0_R = crate::FieldReader;
#[doc = "Field `PDDV0` writer - PDDV0"]
pub type PDDV0_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV1` reader - PDDV1"]
pub type PDDV1_R = crate::FieldReader;
#[doc = "Field `PDDV1` writer - PDDV1"]
pub type PDDV1_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV2` reader - PDDV2"]
pub type PDDV2_R = crate::FieldReader;
#[doc = "Field `PDDV2` writer - PDDV2"]
pub type PDDV2_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
#[doc = "Field `PDDV3` reader - PDDV3"]
pub type PDDV3_R = crate::FieldReader;
#[doc = "Field `PDDV3` writer - PDDV3"]
pub type PDDV3_W<'a, const O: u8> = crate::FieldWriter<'a, PDDRVR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PDDV0"]
    #[inline(always)]
    pub fn pddv0(&self) -> PDDV0_R {
        PDDV0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PDDV1"]
    #[inline(always)]
    pub fn pddv1(&self) -> PDDV1_R {
        PDDV1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PDDV2"]
    #[inline(always)]
    pub fn pddv2(&self) -> PDDV2_R {
        PDDV2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PDDV3"]
    #[inline(always)]
    pub fn pddv3(&self) -> PDDV3_R {
        PDDV3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PDDV0"]
    #[inline(always)]
    #[must_use]
    pub fn pddv0(&mut self) -> PDDV0_W<0> {
        PDDV0_W::new(self)
    }
    #[doc = "Bits 2:3 - PDDV1"]
    #[inline(always)]
    #[must_use]
    pub fn pddv1(&mut self) -> PDDV1_W<2> {
        PDDV1_W::new(self)
    }
    #[doc = "Bits 4:5 - PDDV2"]
    #[inline(always)]
    #[must_use]
    pub fn pddv2(&mut self) -> PDDV2_W<4> {
        PDDV2_W::new(self)
    }
    #[doc = "Bits 6:7 - PDDV3"]
    #[inline(always)]
    #[must_use]
    pub fn pddv3(&mut self) -> PDDV3_W<6> {
        PDDV3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDDRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddrvr](index.html) module"]
pub struct PDDRVR_SPEC;
impl crate::RegisterSpec for PDDRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pddrvr::R](R) reader structure"]
impl crate::Readable for PDDRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddrvr::W](W) writer structure"]
impl crate::Writable for PDDRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDDRVR to value 0"]
impl crate::Resettable for PDDRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
