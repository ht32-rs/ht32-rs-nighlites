#[doc = "Register `PEDRVR` reader"]
pub struct R(crate::R<PEDRVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEDRVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEDRVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEDRVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEDRVR` writer"]
pub struct W(crate::W<PEDRVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEDRVR_SPEC>;
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
impl From<crate::W<PEDRVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEDRVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEDV0` reader - PEDV0"]
pub type PEDV0_R = crate::FieldReader;
#[doc = "Field `PEDV0` writer - PEDV0"]
pub type PEDV0_W<'a, const O: u8> = crate::FieldWriter<'a, PEDRVR_SPEC, 2, O>;
#[doc = "Field `PEDV1` reader - PEDV1"]
pub type PEDV1_R = crate::FieldReader;
#[doc = "Field `PEDV1` writer - PEDV1"]
pub type PEDV1_W<'a, const O: u8> = crate::FieldWriter<'a, PEDRVR_SPEC, 2, O>;
#[doc = "Field `PEDV2` reader - PEDV2"]
pub type PEDV2_R = crate::FieldReader;
#[doc = "Field `PEDV2` writer - PEDV2"]
pub type PEDV2_W<'a, const O: u8> = crate::FieldWriter<'a, PEDRVR_SPEC, 2, O>;
#[doc = "Field `PEDV3` reader - PEDV3"]
pub type PEDV3_R = crate::FieldReader;
#[doc = "Field `PEDV3` writer - PEDV3"]
pub type PEDV3_W<'a, const O: u8> = crate::FieldWriter<'a, PEDRVR_SPEC, 2, O>;
impl R {
    #[doc = "Bits 0:1 - PEDV0"]
    #[inline(always)]
    pub fn pedv0(&self) -> PEDV0_R {
        PEDV0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - PEDV1"]
    #[inline(always)]
    pub fn pedv1(&self) -> PEDV1_R {
        PEDV1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - PEDV2"]
    #[inline(always)]
    pub fn pedv2(&self) -> PEDV2_R {
        PEDV2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - PEDV3"]
    #[inline(always)]
    pub fn pedv3(&self) -> PEDV3_R {
        PEDV3_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - PEDV0"]
    #[inline(always)]
    #[must_use]
    pub fn pedv0(&mut self) -> PEDV0_W<0> {
        PEDV0_W::new(self)
    }
    #[doc = "Bits 2:3 - PEDV1"]
    #[inline(always)]
    #[must_use]
    pub fn pedv1(&mut self) -> PEDV1_W<2> {
        PEDV1_W::new(self)
    }
    #[doc = "Bits 4:5 - PEDV2"]
    #[inline(always)]
    #[must_use]
    pub fn pedv2(&mut self) -> PEDV2_W<4> {
        PEDV2_W::new(self)
    }
    #[doc = "Bits 6:7 - PEDV3"]
    #[inline(always)]
    #[must_use]
    pub fn pedv3(&mut self) -> PEDV3_W<6> {
        PEDV3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PEDRVR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pedrvr](index.html) module"]
pub struct PEDRVR_SPEC;
impl crate::RegisterSpec for PEDRVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pedrvr::R](R) reader structure"]
impl crate::Readable for PEDRVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pedrvr::W](W) writer structure"]
impl crate::Writable for PEDRVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEDRVR to value 0"]
impl crate::Resettable for PEDRVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
