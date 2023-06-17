#[doc = "Register `PEDIRCR` reader"]
pub struct R(crate::R<PEDIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEDIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEDIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEDIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEDIRCR` writer"]
pub struct W(crate::W<PEDIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEDIRCR_SPEC>;
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
impl From<crate::W<PEDIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEDIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEDIR0` reader - PEDIR0"]
pub type PEDIR0_R = crate::BitReader;
#[doc = "Field `PEDIR0` writer - PEDIR0"]
pub type PEDIR0_W<'a, const O: u8> = crate::BitWriter<'a, PEDIRCR_SPEC, O>;
#[doc = "Field `PEDIR1` reader - PEDIR1"]
pub type PEDIR1_R = crate::BitReader;
#[doc = "Field `PEDIR1` writer - PEDIR1"]
pub type PEDIR1_W<'a, const O: u8> = crate::BitWriter<'a, PEDIRCR_SPEC, O>;
#[doc = "Field `PEDIR2` reader - PEDIR2"]
pub type PEDIR2_R = crate::BitReader;
#[doc = "Field `PEDIR2` writer - PEDIR2"]
pub type PEDIR2_W<'a, const O: u8> = crate::BitWriter<'a, PEDIRCR_SPEC, O>;
#[doc = "Field `PEDIR3` reader - PEDIR3"]
pub type PEDIR3_R = crate::BitReader;
#[doc = "Field `PEDIR3` writer - PEDIR3"]
pub type PEDIR3_W<'a, const O: u8> = crate::BitWriter<'a, PEDIRCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PEDIR0"]
    #[inline(always)]
    pub fn pedir0(&self) -> PEDIR0_R {
        PEDIR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PEDIR1"]
    #[inline(always)]
    pub fn pedir1(&self) -> PEDIR1_R {
        PEDIR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PEDIR2"]
    #[inline(always)]
    pub fn pedir2(&self) -> PEDIR2_R {
        PEDIR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEDIR3"]
    #[inline(always)]
    pub fn pedir3(&self) -> PEDIR3_R {
        PEDIR3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PEDIR0"]
    #[inline(always)]
    #[must_use]
    pub fn pedir0(&mut self) -> PEDIR0_W<0> {
        PEDIR0_W::new(self)
    }
    #[doc = "Bit 1 - PEDIR1"]
    #[inline(always)]
    #[must_use]
    pub fn pedir1(&mut self) -> PEDIR1_W<1> {
        PEDIR1_W::new(self)
    }
    #[doc = "Bit 2 - PEDIR2"]
    #[inline(always)]
    #[must_use]
    pub fn pedir2(&mut self) -> PEDIR2_W<2> {
        PEDIR2_W::new(self)
    }
    #[doc = "Bit 3 - PEDIR3"]
    #[inline(always)]
    #[must_use]
    pub fn pedir3(&mut self) -> PEDIR3_W<3> {
        PEDIR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PEDIRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pedircr](index.html) module"]
pub struct PEDIRCR_SPEC;
impl crate::RegisterSpec for PEDIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pedircr::R](R) reader structure"]
impl crate::Readable for PEDIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pedircr::W](W) writer structure"]
impl crate::Writable for PEDIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PEDIRCR to value 0"]
impl crate::Resettable for PEDIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
