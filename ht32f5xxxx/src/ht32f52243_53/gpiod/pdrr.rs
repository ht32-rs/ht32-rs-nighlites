#[doc = "Register `PDRR` reader"]
pub struct R(crate::R<PDRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDRR` writer"]
pub struct W(crate::W<PDRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDRR_SPEC>;
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
impl From<crate::W<PDRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDRST0` reader - PDRST0"]
pub type PDRST0_R = crate::BitReader;
#[doc = "Field `PDRST0` writer - PDRST0"]
pub type PDRST0_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST1` reader - PDRST1"]
pub type PDRST1_R = crate::BitReader;
#[doc = "Field `PDRST1` writer - PDRST1"]
pub type PDRST1_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST2` reader - PDRST2"]
pub type PDRST2_R = crate::BitReader;
#[doc = "Field `PDRST2` writer - PDRST2"]
pub type PDRST2_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
#[doc = "Field `PDRST3` reader - PDRST3"]
pub type PDRST3_R = crate::BitReader;
#[doc = "Field `PDRST3` writer - PDRST3"]
pub type PDRST3_W<'a, const O: u8> = crate::BitWriter<'a, PDRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PDRST0"]
    #[inline(always)]
    pub fn pdrst0(&self) -> PDRST0_R {
        PDRST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDRST1"]
    #[inline(always)]
    pub fn pdrst1(&self) -> PDRST1_R {
        PDRST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDRST2"]
    #[inline(always)]
    pub fn pdrst2(&self) -> PDRST2_R {
        PDRST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDRST3"]
    #[inline(always)]
    pub fn pdrst3(&self) -> PDRST3_R {
        PDRST3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDRST0"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst0(&mut self) -> PDRST0_W<0> {
        PDRST0_W::new(self)
    }
    #[doc = "Bit 1 - PDRST1"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst1(&mut self) -> PDRST1_W<1> {
        PDRST1_W::new(self)
    }
    #[doc = "Bit 2 - PDRST2"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst2(&mut self) -> PDRST2_W<2> {
        PDRST2_W::new(self)
    }
    #[doc = "Bit 3 - PDRST3"]
    #[inline(always)]
    #[must_use]
    pub fn pdrst3(&mut self) -> PDRST3_W<3> {
        PDRST3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdrr](index.html) module"]
pub struct PDRR_SPEC;
impl crate::RegisterSpec for PDRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdrr::R](R) reader structure"]
impl crate::Readable for PDRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdrr::W](W) writer structure"]
impl crate::Writable for PDRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDRR to value 0"]
impl crate::Resettable for PDRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
