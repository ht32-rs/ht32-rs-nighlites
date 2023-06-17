#[doc = "Register `PESRR` reader"]
pub struct R(crate::R<PESRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PESRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PESRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PESRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PESRR` writer"]
pub struct W(crate::W<PESRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PESRR_SPEC>;
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
impl From<crate::W<PESRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PESRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PESET0` reader - PESET0"]
pub type PESET0_R = crate::BitReader;
#[doc = "Field `PESET0` writer - PESET0"]
pub type PESET0_W<'a, const O: u8> = crate::BitWriter<'a, PESRR_SPEC, O>;
#[doc = "Field `PESET1` reader - PESET1"]
pub type PESET1_R = crate::BitReader;
#[doc = "Field `PESET1` writer - PESET1"]
pub type PESET1_W<'a, const O: u8> = crate::BitWriter<'a, PESRR_SPEC, O>;
#[doc = "Field `PESET2` reader - PESET2"]
pub type PESET2_R = crate::BitReader;
#[doc = "Field `PESET2` writer - PESET2"]
pub type PESET2_W<'a, const O: u8> = crate::BitWriter<'a, PESRR_SPEC, O>;
#[doc = "Field `PESET3` reader - PESET3"]
pub type PESET3_R = crate::BitReader;
#[doc = "Field `PESET3` writer - PESET3"]
pub type PESET3_W<'a, const O: u8> = crate::BitWriter<'a, PESRR_SPEC, O>;
#[doc = "Field `PERST0` reader - PERST0"]
pub type PERST0_R = crate::BitReader;
#[doc = "Field `PERST0` writer - PERST0"]
pub type PERST0_W<'a, const O: u8> = crate::BitWriter<'a, PESRR_SPEC, O>;
#[doc = "Field `PERST1` reader - PERST1"]
pub type PERST1_R = crate::BitReader;
#[doc = "Field `PERST1` writer - PERST1"]
pub type PERST1_W<'a, const O: u8> = crate::BitWriter<'a, PESRR_SPEC, O>;
#[doc = "Field `PERST2` reader - PERST2"]
pub type PERST2_R = crate::BitReader;
#[doc = "Field `PERST2` writer - PERST2"]
pub type PERST2_W<'a, const O: u8> = crate::BitWriter<'a, PESRR_SPEC, O>;
#[doc = "Field `PERST3` reader - PERST3"]
pub type PERST3_R = crate::BitReader;
#[doc = "Field `PERST3` writer - PERST3"]
pub type PERST3_W<'a, const O: u8> = crate::BitWriter<'a, PESRR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PESET0"]
    #[inline(always)]
    pub fn peset0(&self) -> PESET0_R {
        PESET0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PESET1"]
    #[inline(always)]
    pub fn peset1(&self) -> PESET1_R {
        PESET1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PESET2"]
    #[inline(always)]
    pub fn peset2(&self) -> PESET2_R {
        PESET2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PESET3"]
    #[inline(always)]
    pub fn peset3(&self) -> PESET3_R {
        PESET3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - PERST0"]
    #[inline(always)]
    pub fn perst0(&self) -> PERST0_R {
        PERST0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PERST1"]
    #[inline(always)]
    pub fn perst1(&self) -> PERST1_R {
        PERST1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PERST2"]
    #[inline(always)]
    pub fn perst2(&self) -> PERST2_R {
        PERST2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PERST3"]
    #[inline(always)]
    pub fn perst3(&self) -> PERST3_R {
        PERST3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PESET0"]
    #[inline(always)]
    #[must_use]
    pub fn peset0(&mut self) -> PESET0_W<0> {
        PESET0_W::new(self)
    }
    #[doc = "Bit 1 - PESET1"]
    #[inline(always)]
    #[must_use]
    pub fn peset1(&mut self) -> PESET1_W<1> {
        PESET1_W::new(self)
    }
    #[doc = "Bit 2 - PESET2"]
    #[inline(always)]
    #[must_use]
    pub fn peset2(&mut self) -> PESET2_W<2> {
        PESET2_W::new(self)
    }
    #[doc = "Bit 3 - PESET3"]
    #[inline(always)]
    #[must_use]
    pub fn peset3(&mut self) -> PESET3_W<3> {
        PESET3_W::new(self)
    }
    #[doc = "Bit 16 - PERST0"]
    #[inline(always)]
    #[must_use]
    pub fn perst0(&mut self) -> PERST0_W<16> {
        PERST0_W::new(self)
    }
    #[doc = "Bit 17 - PERST1"]
    #[inline(always)]
    #[must_use]
    pub fn perst1(&mut self) -> PERST1_W<17> {
        PERST1_W::new(self)
    }
    #[doc = "Bit 18 - PERST2"]
    #[inline(always)]
    #[must_use]
    pub fn perst2(&mut self) -> PERST2_W<18> {
        PERST2_W::new(self)
    }
    #[doc = "Bit 19 - PERST3"]
    #[inline(always)]
    #[must_use]
    pub fn perst3(&mut self) -> PERST3_W<19> {
        PERST3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PESRR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pesrr](index.html) module"]
pub struct PESRR_SPEC;
impl crate::RegisterSpec for PESRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pesrr::R](R) reader structure"]
impl crate::Readable for PESRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pesrr::W](W) writer structure"]
impl crate::Writable for PESRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PESRR to value 0"]
impl crate::Resettable for PESRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
