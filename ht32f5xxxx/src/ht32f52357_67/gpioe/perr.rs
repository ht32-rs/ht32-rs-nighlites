#[doc = "Register `PERR` reader"]
pub struct R(crate::R<PERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERR` writer"]
pub struct W(crate::W<PERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERR_SPEC>;
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
impl From<crate::W<PERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERST0` reader - PERST0"]
pub type PERST0_R = crate::BitReader;
#[doc = "Field `PERST0` writer - PERST0"]
pub type PERST0_W<'a, const O: u8> = crate::BitWriter<'a, PERR_SPEC, O>;
#[doc = "Field `PERST1` reader - PERST1"]
pub type PERST1_R = crate::BitReader;
#[doc = "Field `PERST1` writer - PERST1"]
pub type PERST1_W<'a, const O: u8> = crate::BitWriter<'a, PERR_SPEC, O>;
#[doc = "Field `PERST2` reader - PERST2"]
pub type PERST2_R = crate::BitReader;
#[doc = "Field `PERST2` writer - PERST2"]
pub type PERST2_W<'a, const O: u8> = crate::BitWriter<'a, PERR_SPEC, O>;
#[doc = "Field `PERST3` reader - PERST3"]
pub type PERST3_R = crate::BitReader;
#[doc = "Field `PERST3` writer - PERST3"]
pub type PERST3_W<'a, const O: u8> = crate::BitWriter<'a, PERR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PERST0"]
    #[inline(always)]
    pub fn perst0(&self) -> PERST0_R {
        PERST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PERST1"]
    #[inline(always)]
    pub fn perst1(&self) -> PERST1_R {
        PERST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PERST2"]
    #[inline(always)]
    pub fn perst2(&self) -> PERST2_R {
        PERST2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PERST3"]
    #[inline(always)]
    pub fn perst3(&self) -> PERST3_R {
        PERST3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PERST0"]
    #[inline(always)]
    #[must_use]
    pub fn perst0(&mut self) -> PERST0_W<0> {
        PERST0_W::new(self)
    }
    #[doc = "Bit 1 - PERST1"]
    #[inline(always)]
    #[must_use]
    pub fn perst1(&mut self) -> PERST1_W<1> {
        PERST1_W::new(self)
    }
    #[doc = "Bit 2 - PERST2"]
    #[inline(always)]
    #[must_use]
    pub fn perst2(&mut self) -> PERST2_W<2> {
        PERST2_W::new(self)
    }
    #[doc = "Bit 3 - PERST3"]
    #[inline(always)]
    #[must_use]
    pub fn perst3(&mut self) -> PERST3_W<3> {
        PERST3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PERR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perr](index.html) module"]
pub struct PERR_SPEC;
impl crate::RegisterSpec for PERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perr::R](R) reader structure"]
impl crate::Readable for PERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perr::W](W) writer structure"]
impl crate::Writable for PERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERR to value 0"]
impl crate::Resettable for PERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
